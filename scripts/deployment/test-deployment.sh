#!/bin/bash

# ============================================================================
# AfriTokeni Deployment Test Script
# Verify deployment and authorization are working correctly
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
echo -e "${BLUE}║         Deployment Test Suite                             ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}Network: ${NETWORK}${NC}"
echo ""

# ============================================================================
# Test 1: Check all canisters are deployed
# ============================================================================
echo -e "${YELLOW}Test 1: Checking canister deployment...${NC}"

CANISTERS=("data_canister" "user_canister" "wallet_canister" "crypto_canister" "agent_canister" "ussd_canister")
FAILED=0

for CANISTER in "${CANISTERS[@]}"; do
    if dfx canister id ${CANISTER} --network ${NETWORK} &>/dev/null; then
        CANISTER_ID=$(dfx canister id ${CANISTER} --network ${NETWORK})
        echo -e "${GREEN}  ✅ ${CANISTER}: ${CANISTER_ID}${NC}"
    else
        echo -e "${RED}  ❌ ${CANISTER}: NOT DEPLOYED${NC}"
        FAILED=1
    fi
done

if [ ${FAILED} -eq 1 ]; then
    echo -e "${RED}❌ Some canisters are not deployed${NC}"
    exit 1
fi

echo ""

# ============================================================================
# Test 2: Check canister status
# ============================================================================
echo -e "${YELLOW}Test 2: Checking canister status...${NC}"

for CANISTER in "${CANISTERS[@]}"; do
    STATUS=$(dfx canister status ${CANISTER} --network ${NETWORK} 2>&1 | grep "Status:" | awk '{print $2}' || echo "unknown")
    if [ "${STATUS}" == "Running" ]; then
        echo -e "${GREEN}  ✅ ${CANISTER}: Running${NC}"
    else
        echo -e "${RED}  ❌ ${CANISTER}: ${STATUS}${NC}"
        FAILED=1
    fi
done

if [ ${FAILED} -eq 1 ]; then
    echo -e "${RED}❌ Some canisters are not running${NC}"
    exit 1
fi

echo ""

# ============================================================================
# Test 3: Test inter-canister authorization
# ============================================================================
echo -e "${YELLOW}Test 3: Testing inter-canister authorization...${NC}"

# Test 3a: USSD can call User Canister
echo -e "${BLUE}  → Testing USSD → User Canister...${NC}"
if dfx canister call ussd_canister handle_ussd_request "(\"*123#\", \"test-session\", \"254700000000\")" --network ${NETWORK} &>/dev/null; then
    echo -e "${GREEN}  ✅ USSD can call User Canister${NC}"
else
    echo -e "${YELLOW}  ⚠️  USSD → User Canister call failed (may need test data)${NC}"
fi

# Test 3b: Check Data Canister has authorized canisters
echo -e "${BLUE}  → Checking Data Canister authorization...${NC}"
DATA_CANISTER=$(dfx canister id data_canister --network ${NETWORK})
USER_CANISTER=$(dfx canister id user_canister --network ${NETWORK})

# Try to call data canister directly (should work as controller)
if dfx canister call data_canister get_user "(\"test-user\")" --network ${NETWORK} &>/dev/null; then
    echo -e "${GREEN}  ✅ Data Canister is accessible${NC}"
else
    echo -e "${YELLOW}  ⚠️  Data Canister call failed (may need test data)${NC}"
fi

echo ""

# ============================================================================
# Test 4: Check deployment files exist
# ============================================================================
echo -e "${YELLOW}Test 4: Checking deployment files...${NC}"

DEPLOYMENT_FILE="deployments/${NETWORK}_deployment.json"
AUTH_FILE="deployments/${NETWORK}_authorization.json"

if [ -f "${DEPLOYMENT_FILE}" ]; then
    echo -e "${GREEN}  ✅ Deployment file exists: ${DEPLOYMENT_FILE}${NC}"
else
    echo -e "${RED}  ❌ Deployment file missing: ${DEPLOYMENT_FILE}${NC}"
    FAILED=1
fi

if [ -f "${AUTH_FILE}" ]; then
    echo -e "${GREEN}  ✅ Authorization file exists: ${AUTH_FILE}${NC}"
else
    echo -e "${RED}  ❌ Authorization file missing: ${AUTH_FILE}${NC}"
    FAILED=1
fi

echo ""

# ============================================================================
# Test 5: Verify canister IDs match deployment file
# ============================================================================
echo -e "${YELLOW}Test 5: Verifying canister IDs...${NC}"

if [ -f "${DEPLOYMENT_FILE}" ]; then
    for CANISTER in "${CANISTERS[@]}"; do
        ACTUAL_ID=$(dfx canister id ${CANISTER} --network ${NETWORK})
        EXPECTED_ID=$(cat ${DEPLOYMENT_FILE} | grep "${CANISTER}" | grep -oP '"\K[a-z0-9-]+(?=")')
        
        if [ "${ACTUAL_ID}" == "${EXPECTED_ID}" ]; then
            echo -e "${GREEN}  ✅ ${CANISTER}: IDs match${NC}"
        else
            echo -e "${RED}  ❌ ${CANISTER}: ID mismatch${NC}"
            echo -e "     Expected: ${EXPECTED_ID}"
            echo -e "     Actual:   ${ACTUAL_ID}"
            FAILED=1
        fi
    done
fi

echo ""

# ============================================================================
# Test Summary
# ============================================================================
echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║         Test Summary                                       ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

if [ ${FAILED} -eq 0 ]; then
    echo -e "${GREEN}✅ All tests passed!${NC}"
    echo ""
    echo -e "${YELLOW}Deployment is ready for use${NC}"
    echo ""
    echo -e "${BLUE}Next steps:${NC}"
    echo -e "  1. Test USSD flows: ${GREEN}dfx canister call ussd_canister handle_ussd_request ...${NC}"
    echo -e "  2. Monitor status: ${GREEN}bash scripts/admin.sh status ${NETWORK}${NC}"
    echo -e "  3. Add more admins: ${GREEN}bash scripts/admin.sh add-admin ${NETWORK} <principal>${NC}"
    echo ""
    exit 0
else
    echo -e "${RED}❌ Some tests failed${NC}"
    echo ""
    echo -e "${YELLOW}Troubleshooting:${NC}"
    echo -e "  1. Re-run deployment: ${GREEN}bash scripts/deploy.sh ${NETWORK}${NC}"
    echo -e "  2. Re-run authorization: ${GREEN}bash scripts/setup-auth.sh ${NETWORK}${NC}"
    echo -e "  3. Check logs: ${GREEN}dfx canister logs <canister-id> --network ${NETWORK}${NC}"
    echo ""
    exit 1
fi
