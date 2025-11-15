#!/bin/bash

# ============================================================================
# AfriTokeni Admin Management Script
# Manage admin access and emergency operations
# ============================================================================

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

COMMAND="${1}"
NETWORK="${2:-playground}"
PRINCIPAL="${3}"

# ============================================================================
# Helper Functions
# ============================================================================

show_usage() {
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║         AfriTokeni Admin Management                       ║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${YELLOW}Usage:${NC}"
    echo -e "  ${GREEN}bash scripts/admin.sh <command> <network> [principal]${NC}"
    echo ""
    echo -e "${YELLOW}Commands:${NC}"
    echo -e "  ${BLUE}add-admin${NC}         Add admin to all canisters"
    echo -e "  ${BLUE}remove-admin${NC}      Remove admin from all canisters"
    echo -e "  ${BLUE}list-admins${NC}       List all admins"
    echo -e "  ${BLUE}emergency-stop${NC}    Stop all canisters (emergency)"
    echo -e "  ${BLUE}emergency-start${NC}   Start all canisters"
    echo -e "  ${BLUE}status${NC}            Check canister status"
    echo -e "  ${BLUE}upgrade${NC}           Upgrade all canisters"
    echo ""
    echo -e "${YELLOW}Networks:${NC}"
    echo -e "  ${GREEN}local${NC}            Local dfx network"
    echo -e "  ${GREEN}playground${NC}       IC playground"
    echo -e "  ${GREEN}ic${NC}               IC mainnet"
    echo ""
    echo -e "${YELLOW}Examples:${NC}"
    echo -e "  ${GREEN}bash scripts/admin.sh add-admin playground abc123-def456${NC}"
    echo -e "  ${GREEN}bash scripts/admin.sh status playground${NC}"
    echo -e "  ${GREEN}bash scripts/admin.sh emergency-stop ic${NC}"
    echo ""
}

get_canister_ids() {
    DATA_CANISTER=$(dfx canister id data_canister --network ${NETWORK} 2>/dev/null || echo "")
    USER_CANISTER=$(dfx canister id user_canister --network ${NETWORK} 2>/dev/null || echo "")
    WALLET_CANISTER=$(dfx canister id wallet_canister --network ${NETWORK} 2>/dev/null || echo "")
    CRYPTO_CANISTER=$(dfx canister id crypto_canister --network ${NETWORK} 2>/dev/null || echo "")
    AGENT_CANISTER=$(dfx canister id agent_canister --network ${NETWORK} 2>/dev/null || echo "")
    USSD_CANISTER=$(dfx canister id ussd_canister --network ${NETWORK} 2>/dev/null || echo "")
}

# ============================================================================
# Command: Add Admin
# ============================================================================

add_admin() {
    if [ -z "${PRINCIPAL}" ]; then
        echo -e "${RED}❌ Error: Principal required${NC}"
        echo -e "${YELLOW}Usage: bash scripts/admin.sh add-admin ${NETWORK} <principal>${NC}"
        exit 1
    fi

    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║         Adding Admin to All Canisters                     ║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${GREEN}Network:   ${NETWORK}${NC}"
    echo -e "${GREEN}Principal: ${PRINCIPAL}${NC}"
    echo ""

    get_canister_ids

    # Add as controller to all canisters
    for CANISTER in "${DATA_CANISTER}" "${USER_CANISTER}" "${WALLET_CANISTER}" "${CRYPTO_CANISTER}" "${AGENT_CANISTER}" "${USSD_CANISTER}"; do
        if [ -n "${CANISTER}" ]; then
            CANISTER_NAME=$(dfx canister info ${CANISTER} --network ${NETWORK} 2>/dev/null | grep -oP 'Name: \K.*' || echo "${CANISTER}")
            echo -e "${BLUE}  → Adding admin to ${CANISTER_NAME}...${NC}"
            dfx canister update-settings ${CANISTER} --add-controller ${PRINCIPAL} --network ${NETWORK}
            echo -e "${GREEN}  ✅ Admin added to ${CANISTER_NAME}${NC}"
        fi
    done

    echo ""
    echo -e "${GREEN}✅ Admin ${PRINCIPAL} added to all canisters${NC}"
}

