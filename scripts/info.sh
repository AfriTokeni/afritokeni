#!/bin/bash

# ============================================================================
# AfriTokeni Deployment Info Script
# Quick view of deployment information
# ============================================================================

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

NETWORK="${1:-playground}"

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║         AfriTokeni Deployment Information                 ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if deployment exists
DEPLOYMENT_FILE="deployments/${NETWORK}_deployment.json"
AUTH_FILE="deployments/${NETWORK}_authorization.json"

if [ ! -f "${DEPLOYMENT_FILE}" ]; then
    echo -e "${RED}❌ No deployment found for network: ${NETWORK}${NC}"
    echo -e "${YELLOW}Run: bash scripts/deploy.sh ${NETWORK}${NC}"
    exit 1
fi

# ============================================================================
# Display Deployment Info
# ============================================================================

echo -e "${CYAN}📦 Deployment Details${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

DEPLOYED_AT=$(cat ${DEPLOYMENT_FILE} | grep "deployed_at" | cut -d'"' -f4)
ADMIN=$(cat ${DEPLOYMENT_FILE} | grep "admin_principal" | cut -d'"' -f4)

echo -e "${GREEN}Network:${NC}      ${NETWORK}"
echo -e "${GREEN}Deployed:${NC}     ${DEPLOYED_AT}"
echo -e "${GREEN}Admin:${NC}        ${ADMIN}"
echo ""

# ============================================================================
# Display Canister IDs
# ============================================================================

echo -e "${CYAN}🔗 Canister IDs${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

DATA_CANISTER=$(cat ${DEPLOYMENT_FILE} | grep "data_canister" | cut -d'"' -f4)
USER_CANISTER=$(cat ${DEPLOYMENT_FILE} | grep "user_canister" | cut -d'"' -f4)
WALLET_CANISTER=$(cat ${DEPLOYMENT_FILE} | grep "wallet_canister" | cut -d'"' -f4)
CRYPTO_CANISTER=$(cat ${DEPLOYMENT_FILE} | grep "crypto_canister" | cut -d'"' -f4)
AGENT_CANISTER=$(cat ${DEPLOYMENT_FILE} | grep "agent_canister" | cut -d'"' -f4)
USSD_CANISTER=$(cat ${DEPLOYMENT_FILE} | grep "ussd_canister" | cut -d'"' -f4)

echo -e "${YELLOW}Storage Layer:${NC}"
echo -e "  ${GREEN}Data Canister:${NC}    ${DATA_CANISTER}"
echo ""
echo -e "${YELLOW}Business Logic Layer:${NC}"
echo -e "  ${GREEN}User Canister:${NC}    ${USER_CANISTER}"
echo -e "  ${GREEN}Wallet Canister:${NC}  ${WALLET_CANISTER}"
echo -e "  ${GREEN}Crypto Canister:${NC}  ${CRYPTO_CANISTER}"
echo -e "  ${GREEN}Agent Canister:${NC}   ${AGENT_CANISTER}"
echo ""
echo -e "${YELLOW}Presentation Layer:${NC}"
echo -e "  ${GREEN}USSD Canister:${NC}    ${USSD_CANISTER}"
echo ""

# ============================================================================
# Display Authorization Architecture
# ============================================================================

if [ -f "${AUTH_FILE}" ]; then
    AUTH_DATE=$(cat ${AUTH_FILE} | grep "configured_at" | cut -d'"' -f4)
    
    echo -e "${CYAN}🔐 Authorization Architecture${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}Configured:${NC}   ${AUTH_DATE}"
    echo ""
    
    echo -e "${YELLOW}Call Flow:${NC}"
    echo -e "  ${CYAN}USSD Canister${NC}"
    echo -e "      ↓ (authorized)"
    echo -e "  ${CYAN}Domain Canisters${NC} (User, Wallet, Crypto, Agent)"
    echo -e "      ↓ (authorized)"
    echo -e "  ${CYAN}Data Canister${NC}"
    echo ""
    
    echo -e "${YELLOW}Admin Access:${NC}"
    echo -e "  ${RED}Admin${NC} → ${GREEN}All Canisters${NC} (emergency access)"
    echo ""
fi

# ============================================================================
# Display URLs
# ============================================================================

echo -e "${CYAN}🌐 Access URLs${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

if [ "${NETWORK}" == "playground" ] || [ "${NETWORK}" == "ic" ]; then
    echo -e "${GREEN}Data Canister:${NC}    https://${DATA_CANISTER}.ic0.app"
    echo -e "${GREEN}User Canister:${NC}    https://${USER_CANISTER}.ic0.app"
    echo -e "${GREEN}Wallet Canister:${NC}  https://${WALLET_CANISTER}.ic0.app"
    echo -e "${GREEN}Crypto Canister:${NC}  https://${CRYPTO_CANISTER}.ic0.app"
    echo -e "${GREEN}Agent Canister:${NC}   https://${AGENT_CANISTER}.ic0.app"
    echo -e "${GREEN}USSD Canister:${NC}    https://${USSD_CANISTER}.ic0.app"
elif [ "${NETWORK}" == "local" ]; then
    echo -e "${GREEN}Data Canister:${NC}    http://localhost:8000/?canisterId=${DATA_CANISTER}"
    echo -e "${GREEN}User Canister:${NC}    http://localhost:8000/?canisterId=${USER_CANISTER}"
    echo -e "${GREEN}Wallet Canister:${NC}  http://localhost:8000/?canisterId=${WALLET_CANISTER}"
    echo -e "${GREEN}Crypto Canister:${NC}  http://localhost:8000/?canisterId=${CRYPTO_CANISTER}"
    echo -e "${GREEN}Agent Canister:${NC}   http://localhost:8000/?canisterId=${AGENT_CANISTER}"
    echo -e "${GREEN}USSD Canister:${NC}    http://localhost:8000/?canisterId=${USSD_CANISTER}"
fi

echo ""

# ============================================================================
# Display Quick Commands
# ============================================================================

echo -e "${CYAN}⚡ Quick Commands${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${YELLOW}Check status:${NC}"
echo -e "  ${GREEN}bash scripts/admin.sh status ${NETWORK}${NC}"
echo ""
echo -e "${YELLOW}Add admin:${NC}"
echo -e "  ${GREEN}bash scripts/admin.sh add-admin ${NETWORK} <principal>${NC}"
echo ""
echo -e "${YELLOW}Test deployment:${NC}"
echo -e "  ${GREEN}bash scripts/test-deployment.sh ${NETWORK}${NC}"
echo ""
echo -e "${YELLOW}Call canister:${NC}"
echo -e "  ${GREEN}dfx canister call user_canister <function> --network ${NETWORK}${NC}"
echo ""
echo -e "${YELLOW}View logs:${NC}"
echo -e "  ${GREEN}dfx canister logs ${DATA_CANISTER} --network ${NETWORK}${NC}"
echo ""

echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
