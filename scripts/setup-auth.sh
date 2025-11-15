#!/bin/bash

# ============================================================================
# AfriTokeni Authorization Setup Script
# Sets up inter-canister communication permissions
# ============================================================================

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

NETWORK="${1:-playground}"

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║         Authorization Setup                                ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Get canister IDs
DATA_CANISTER=$(dfx canister id data_canister --network ${NETWORK})
USER_CANISTER=$(dfx canister id user_canister --network ${NETWORK})
WALLET_CANISTER=$(dfx canister id wallet_canister --network ${NETWORK})
CRYPTO_CANISTER=$(dfx canister id crypto_canister --network ${NETWORK})
AGENT_CANISTER=$(dfx canister id agent_canister --network ${NETWORK})
USSD_CANISTER=$(dfx canister id ussd_canister --network ${NETWORK})

echo -e "${GREEN}Canister IDs loaded:${NC}"
echo -e "  Data:    ${DATA_CANISTER}"
echo -e "  User:    ${USER_CANISTER}"
echo -e "  Wallet:  ${WALLET_CANISTER}"
echo -e "  Crypto:  ${CRYPTO_CANISTER}"
echo -e "  Agent:   ${AGENT_CANISTER}"
echo -e "  USSD:    ${USSD_CANISTER}"
echo ""

# ============================================================================
# Authorization Architecture:
# 
# 1. DATA CANISTER (Storage Layer)
#    - Can be called by: Domain canisters (user, wallet, crypto, agent)
#    - Cannot be called by: USSD canister (must go through domain layer)
#
# 2. DOMAIN CANISTERS (Business Logic Layer)
#    - User, Wallet, Crypto, Agent canisters
#    - Can be called by: USSD canister, Admin
#    - Can call: Data canister
#
# 3. USSD CANISTER (Presentation Layer)
#    - Can be called by: External USSD gateway, Admin
#    - Can call: Domain canisters (user, wallet, crypto, agent)
#    - Cannot call: Data canister directly
#
# 4. ADMIN
#    - Can call: All canisters (emergency access)
# ============================================================================

echo -e "${YELLOW}🔐 Setting up Data Canister authorization...${NC}"
echo -e "${BLUE}  → Authorizing domain canisters to access Data Canister${NC}"

# Data canister should only accept calls from domain canisters
dfx canister call data_canister add_authorized_canister "(principal \"${USER_CANISTER}\")" --network ${NETWORK}
echo -e "${GREEN}  ✅ User Canister authorized${NC}"

dfx canister call data_canister add_authorized_canister "(principal \"${WALLET_CANISTER}\")" --network ${NETWORK}
echo -e "${GREEN}  ✅ Wallet Canister authorized${NC}"

dfx canister call data_canister add_authorized_canister "(principal \"${CRYPTO_CANISTER}\")" --network ${NETWORK}
echo -e "${GREEN}  ✅ Crypto Canister authorized${NC}"

dfx canister call data_canister add_authorized_canister "(principal \"${AGENT_CANISTER}\")" --network ${NETWORK}
echo -e "${GREEN}  ✅ Agent Canister authorized${NC}"

echo ""

echo -e "${YELLOW}🔐 Setting up User Canister authorization...${NC}"
echo -e "${BLUE}  → Authorizing USSD canister to access User Canister${NC}"
dfx canister call user_canister add_authorized_canister "(principal \"${USSD_CANISTER}\")" --network ${NETWORK}
echo -e "${GREEN}  ✅ USSD Canister authorized${NC}"

echo -e "${BLUE}  → Setting Data Canister ID in User Canister${NC}"
dfx canister call user_canister set_data_canister_id "(principal \"${DATA_CANISTER}\")" --network ${NETWORK}
echo -e "${GREEN}  ✅ Data Canister ID configured${NC}"

echo ""

echo -e "${YELLOW}🔐 Setting up Wallet Canister authorization...${NC}"
echo -e "${BLUE}  → Authorizing USSD canister to access Wallet Canister${NC}"
dfx canister call wallet_canister add_authorized_canister "(principal \"${USSD_CANISTER}\")" --network ${NETWORK}
echo -e "${GREEN}  ✅ USSD Canister authorized${NC}"

echo -e "${BLUE}  → Setting Data Canister ID in Wallet Canister${NC}"
dfx canister call wallet_canister set_data_canister_id "(principal \"${DATA_CANISTER}\")" --network ${NETWORK}
echo -e "${GREEN}  ✅ Data Canister ID configured${NC}"

echo -e "${BLUE}  → Setting User Canister ID in Wallet Canister${NC}"
dfx canister call wallet_canister set_user_canister_id "(principal \"${USER_CANISTER}\")" --network ${NETWORK}
echo -e "${GREEN}  ✅ User Canister ID configured${NC}"

echo ""

echo -e "${YELLOW}🔐 Setting up Crypto Canister authorization...${NC}"
echo -e "${BLUE}  → Authorizing USSD canister to access Crypto Canister${NC}"
dfx canister call crypto_canister add_authorized_canister "(principal \"${USSD_CANISTER}\")" --network ${NETWORK}
echo -e "${GREEN}  ✅ USSD Canister authorized${NC}"