# ============================================================================
# Command: Remove Admin
# ============================================================================

remove_admin() {
    if [ -z "${PRINCIPAL}" ]; then
        echo -e "${RED}❌ Error: Principal required${NC}"
        echo -e "${YELLOW}Usage: bash scripts/admin.sh remove-admin ${NETWORK} <principal>${NC}"
        exit 1
    fi

    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║         Removing Admin from All Canisters                 ║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${YELLOW}⚠️  WARNING: This will remove controller access${NC}"
    echo -e "${GREEN}Network:   ${NETWORK}${NC}"
    echo -e "${GREEN}Principal: ${PRINCIPAL}${NC}"
    echo ""
    read -p "Are you sure? (yes/no): " CONFIRM

    if [ "${CONFIRM}" != "yes" ]; then
        echo -e "${YELLOW}Cancelled${NC}"
        exit 0
    fi

    get_canister_ids

    for CANISTER in "${DATA_CANISTER}" "${USER_CANISTER}" "${WALLET_CANISTER}" "${CRYPTO_CANISTER}" "${AGENT_CANISTER}" "${USSD_CANISTER}"; do
        if [ -n "${CANISTER}" ]; then
            CANISTER_NAME=$(dfx canister info ${CANISTER} --network ${NETWORK} 2>/dev/null | grep -oP 'Name: \K.*' || echo "${CANISTER}")
            echo -e "${BLUE}  → Removing admin from ${CANISTER_NAME}...${NC}"
            dfx canister update-settings ${CANISTER} --remove-controller ${PRINCIPAL} --network ${NETWORK}
            echo -e "${GREEN}  ✅ Admin removed from ${CANISTER_NAME}${NC}"
        fi
    done

    echo ""
    echo -e "${GREEN}✅ Admin ${PRINCIPAL} removed from all canisters${NC}"
}

# ============================================================================
# Command: List Admins
# ============================================================================

list_admins() {
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║         Canister Controllers (Admins)                     ║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""

    get_canister_ids

    for CANISTER in "${DATA_CANISTER}" "${USER_CANISTER}" "${WALLET_CANISTER}" "${CRYPTO_CANISTER}" "${AGENT_CANISTER}" "${USSD_CANISTER}"; do
        if [ -n "${CANISTER}" ]; then
            CANISTER_NAME=$(dfx canister info ${CANISTER} --network ${NETWORK} 2>/dev/null | grep -oP 'Name: \K.*' || echo "${CANISTER}")
            echo -e "${YELLOW}${CANISTER_NAME} (${CANISTER}):${NC}"
            dfx canister info ${CANISTER} --network ${NETWORK} | grep "Controllers:" -A 10
            echo ""
        fi
    done
}

# ============================================================================
# Command: Emergency Stop
# ============================================================================

emergency_stop() {
    echo -e "${RED}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${RED}║         EMERGENCY STOP - All Canisters                    ║${NC}"
    echo -e "${RED}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${YELLOW}⚠️  WARNING: This will stop all canisters${NC}"
    echo -e "${GREEN}Network: ${NETWORK}${NC}"
    echo ""
    read -p "Are you sure? (yes/no): " CONFIRM

    if [ "${CONFIRM}" != "yes" ]; then
        echo -e "${YELLOW}Cancelled${NC}"
        exit 0
    fi

    get_canister_ids

    for CANISTER in "${DATA_CANISTER}" "${USER_CANISTER}" "${WALLET_CANISTER}" "${CRYPTO_CANISTER}" "${AGENT_CANISTER}" "${USSD_CANISTER}"; do
        if [ -n "${CANISTER}" ]; then
            CANISTER_NAME=$(dfx canister info ${CANISTER} --network ${NETWORK} 2>/dev/null | grep -oP 'Name: \K.*' || echo "${CANISTER}")
            echo -e "${BLUE}  → Stopping ${CANISTER_NAME}...${NC}"
            dfx canister stop ${CANISTER} --network ${NETWORK}
            echo -e "${RED}  ⏸  ${CANISTER_NAME} stopped${NC}"
        fi
    done

    echo ""
    echo -e "${RED}🛑 All canisters stopped${NC}"
}

