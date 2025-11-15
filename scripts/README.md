# AfriTokeni Deployment Scripts

This directory contains deployment and management scripts for AfriTokeni canisters on the Internet Computer.

## Scripts Overview

### 🚀 deploy.sh
**Main deployment script** - Deploys all canisters in the correct order and sets up authorization.

```bash
bash scripts/deploy.sh <network> [admin-principal]
```

**Examples:**
```bash
# Deploy to playground
bash scripts/deploy.sh playground

# Deploy to IC mainnet with specific admin
bash scripts/deploy.sh ic $(dfx identity get-principal)

# Deploy to local network
bash scripts/deploy.sh local
```

**What it does:**
1. Builds all canisters
2. Deploys Data Canister (storage layer)
3. Deploys Domain Canisters (user, wallet, crypto, agent)
4. Deploys USSD Canister (presentation layer)
5. Saves deployment info to `deployments/<network>_deployment.json`
6. Automatically runs `setup-auth.sh`

---

### 🔐 setup-auth.sh
**Authorization configuration** - Sets up inter-canister permissions.

```bash
bash scripts/setup-auth.sh <network>
```

**Examples:**
```bash
bash scripts/setup-auth.sh playground
bash scripts/setup-auth.sh ic
```

**What it does:**
1. Authorizes domain canisters to call Data Canister
2. Authorizes USSD Canister to call domain canisters
3. Configures canister IDs in each canister
4. Saves authorization config to `deployments/<network>_authorization.json`

**Authorization Flow:**
```
USSD Canister
    ↓ (authorized)
Domain Canisters (User, Wallet, Crypto, Agent)
    ↓ (authorized)
Data Canister
```

---

### 👨‍💼 admin.sh
**Admin management** - Manage admin access and perform emergency operations.

```bash
bash scripts/admin.sh <command> <network> [principal]
```

#### Commands:

**add-admin** - Add admin to all canisters
```bash
bash scripts/admin.sh add-admin playground abc123-def456
```

**remove-admin** - Remove admin from all canisters
```bash
bash scripts/admin.sh remove-admin playground abc123-def456
```

**list-admins** - List all admins (controllers)
```bash
bash scripts/admin.sh list-admins playground
```

**emergency-stop** - Stop all canisters (emergency only!)
```bash
bash scripts/admin.sh emergency-stop playground
```

**emergency-start** - Start all canisters
```bash
bash scripts/admin.sh emergency-start playground
```

**status** - Check status of all canisters
```bash
bash scripts/admin.sh status playground
```

**upgrade** - Upgrade all canisters to latest code
```bash
bash scripts/admin.sh upgrade playground
```

---

### ✅ test-deployment.sh
**Deployment verification** - Test that deployment was successful.

```bash
bash scripts/test-deployment.sh <network>
```

**Examples:**
```bash
bash scripts/test-deployment.sh playground
bash scripts/test-deployment.sh ic
```

**What it tests:**
1. ✅ All canisters are deployed
2. ✅ All canisters are running
3. ✅ Inter-canister authorization works
4. ✅ Deployment files exist
5. ✅ Canister IDs match deployment file

---

## Quick Start

### First Time Deployment

```bash
# 1. Make scripts executable
chmod +x scripts/*.sh

# 2. Deploy to playground
bash scripts/deploy.sh playground

# 3. Test deployment
bash scripts/test-deployment.sh playground

# 4. Check status
bash scripts/admin.sh status playground
```

### Add Another Admin

```bash
# Get your principal
dfx identity get-principal

# Add as admin
bash scripts/admin.sh add-admin playground <your-principal>
```

### Upgrade After Code Changes

```bash
# Build and upgrade all canisters
bash scripts/admin.sh upgrade playground
```

### Emergency Stop

```bash
# Stop all canisters
bash scripts/admin.sh emergency-stop playground

# Start them again
bash scripts/admin.sh emergency-start playground
```

---

## File Structure

```
scripts/
├── deploy.sh              # Main deployment script
├── setup-auth.sh          # Authorization setup
├── admin.sh               # Admin management
├── test-deployment.sh     # Deployment tests
└── README.md              # This file

deployments/               # Created by scripts
├── playground_deployment.json
├── playground_authorization.json
├── ic_deployment.json
└── ic_authorization.json
```

---

## Networks

### playground
- **Purpose**: Testing and development
- **Cycles**: Free (limited)
- **Duration**: Temporary
- **URL**: https://ic0.app

### ic
- **Purpose**: Production
- **Cycles**: Paid (ICP tokens)
- **Duration**: Permanent
- **URL**: https://ic0.app

### local
- **Purpose**: Local development
- **Cycles**: Unlimited (simulated)
- **Duration**: Until `dfx stop`
- **URL**: http://localhost:8000

---

## Troubleshooting

### "Unauthorized caller" error

**Solution**: Re-run authorization setup
```bash
bash scripts/setup-auth.sh playground
```

### "Canister not found" error

**Solution**: Check deployment or re-deploy
```bash
cat deployments/playground_deployment.json
bash scripts/deploy.sh playground
```

### Canister out of cycles

**Solution**: Check cycles and add more (mainnet only)
```bash
dfx canister status <canister-id> --network ic
dfx canister deposit-cycles <amount> <canister-id> --network ic
```

### Script permission denied

**Solution**: Make scripts executable
```bash
chmod +x scripts/*.sh
```

---

## Best Practices

1. **Always test on playground first** before deploying to mainnet
2. **Run test-deployment.sh** after every deployment
3. **Keep deployment files** in version control (they're in `deployments/`)
4. **Limit admin access** - only add necessary controllers
5. **Monitor regularly** - use `admin.sh status` to check canister health
6. **Backup before upgrades** - canisters have stable storage but be cautious

---

## Security Notes

- **Controllers (Admins)** have full access to canisters
- **Authorized Canisters** can only call specific functions
- **Regular Users** can only access their own data
- **Emergency Stop** should only be used in critical situations

---

## Support

For more information, see:
- [DEPLOYMENT.md](../DEPLOYMENT.md) - Full deployment guide
- [dfx documentation](https://internetcomputer.org/docs/current/developer-docs/setup/install/)
- [IC Dashboard](https://dashboard.internetcomputer.org/)

---

**Last Updated**: November 2025
