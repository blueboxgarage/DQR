#!/bin/bash
# DQR Environment Setup Script
# Run this script with: source setup-dqr.sh

# Default configuration
export DQR_HOST=127.0.0.1
export DQR_PORT=8081
export DQR_RULES_PATH=rules/default.csv
export RUST_LOG=info

# Uncomment and modify these lines to customize your configuration
# export DQR_HOST=0.0.0.0        # Listen on all interfaces
# export DQR_PORT=9000           # Use custom port
# export DQR_RULES_PATH=rules/examples/conditionals.csv  # Use a different rule set
# export RUST_LOG=debug          # More detailed logging

echo "DQR environment configured:"
echo "  Host:      $DQR_HOST"
echo "  Port:      $DQR_PORT"
echo "  Rules:     $DQR_RULES_PATH"
echo "  Log Level: $RUST_LOG"