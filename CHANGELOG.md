# Changelog

All notable changes to DeepGoose will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **DeepSeek Provider Integration**: Complete integration of DeepSeek AI models with OpenAI-compatible API
  - Support for `deepseek-chat` (DeepSeek-V3-0324) as lead model for complex reasoning and code generation
  - Support for `deepseek-reasoner` (DeepSeek-R1-0528) as planner model for strategic thinking
  - Full streaming support for real-time response generation
  - Comprehensive error handling and timeout configuration
  - Environment variable configuration with sensible defaults

- **Multi-Model Architecture**: Enhanced support for specialized model roles
  - Lead/Worker model pattern with DeepSeek-V3 for complex tasks
  - Planning model configuration with DeepSeek-R1 for `/plan` command
  - Seamless model switching based on task requirements

- **UI Integration**: Complete desktop application support
  - DeepSeek provider in settings/provider registry
  - Configuration interface for API keys and endpoints
  - Model selection and parameter configuration

- **Documentation**: Comprehensive setup and usage documentation
  - `DEEPSEEK_INTEGRATION.md` with complete implementation details
  - Updated `README.md` with quick start guide
  - Enhanced provider documentation with DeepSeek configuration
  - Setup scripts for easy environment configuration

- **Development Tools**: Enhanced development experience
  - `setup_deepseek.sh` script for one-command environment setup
  - `goose-cursor.sh` launcher script for Cursor IDE integration
  - Comprehensive testing and validation procedures

### Technical Implementation
- **Provider Architecture**: 
  - OpenAI-compatible API implementation in `crates/goose/src/providers/deepseek.rs`
  - Provider factory registration in `crates/goose/src/providers/factory.rs`
  - Module exports in `crates/goose/src/providers/mod.rs`

- **Server Integration**:
  - Provider metadata in `crates/goose-server/src/routes/providers_and_keys.json`
  - UI component integration in `ui/desktop/src/components/settings/providers/ProviderRegistry.tsx`

- **Configuration Management**:
  - Environment variable support: `DEEPSEEK_API_KEY`, `DEEPSEEK_HOST`, `DEEPSEEK_BASE_PATH`, `DEEPSEEK_TIMEOUT`
  - Goose configuration variables: `GOOSE_PROVIDER`, `GOOSE_LEAD_MODEL`, `GOOSE_PLANNER_MODEL`
  - Secure API key handling and validation

### Performance & Reliability
- **Streaming Support**: Real-time response streaming for improved user experience
- **Error Handling**: Comprehensive error handling with detailed error messages
- **Timeout Management**: Configurable request timeouts (default: 600 seconds)
- **API Compatibility**: Full OpenAI API compatibility for seamless integration

### Developer Experience
- **Easy Setup**: One-command setup with `source setup_deepseek.sh`
- **IDE Integration**: Cursor IDE launcher script for seamless development workflow
- **Comprehensive Testing**: Validated integration with both lead and planner models
- **Documentation**: Complete setup, configuration, and usage documentation

## [1.0.24] - Base Goose Version
- Base Goose functionality from upstream repository
- Multi-provider support framework
- Extension system and MCP integration
- Desktop and CLI interfaces 