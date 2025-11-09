#!/bin/bash
# Post-deployment script to configure canister dependencies

set -e

NETWORK="${DFX_NETWORK:-local}"

echo "🔧 Configuring canister dependencies on network: $NETWORK"

# Get canister IDs
BUSINESS_LOGIC_ID=$(dfx canister id business_logic_canister --network $NETWORK)
DATA_CANISTER_ID=$(dfx canister id data_canister --network $NETWORK)

echo "📋 Canister IDs:"
echo "  Business Logic: $BUSINESS_LOGIC_ID"
echo "  Data Canister: $DATA_CANISTER_ID"

# Configure USSD Canister
echo "⚙️  Configuring USSD Canister..."
dfx canister call ussd_canister set_business_logic_canister_id "(\"$BUSINESS_LOGIC_ID\")" --network $NETWORK

# Configure Business Logic Canister
echo "⚙️  Configuring Business Logic Canister..."
dfx canister call business_logic_canister set_data_canister_id "(\"$DATA_CANISTER_ID\")" --network $NETWORK

echo "✅ Configuration complete!"
