#!/bin/bash

# ============================================================================
# AfriTokeni Canister Deployment Script
# Similar to Hardhat Ignition for Ethereum
# ============================================================================

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
NETWORK="${1:-playground}"  # Default to playground, can be: local, playground, ic
ADMIN_PRINCIPAL="${2:-$(dfx identity get-principal)}"

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║         AfriTokeni Canister Deployment Script             ║${NC}"
echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}Network: ${NETWORK}${NC}"
echo -e "${GREEN}Admin Principal: ${ADMIN_PRINCIPAL}${NC}"
echo ""

# ============================================================================
# Step 1: Build all canisters
# ============================================================================
echo -e "${YELLOW}📦 Step 1: Building all canisters...${NC}"
dfx build --network ${NETWORK}
echo -e "${GREEN}✅ Build complete${NC}\n"

# ============================================================================
# Step 2: Deploy Data Canister (Storage Layer)
# ============================================================================
echo -e "${YELLOW}🚀 Step 2: Deploying Data Canister (Storage Layer)...${NC}"
DATA_CANISTER_ID=$(dfx deploy data_canister --network ${NETWORK} --argument '(null, null)' 2>&1 | grep -oP 'Canister ID: \K[a-z0-9-]+' || dfx canister id data_canister --network ${NETWORK})
echo -e "${GREEN}✅ Data Canister deployed: ${DATA_CANISTER_ID}${NC}\n"

# ============================================================================
# Step 3: Deploy Domain Canisters (Business Logic Layer)
# ============================================================================
echo -e "${YELLOW}🚀 Step 3: Deploying Domain Canisters...${NC}"

# User Canister
echo -e "${BLUE}  → Deploying User Canister...${NC}"
USER_CANISTER_ID=$(dfx deploy user_canister --network ${NETWORK} 2>&1 | grep -oP 'Canister ID: \K[a-z0-9-]+' || dfx canister id user_canister --network ${NETWORK})
echo -e "${GREEN}  ✅ User Canister: ${USER_CANISTER_ID}${NC}"

# Wallet Canister
echo -e "${BLUE}  → Deploying Wallet Canister...${NC}"
WALLET_CANISTER_ID=$(dfx deploy wallet_canister --network ${NETWORK} 2>&1 | grep -oP 'Canister ID: \K[a-z0-9-]+' || dfx canister id wallet_canister --network ${NETWORK})
echo -e "${GREEN}  ✅ Wallet Canister: ${WALLET_CANISTER_ID}${NC}"

# Crypto Canister
echo -e "${BLUE}  → Deploying Crypto Canister...${NC}"
CRYPTO_CANISTER_ID=$(dfx deploy crypto_canister --network ${NETWORK} 2>&1 | grep -oP 'Canister ID: \K[a-z0-9-]+' || dfx canister id crypto_canister --network ${NETWORK})
echo -e "${GREEN}  ✅ Crypto Canister: ${CRYPTO_CANISTER_ID}${NC}"

# Agent Canister
echo -e "${BLUE}  → Deploying Agent Canister...${NC}"
AGENT_CANISTER_ID=$(dfx deploy agent_canister --network ${NETWORK} 2>&1 | grep -oP 'Canister ID: \K[a-z0-9-]+' || dfx canister id agent_canister --network ${NETWORK})
echo -e "${GREEN}  ✅ Agent Canister: ${AGENT_CANISTER_ID}${NC}"

echo ""

# ============================================================================
# Step 4: Deploy USSD Canister (Presentation Layer)
# ============================================================================
echo -e "${YELLOW}🚀 Step 4: Deploying USSD Canister (Presentation Layer)...${NC}"
USSD_CANISTER_ID=$(dfx deploy ussd_canister --network ${NETWORK} 2>&1 | grep -oP 'Canister ID: \K[a-z0-9-]+' || dfx canister id ussd_canister --network ${NETWORK})
echo -e "${GREEN}✅ USSD Canister deployed: ${USSD_CANISTER_ID}${NC}\n"

# ============================================================================
# Step 5: Save deployment info
# ============================================================================
echo -e "${YELLOW}💾 Step 5: Saving deployment information...${NC}"

DEPLOYMENT_FILE="deployments/${NETWORK}_deployment.json"
mkdir -p deployments

cat > ${DEPLOYMENT_FILE} << EOF
{
  "network": "${NETWORK}",
  "deployed_at": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "admin_principal": "${ADMIN_PRINCIPAL}",
  "canisters": {
    "data_canister": "${DATA_CANISTER_ID}",
    "user_canister": "${USER_CANISTER_ID}",
    "wallet_canister": "${WALLET_CANISTER_ID}",
    "crypto_canister": "${CRYPTO_CANISTER_ID}",
    "agent_canister": "${AGENT_CANISTER_ID}",
    "ussd_canister": "${USSD_CANISTER_ID}"
  }
}
EOF

echo -e "${GREEN}✅ Deployment info saved to ${DEPLOYMENT_FILE}${NC}\n"

# ============================================================================
# Step 6: Setup Authorization (call separate script)
# ============================================================================
echo -e "${YELLOW}🔐 Step 6: Setting up authorization...${NC}"
bash scripts/setup-auth.sh ${NETWORK}
echo ""

# ============================================================================
# Deployment Summary
# ============================================================================
echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║              Deployment Summary                            ║${NC}"
echo -e "${BLUE}╠════════════════════════════════════════════════════════════╣${NC}"
echo -e "${GREEN}║ Network:          ${NETWORK}${NC}"
echo -e "${GREEN}║ Admin:            ${ADMIN_PRINCIPAL}${NC}"
echo -e "${BLUE}╠════════════════════════════════════════════════════════════╣${NC}"
echo -e "${GREEN}║ Data Canister:    ${DATA_CANISTER_ID}${NC}"
echo -e "${GREEN}║ User Canister:    ${USER_CANISTER_ID}${NC}"
echo -e "${GREEN}║ Wallet Canister:  ${WALLET_CANISTER_ID}${NC}"
echo -e "${GREEN}║ Crypto Canister:  ${CRYPTO_CANISTER_ID}${NC}"
echo -e "${GREEN}║ Agent Canister:   ${AGENT_CANISTER_ID}${NC}"
echo -e "${GREEN}║ USSD Canister:    ${USSD_CANISTER_ID}${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}🎉 Deployment complete!${NC}"
echo -e "${YELLOW}📝 Next steps:${NC}"
echo -e "   1. Run: ${BLUE}bash scripts/admin.sh add-admin ${NETWORK} <principal>${NC}"
echo -e "   2. Test: ${BLUE}bash scripts/test-deployment.sh ${NETWORK}${NC}"
echo ""
