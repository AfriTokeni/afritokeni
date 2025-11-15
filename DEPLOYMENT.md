# AfriTokeni Deployment Guide

## Overview

This guide explains how to deploy AfriTokeni canisters to the Internet Computer (IC) with proper authorization and access control, similar to Hardhat Ignition for Ethereum.

## Architecture

### 3-Tier Architecture

```
┌─────────────────────────────────────────────────────────┐
│                  Presentation Layer                      │
│  ┌────────────────────────────────────────────────┐    │
│  │           USSD Canister                        │    │
│  │  - USSD interface                              │    │
│  │  - Can call: Domain canisters only             │    │
│  └────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────┐
│                  Business Logic Layer                    │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────┐ │
│  │  User    │  │  Wallet  │  │  Crypto  │  │ Agent  │ │
│  │ Canister │  │ Canister │  │ Canister │  │Canister│ │
│  └──────────┘  └──────────┘  └──────────┘  └────────┘ │
│       ↓              ↓              ↓            ↓      │
└─────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────┐
│                    Storage Layer                         │
│  ┌────────────────────────────────────────────────┐    │
│  │           Data Canister                        │    │
│  │  - Pure CRUD operations                        │    │
│  │  - Can be called by: Domain canisters only     │    │
│  └────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
```

### Authorization Model

| Canister | Can Call | Can Be Called By |
|----------|----------|------------------|
| **Data Canister** | None | User, Wallet, Crypto, Agent, Admin |
| **User Canister** | Data | USSD, Admin |
| **Wallet Canister** | Data, User | USSD, Admin |
| **Crypto Canister** | Data | USSD, Admin |
| **Agent Canister** | Data, Wallet | USSD, Admin |
| **USSD Canister** | User, Wallet, Crypto, Agent | External Gateway, Admin |

**Admin** has emergency access to all canisters.

## Prerequisites

1. **Install dfx** (DFX SDK)
   ```bash
   sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
   ```

2. **Create Identity**
   ```bash
   dfx identity new my-identity
   dfx identity use my-identity
   ```

3. **Get Principal**
   ```bash
   dfx identity get-principal
   ```

## Deployment Steps

### 1. Deploy to Playground (Testing)

```bash
# Make scripts executable
chmod +x scripts/*.sh

# Deploy all canisters
bash scripts/deploy.sh playground

# This will:
# - Build all canisters
# - Deploy in correct order (Data → Domain → USSD)
# - Set up authorization automatically
# - Save deployment info to deployments/playground_deployment.json
```

### 2. Deploy to IC Mainnet (Production)

```bash
# Deploy to mainnet
bash scripts/deploy.sh ic $(dfx identity get-principal)

# This deploys to production with your principal as admin
```

### 3. Deploy to Local Network (Development)

```bash
# Start local replica
dfx start --background --clean

# Deploy locally
bash scripts/deploy.sh local
```

## Post-Deployment Configuration

### Add Additional Admins

```bash
# Add another admin principal
bash scripts/admin.sh add-admin playground <principal-id>

# Example:
bash scripts/admin.sh add-admin playground abc123-def456-xyz789
```

### Remove Admin

```bash
bash scripts/admin.sh remove-admin playground <principal-id>
```

### List All Admins

```bash
bash scripts/admin.sh list-admins playground
```

## Authorization Setup

The authorization is set up automatically during deployment, but you can re-run it:

```bash
bash scripts/setup-auth.sh playground
```

This configures:

1. **Data Canister** - Authorizes domain canisters (user, wallet, crypto, agent)
2. **Domain Canisters** - Authorizes USSD canister and sets data canister ID
3. **USSD Canister** - Sets all domain canister IDs

## Emergency Operations

### Stop All Canisters

```bash
bash scripts/admin.sh emergency-stop playground
```

⚠️ **Use only in emergencies** - This stops all canisters and makes the platform unavailable.

### Start All Canisters

```bash
bash scripts/admin.sh emergency-start playground
```

### Check Status

```bash
bash scripts/admin.sh status playground
```

### Upgrade Canisters

```bash
bash scripts/admin.sh upgrade playground
```

## Deployment Files

After deployment, you'll find:

