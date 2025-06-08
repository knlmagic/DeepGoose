#!/bin/bash

# Goose launcher script for Cursor integration
# This script ensures DeepSeek environment is loaded before starting Goose

echo "🦢 Starting Goose with DeepSeek integration..."

# Load DeepSeek configuration
export DEEPSEEK_API_KEY="sk-07383265419747c895e73f27c807b4cb"
export GOOSE_PROVIDER="deepseek"
export GOOSE_MODEL="deepseek-chat"
export GOOSE_LEAD_MODEL="deepseek-chat"
export GOOSE_LEAD_PROVIDER="deepseek"
export GOOSE_PLANNER_PROVIDER="deepseek"
export GOOSE_PLANNER_MODEL="deepseek-reasoner"
export DEEPSEEK_HOST="https://api.deepseek.com"
export DEEPSEEK_BASE_PATH="v1/chat/completions"
export DEEPSEEK_TIMEOUT="600"

# Start Goose with any arguments passed to this script
goose "$@" 