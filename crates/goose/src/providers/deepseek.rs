use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

use super::base::{ConfigKey, Provider, ProviderMetadata, ProviderUsage, Usage};
use super::errors::ProviderError;
use super::formats::openai::{create_request, get_usage, response_to_message};
use super::utils::{emit_debug_trace, get_model, ImageFormat};
use crate::message::Message;
use crate::model::ModelConfig;
use mcp_core::tool::Tool;

pub const DEEPSEEK_DEFAULT_MODEL: &str = "deepseek-chat";
pub const DEEPSEEK_KNOWN_MODELS: &[&str] = &[
    "deepseek-chat",      // DeepSeek-V3-0324 - Lead Model
    "deepseek-reasoner",  // DeepSeek-R1-0528 - Planner Model
];

pub const DEEPSEEK_DOC_URL: &str = "https://platform.deepseek.com/api-docs";

#[derive(Debug, serde::Serialize)]
pub struct DeepSeekProvider {
    #[serde(skip)]
    client: Client,
    host: String,
    base_path: String,
    api_key: String,
    model: ModelConfig,
    custom_headers: Option<HashMap<String, String>>,
}

impl Default for DeepSeekProvider {
    fn default() -> Self {
        let model = ModelConfig::new(DeepSeekProvider::metadata().default_model);
        DeepSeekProvider::from_env(model).expect("Failed to initialize DeepSeek provider")
    }
}

impl DeepSeekProvider {
    pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let api_key: String = config.get_secret("DEEPSEEK_API_KEY")?;
        let host: String = config
            .get_param("DEEPSEEK_HOST")
            .unwrap_or_else(|_| "https://api.deepseek.com".to_string());
        let base_path: String = config
            .get_param("DEEPSEEK_BASE_PATH")
            .unwrap_or_else(|_| "v1/chat/completions".to_string());
        let custom_headers: Option<HashMap<String, String>> = config
            .get_secret("DEEPSEEK_CUSTOM_HEADERS")
            .or_else(|_| config.get_param("DEEPSEEK_CUSTOM_HEADERS"))
            .ok()
            .map(parse_custom_headers);
        let timeout_secs: u64 = config.get_param("DEEPSEEK_TIMEOUT").unwrap_or(600);
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()?;

        Ok(Self {
            client,
            host,
            base_path,
            api_key,
            model,
            custom_headers,
        })
    }

    /// Helper function to add DeepSeek-specific headers to a request
    fn add_headers(&self, mut request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        // Add custom headers if present
        if let Some(custom_headers) = &self.custom_headers {
            for (key, value) in custom_headers {
                request = request.header(key, value);
            }
        }

        request
    }

    async fn post(&self, mut payload: Value) -> Result<Value, ProviderError> {
        // Enable streaming for DeepSeek
        payload
            .as_object_mut()
            .unwrap()
            .insert("stream".to_string(), serde_json::Value::Bool(true));
            
        let base_url = url::Url::parse(&self.host)
            .map_err(|e| ProviderError::RequestFailed(format!("Invalid base URL: {e}")))?;
        let url = base_url.join(&self.base_path).map_err(|e| {
            ProviderError::RequestFailed(format!("Failed to construct endpoint URL: {e}"))
        })?;

        let request = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key));

        let request = self.add_headers(request);

        let response = request.json(&payload).send().await?;

        // Handle streaming response
        use super::utils_universal_openai_stream::{OAIStreamChunk, OAIStreamCollector};
        use futures_util::StreamExt;
        
        let mut collector = OAIStreamCollector::new();
        let mut stream = response.bytes_stream();
        
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| ProviderError::RequestFailed(e.to_string()))?;
            let text = String::from_utf8_lossy(&chunk);
            
            for line in text.lines() {
                let tline = line.trim();
                if !tline.starts_with("data: ") {
                    continue;
                }
                let payload = &tline[6..];
                if payload == "[DONE]" {
                    break;
                }
                match serde_json::from_str::<OAIStreamChunk>(payload) {
                    Ok(ch) => collector.add_chunk(&ch),
                    Err(_) => continue,
                }
            }
        }
        
        let final_response = collector.build_response();
        let response_value = serde_json::to_value(final_response)
            .map_err(|e| ProviderError::RequestFailed(e.to_string()))?;
            
        Ok(response_value)
    }
}