# ============================================================================
# Command: Emergency Start
# ============================================================================

emergency_start() {
    echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║         Starting All Canisters                            ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""

    get_canister_ids

    for CANISTER in "${DATA_CANISTER}" "${USER_CANISTER}" "${WALLET_CANISTER}" "${CRYPTO_CANISTER}" "${AGENT_CANISTER}" "${USSD_CANISTER}"; do
        if [ -n "${CANISTER}" ]; then
            CANISTER_NAME=$(dfx canister info ${CANISTER} --network ${NETWORK} 2>/dev/null | grep -oP 'Name: \K.*' || echo "${CANISTER}")
            echo -e "${BLUE}  → Starting ${CANISTER_NAME}...${NC}"
            dfx canister start ${CANISTER} --network ${NETWORK}
            echo -e "${GREEN}  ▶️  ${CANISTER_NAME} started${NC}"
        fi
    done

    echo ""
    echo -e "${GREEN}✅ All canisters started${NC}"
}

# ============================================================================
# Command: Status
# ============================================================================

check_status() {
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║         Canister Status                                   ║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""

    get_canister_ids

    for CANISTER in "${DATA_CANISTER}" "${USER_CANISTER}" "${WALLET_CANISTER}" "${CRYPTO_CANISTER}" "${AGENT_CANISTER}" "${USSD_CANISTER}"; do
        if [ -n "${CANISTER}" ]; then
            CANISTER_NAME=$(dfx canister info ${CANISTER} --network ${NETWORK} 2>/dev/null | grep -oP 'Name: \K.*' || echo "${CANISTER}")
            echo -e "${YELLOW}${CANISTER_NAME}:${NC}"
            dfx canister status ${CANISTER} --network ${NETWORK}
            echo ""
        fi
    done
}

# ============================================================================
# Command: Upgrade
# ============================================================================

upgrade_canisters() {
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║         Upgrading All Canisters                           ║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${YELLOW}⚠️  This will upgrade all canisters to latest code${NC}"
    echo ""
    read -p "Continue? (yes/no): " CONFIRM

    if [ "${CONFIRM}" != "yes" ]; then
        echo -e "${YELLOW}Cancelled${NC}"
        exit 0
    fi

    echo -e "${BLUE}  → Building canisters...${NC}"
    dfx build --network ${NETWORK}
    echo -e "${GREEN}  ✅ Build complete${NC}"
    echo ""

    get_canister_ids

    for CANISTER_NAME in "data_canister" "user_canister" "wallet_canister" "crypto_canister" "agent_canister" "ussd_canister"; do
        echo -e "${BLUE}  → Upgrading ${CANISTER_NAME}...${NC}"
        dfx canister install ${CANISTER_NAME} --mode upgrade --network ${NETWORK}
        echo -e "${GREEN}  ✅ ${CANISTER_NAME} upgraded${NC}"
    done

    echo ""
    echo -e "${GREEN}✅ All canisters upgraded${NC}"
}

# ============================================================================
# Main Script
# ============================================================================

case "${COMMAND}" in
    add-admin)
        add_admin
        ;;
    remove-admin)
        remove_admin
        ;;
    list-admins)
        list_admins
        ;;
    emergency-stop)
        emergency_stop
        ;;
    emergency-start)
        emergency_start
        ;;
    status)
        check_status
        ;;
    upgrade)
        upgrade_canisters
        ;;
    *)
        show_usage
        exit 1
        ;;
esac