echo -e "${BLUE}  → Setting Data Canister ID in Crypto Canister${NC}"
dfx canister call crypto_canister set_data_canister_id "(principal \"${DATA_CANISTER}\")" --network ${NETWORK}
echo -e "${GREEN}  ✅ Data Canister ID configured${NC}"

echo ""

echo -e "${YELLOW}🔐 Setting up Agent Canister authorization...${NC}"
echo -e "${BLUE}  → Authorizing USSD canister to access Agent Canister${NC}"
dfx canister call agent_canister add_authorized_canister "(principal \"${USSD_CANISTER}\")" --network ${NETWORK}
echo -e "${GREEN}  ✅ USSD Canister authorized${NC}"

echo -e "${BLUE}  → Setting Data Canister ID in Agent Canister${NC}"
dfx canister call agent_canister set_data_canister_id "(principal \"${DATA_CANISTER}\")" --network ${NETWORK}
echo -e "${GREEN}  ✅ Data Canister ID configured${NC}"

echo -e "${BLUE}  → Setting Wallet Canister ID in Agent Canister${NC}"
dfx canister call agent_canister set_wallet_canister_id "(principal \"${WALLET_CANISTER}\")" --network ${NETWORK}
echo -e "${GREEN}  ✅ Wallet Canister ID configured${NC}"

echo ""

echo -e "${YELLOW}🔐 Setting up USSD Canister configuration...${NC}"
echo -e "${BLUE}  → Configuring domain canister IDs in USSD Canister${NC}"
dfx canister call ussd_canister set_user_canister_id "(principal \"${USER_CANISTER}\")" --network ${NETWORK}
echo -e "${GREEN}  ✅ User Canister ID configured${NC}"

dfx canister call ussd_canister set_wallet_canister_id "(principal \"${WALLET_CANISTER}\")" --network ${NETWORK}
echo -e "${GREEN}  ✅ Wallet Canister ID configured${NC}"

dfx canister call ussd_canister set_crypto_canister_id "(principal \"${CRYPTO_CANISTER}\")" --network ${NETWORK}
echo -e "${GREEN}  ✅ Crypto Canister ID configured${NC}"

dfx canister call ussd_canister set_agent_canister_id "(principal \"${AGENT_CANISTER}\")" --network ${NETWORK}
echo -e "${GREEN}  ✅ Agent Canister ID configured${NC}"

echo ""

# ============================================================================
# Save authorization configuration
# ============================================================================
AUTH_FILE="deployments/${NETWORK}_authorization.json"
mkdir -p deployments

cat > ${AUTH_FILE} << EOF
{
  "network": "${NETWORK}",
  "configured_at": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "architecture": {
    "data_canister": {
      "id": "${DATA_CANISTER}",
      "authorized_callers": [
        "${USER_CANISTER}",
        "${WALLET_CANISTER}",
        "${CRYPTO_CANISTER}",
        "${AGENT_CANISTER}"
      ],
      "description": "Storage layer - only domain canisters can access"
    },
    "domain_canisters": {
      "user_canister": {
        "id": "${USER_CANISTER}",
        "authorized_callers": ["${USSD_CANISTER}"],
        "can_call": ["${DATA_CANISTER}"]
      },
      "wallet_canister": {
        "id": "${WALLET_CANISTER}",
        "authorized_callers": ["${USSD_CANISTER}"],
        "can_call": ["${DATA_CANISTER}", "${USER_CANISTER}"]
      },
      "crypto_canister": {
        "id": "${CRYPTO_CANISTER}",
        "authorized_callers": ["${USSD_CANISTER}"],
        "can_call": ["${DATA_CANISTER}"]
      },
      "agent_canister": {
        "id": "${AGENT_CANISTER}",
        "authorized_callers": ["${USSD_CANISTER}"],
        "can_call": ["${DATA_CANISTER}", "${WALLET_CANISTER}"]
      }
    },
    "presentation_layer": {
      "ussd_canister": {
        "id": "${USSD_CANISTER}",
        "can_call": [
          "${USER_CANISTER}",
          "${WALLET_CANISTER}",
          "${CRYPTO_CANISTER}",
          "${AGENT_CANISTER}"
        ],
        "description": "Presentation layer - can only call domain canisters"
      }
    }
  }
}
EOF

echo -e "${GREEN}✅ Authorization configuration saved to ${AUTH_FILE}${NC}"
echo ""

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║         Authorization Setup Complete                      ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}✅ All inter-canister permissions configured${NC}"
echo -e "${YELLOW}📝 Architecture:${NC}"
echo -e "   ${BLUE}USSD${NC} → ${GREEN}Domain Canisters${NC} → ${YELLOW}Data Canister${NC}"
echo -e "   ${RED}Admin${NC} → ${GREEN}All Canisters${NC} (emergency access)"
echo ""
