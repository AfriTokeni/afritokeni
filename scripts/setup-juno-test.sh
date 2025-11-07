#!/bin/bash

# Setup Juno test satellite for local integration testing
# Based on: https://juno.build/docs/guides/e2e#continuous-integration

set -e

echo "🚀 Setting up Juno test satellite for integration testing..."

# Juno emulator uses development satellite ID
SATELLITE_ID="atbka-rp777-77775-aaaaq-cai"

echo "📦 Starting Juno emulator..."

# Start Juno emulator in headless mode
juno emulator start --headless &

echo "⏳ Waiting for emulator to be ready..."
juno emulator wait

echo "🔐 Logging into emulator..."
juno login --mode development --emulator --headless

echo "⚙️ Applying Juno configuration (deploying satellite with collections)..."
# Emulator always uses development mode
juno config apply --mode development --headless

echo ""
echo "✅ Juno test satellite fully configured!"
echo "📍 Satellite ID: $SATELLITE_ID"
echo "📦 Collections deployed from juno.config.ts"
echo ""
echo "Next steps:"
echo "1. Build USSD canister: cargo build --package ussd_canister --target wasm32-unknown-unknown --release"
echo "2. Deploy USSD canister: dfx deploy ussd_canister --yes"
echo "3. Run tests: pnpm test:integration"
echo ""
echo "To stop the emulator: juno emulator stop"