```
deployments/
├── playground_deployment.json      # Canister IDs and deployment info
├── playground_authorization.json   # Authorization configuration
├── ic_deployment.json             # Production deployment (if deployed to IC)
└── ic_authorization.json          # Production authorization
```

### Example deployment.json

```json
{
  "network": "playground",
  "deployed_at": "2025-11-15T08:35:00Z",
  "admin_principal": "abc123-def456",
  "canisters": {
    "data_canister": "rrkah-fqaaa-aaaaa-aaaaq-cai",
    "user_canister": "ryjl3-tyaaa-aaaaa-aaaba-cai",
    "wallet_canister": "r7inp-6aaaa-aaaaa-aaabq-cai",
    "crypto_canister": "renrk-eyaaa-aaaaa-aaada-cai",
    "agent_canister": "rno2w-sqaaa-aaaaa-aaacq-cai",
    "ussd_canister": "rkp4c-7iaaa-aaaaa-aaaca-cai"
  }
}
```

## Security Best Practices

### 1. Controller Management

- **Minimum Controllers**: Only add necessary controllers
- **Regular Audits**: Review controllers monthly
- **Remove Unused**: Remove controllers that are no longer needed

```bash
# List current controllers
bash scripts/admin.sh list-admins playground

# Remove unused controller
bash scripts/admin.sh remove-admin playground <old-principal>
```

### 2. Authorization Verification

After deployment, verify authorization:

```bash
# Check authorization configuration
cat deployments/playground_authorization.json

# Test inter-canister calls
dfx canister call user_canister get_user "(\"test-user-id\")" --network playground
```

### 3. Monitoring

Monitor canister status regularly:

```bash
# Check all canister statuses
bash scripts/admin.sh status playground

# Check specific canister
dfx canister status data_canister --network playground
```

## Troubleshooting

### Issue: "Unauthorized caller"

**Cause**: Canister not authorized to call another canister

**Solution**:
```bash
# Re-run authorization setup
bash scripts/setup-auth.sh playground
```

### Issue: "Canister not found"

**Cause**: Canister not deployed or wrong network

**Solution**:
```bash
# Check deployment
cat deployments/playground_deployment.json

# Re-deploy if needed
bash scripts/deploy.sh playground
```

### Issue: "Out of cycles"

**Cause**: Canister ran out of cycles

**Solution**:
```bash
# Check cycles balance
dfx canister status <canister-id> --network playground

# Add cycles (on mainnet)
dfx canister deposit-cycles <amount> <canister-id> --network ic
```

## Comparison with Ethereum/Hardhat

| Ethereum (Hardhat Ignition) | ICP (AfriTokeni Scripts) |
|----------------------------|--------------------------|
| `npx hardhat ignition deploy` | `bash scripts/deploy.sh` |
| `hardhat.config.js` | `dfx.json` |
| Contract addresses | Canister IDs |
| `msg.sender` authorization | Principal-based authorization |
| Gas fees | Cycles |
| Upgradeable proxies | Native canister upgrades |

## Network Configuration

### Playground (Testing)

- **URL**: https://ic0.app
- **Cycles**: Free (limited)
- **Duration**: Temporary (deleted after inactivity)
- **Use**: Testing and development

### IC Mainnet (Production)

- **URL**: https://ic0.app
- **Cycles**: Paid (ICP tokens)
- **Duration**: Permanent
- **Use**: Production deployment

### Local (Development)

- **URL**: http://localhost:8000
- **Cycles**: Unlimited (simulated)
- **Duration**: Until dfx stop
- **Use**: Local development

## Next Steps

1. ✅ Deploy canisters: `bash scripts/deploy.sh playground`
2. ✅ Verify authorization: `cat deployments/playground_authorization.json`
3. ✅ Add admin: `bash scripts/admin.sh add-admin playground <principal>`
4. ✅ Test deployment: Call canister functions
5. ✅ Monitor: `bash scripts/admin.sh status playground`

## Support

For issues or questions:
- Check `deployments/` folder for configuration
- Review canister logs: `dfx canister logs <canister-id> --network playground`
- Run status check: `bash scripts/admin.sh status playground`

---

**Last Updated**: November 2025
**Version**: 1.0.0