#[async_trait]
impl Provider for DeepSeekProvider {
    fn metadata() -> ProviderMetadata {
        ProviderMetadata::new(
            "deepseek",
            "DeepSeek",
            "DeepSeek V3 and R1 models with advanced reasoning capabilities",
            DEEPSEEK_DEFAULT_MODEL,
            DEEPSEEK_KNOWN_MODELS.to_vec(),
            DEEPSEEK_DOC_URL,
            vec![
                ConfigKey::new("DEEPSEEK_API_KEY", true, true, None),
                ConfigKey::new("DEEPSEEK_HOST", true, false, Some("https://api.deepseek.com")),
                ConfigKey::new("DEEPSEEK_BASE_PATH", true, false, Some("v1/chat/completions")),
                ConfigKey::new("DEEPSEEK_CUSTOM_HEADERS", false, true, None),
                ConfigKey::new("DEEPSEEK_TIMEOUT", false, false, Some("600")),
            ],
        )
    }

    fn get_model_config(&self) -> ModelConfig {
        self.model.clone()
    }

    #[tracing::instrument(
        skip(self, system, messages, tools),
        fields(model_config, input, output, input_tokens, output_tokens, total_tokens)
    )]
    async fn complete(
        &self,
        system: &str,
        messages: &[Message],
        tools: &[Tool],
    ) -> Result<(Message, ProviderUsage), ProviderError> {
        let payload = create_request(&self.model, system, messages, tools, &ImageFormat::OpenAi)?;

        // Make request
        let response = self.post(payload.clone()).await?;

        // Parse response
        let message = response_to_message(response.clone())?;
        let usage = match get_usage(&response) {
            Ok(usage) => usage,
            Err(ProviderError::UsageError(e)) => {
                tracing::debug!("Failed to get usage data: {}", e);
                Usage::default()
            }
            Err(e) => return Err(e),
        };
        let model = get_model(&response);
        emit_debug_trace(&self.model, &payload, &response, &usage);
        Ok((message, ProviderUsage::new(model, usage)))
    }

    /// Fetch supported models from DeepSeek; returns Err on any failure, Ok(None) if no data
    async fn fetch_supported_models_async(&self) -> Result<Option<Vec<String>>, ProviderError> {
        // List available models via DeepSeek API
        let base_url =
            url::Url::parse(&self.host).map_err(|e| ProviderError::RequestFailed(e.to_string()))?;
        let url = base_url
            .join("v1/models")
            .map_err(|e| ProviderError::RequestFailed(e.to_string()))?;
        let mut request = self.client.get(url).bearer_auth(&self.api_key);
        
        if let Some(headers) = &self.custom_headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }
        
        let response = request.send().await?;
        let json: serde_json::Value = response.json().await?;
        
        if let Some(err_obj) = json.get("error") {
            let msg = err_obj
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown error");
            return Err(ProviderError::Authentication(msg.to_string()));
        }
        
        let data = json.get("data").and_then(|v| v.as_array()).ok_or_else(|| {
            ProviderError::UsageError("Missing data field in JSON response".into())
        })?;
        
        let mut models: Vec<String> = data
            .iter()
            .filter_map(|m| m.get("id").and_then(|v| v.as_str()).map(str::to_string))
            .collect();
        models.sort();
        Ok(Some(models))
    }

    fn supports_embeddings(&self) -> bool {
        false // DeepSeek doesn't currently support embeddings
    }

    async fn create_embeddings(&self, _texts: Vec<String>) -> Result<Vec<Vec<f32>>, ProviderError> {
        Err(ProviderError::ExecutionError(
            "DeepSeek does not support embeddings".to_string(),
        ))
    }
}

fn parse_custom_headers(s: String) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    for pair in s.split(',') {
        if let Some((key, value)) = pair.split_once('=') {
            headers.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    headers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_custom_headers() {
        let headers_str = "X-Custom-Header=value1,X-Another-Header=value2".to_string();
        let headers = parse_custom_headers(headers_str);
        
        assert_eq!(headers.get("X-Custom-Header"), Some(&"value1".to_string()));
        assert_eq!(headers.get("X-Another-Header"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_metadata() {
        let metadata = DeepSeekProvider::metadata();
        assert_eq!(metadata.id, "deepseek");
        assert_eq!(metadata.display_name, "DeepSeek");
        assert_eq!(metadata.default_model, "deepseek-chat");
        assert!(metadata.models.contains(&"deepseek-chat".to_string()));
        assert!(metadata.models.contains(&"deepseek-reasoner".to_string()));
    }
} 