#!/bin/bash

# DeepSeek Configuration Script for Goose
# This script sets up the environment variables for using DeepSeek with Goose

echo "🚀 Setting up DeepSeek provider for Goose..."

# Core DeepSeek configuration
export DEEPSEEK_API_KEY="sk-07383265419747c895e73f27c807b4cb"
export GOOSE_PROVIDER="deepseek"
export GOOSE_MODEL="deepseek-chat"

# Lead/Worker model configuration
export GOOSE_LEAD_MODEL="deepseek-chat"        # DeepSeek-V3-0324 for lead tasks
export GOOSE_LEAD_PROVIDER="deepseek"

# Planner model configuration
export GOOSE_PLANNER_PROVIDER="deepseek"
export GOOSE_PLANNER_MODEL="deepseek-reasoner"  # DeepSeek-R1-0528 for planning

# Optional: DeepSeek host configuration (default values)
export DEEPSEEK_HOST="https://api.deepseek.com"
export DEEPSEEK_BASE_PATH="v1/chat/completions"
export DEEPSEEK_TIMEOUT="600"

echo "✅ DeepSeek configuration completed!"
echo ""
echo "🎯 Available models:"
echo "   • deepseek-chat (DeepSeek-V3-0324) - Lead Model"
echo "   • deepseek-reasoner (DeepSeek-R1-0528) - Planner Model"
echo ""
echo "📝 Usage examples:"
echo "   • Interactive session: ./target/debug/goose session"
echo "   • Run with text: ./target/debug/goose run --text 'Your prompt here'"
echo "   • Use planner: ./target/debug/goose run --text '/plan Your planning task'"
echo ""
echo "🔧 Environment variables set:"
echo "   DEEPSEEK_API_KEY=sk-07...4cb"
echo "   GOOSE_PROVIDER=deepseek"
echo "   GOOSE_MODEL=deepseek-chat"
echo "   GOOSE_LEAD_MODEL=deepseek-chat"
echo "   GOOSE_LEAD_PROVIDER=deepseek"
echo "   GOOSE_PLANNER_PROVIDER=deepseek"
echo "   GOOSE_PLANNER_MODEL=deepseek-reasoner"
echo ""
echo "To use these settings in your shell, run: source setup_deepseek.sh"
echo ""
echo "📚 Documentation:"
echo "   • Complete guide: DEEPSEEK_INTEGRATION.md"
echo "   • Provider docs: documentation/docs/getting-started/providers.md"
echo "   • Changelog: CHANGELOG.md" 