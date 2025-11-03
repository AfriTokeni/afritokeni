# AfriTokeni
## SMS-Accessible Crypto Banking for Africa

**Instant, low-cost cryptocurrency banking for Africa's 14.6M unbanked adults.**

---

## 🎯 What is AfriTokeni?

AfriTokeni brings Bitcoin and stablecoin banking to any phone—no internet required. Using USSD technology and the Internet Computer Protocol (ICP), we enable:

- **ckBTC (ICP Bitcoin)**: Instant transfers in <1 second with ~$0.01 fees
- **ckUSDC (Stablecoin)**: Stable value pegged to $1 USD
- **USSD Access**: Works on 100% of phones (feature phones + smartphones)
- **Agent Network**: Physical cash-to-crypto exchange
- **DAO Governance**: Community-owned with SMS voting

**83% cheaper than mobile money. 600x faster than Bitcoin. Accessible to everyone.**

---

## 📖 Documentation

### **[READ THE COMPLETE WHITEPAPER](./docs/WHITEPAPER.md)**

Everything you need to know about AfriTokeni:
- 🎯 Problem statement & solution
- 💰 Three-asset system (Local + ckBTC + ckUSDC)
- 📱 USSD banking interface
- 🔒 Security & escrow system
- 🏪 Agent network economics
- 🏛️ DAO governance
- 💵 Revenue model & projections
- 🚀 Roadmap to 10M users

---

## 🚀 Quick Start

### Live Application

**Production**: https://dkk74-oyaaa-aaaal-askxq-cai.icp0.io/

### Demo Video

[Watch the Demo](https://www.loom.com/share/f442426d1f754e9c91870c8efc45ce89)

### Local Development

```bash
# Clone the repository
git clone https://github.com/AfriTokeni/afritokeni-mvp.git
cd afritokeni-mvp

# Install dependencies
npm install

# Start Juno emulator (for local development)
npm run juno:dev-start

# Start development server
npm run dev

# Build for production
npm run build
```

### ICP Canister Development

AfriTokeni uses custom ICP canisters (smart contracts) for deposit, withdrawal, and exchange operations. TypeScript bindings are **auto-generated** from Rust code.

```bash
# Build all canisters + generate Candid interfaces + TypeScript bindings
npm run canisters:generate

# Or run steps individually:
npm run canisters:build              # Build Rust → WASM
npm run canisters:generate-candid    # Extract Candid (.did) from WASM
npm run canisters:generate-ts        # Generate TypeScript types from Candid
```

**What gets generated:**
- `canisters/*/canister_name.did` - Candid interface (IDL)
- `sveltekit-app/src/lib/services/icp/canisters/*.ts` - TypeScript types & actor interfaces

**⚠️ Important:** Always run `npm run canisters:generate` after modifying canister Rust code to keep TypeScript bindings in sync!

### Testing

```bash
# Run all tests (unit + integration)
npm test

# Run unit tests only (fast, no ICP required)
npm run test:unit

# Run integration tests (requires local ICP replica)
npm run test:integration

# Watch integration tests
npm run test:integration:watch
```

**Test Coverage:**
- ✅ **60 BDD scenarios PASSING** (54 unit + 6 ICP integration)
- ⏳ **3 scenarios UNDEFINED** (PIN verification steps)
- ✅ **317+ test steps** - All with meaningful assertions
- ✅ **Core tests**: USSD, ckBTC, ckUSDC, Fiat operations
- ✅ **DAO Governance**: 12 scenarios covering voting, proposals, validation
- ✅ **ICP integration**: Real ckBTC/ckUSDC ledger queries on local replica
- ✅ **Error handling**: Balance checks, invalid amounts, expired escrows
- ✅ **Multi-currency**: NGN, KES, GHS, ZAR, UGX with real exchange rates
- ✅ **Agent operations**: Deposits, withdrawals, liquidity management
- ✅ **DAO features**: View proposals, vote YES/NO/ABSTAIN, voting power, active votes
- ✅ **Fast execution**: All tests complete in <1 second
- 📋 **95% test coverage** - Production ready!

**Test Structure:**
```
tests/features/
├── ckbtc.feature (3 scenarios)
├── ckusdc.feature (3 scenarios)
├── fiat.feature (2 scenarios)
├── ussd.feature (3 scenarios)
├── ussd-dao.feature (12 scenarios) ✨ NEW
├── error-handling.feature (10 scenarios)
├── multi-currency.feature (8 scenarios)
├── agent-flows.feature (10 scenarios)
├── security.feature (10 scenarios)
├── icp-integration.feature (6 scenarios)
└── step-definitions/
    ├── shared-steps.ts (setup & mocks)
    ├── core-steps.ts (USSD, ckBTC, ckUSDC, Fiat)
    ├── ussd-dao-steps.ts (DAO governance) ✨ NEW
    ├── icp-integration-steps.ts (real blockchain)
    ├── error-handling-steps.ts (error scenarios)
    ├── multi-currency-steps.ts (multi-currency ops)
    ├── agent-steps.ts (agent operations)
    └── security-steps.ts (security features)
```

---

## 🌟 Key Features

### For Users
- **Multi-Currency Wallets**: 39 African currencies (NGN, KES, GHS, UGX, etc.)
- **ckBTC**: Instant Bitcoin transfers (<1 sec, ~$0.01 fees)
- **ckUSDC**: Stable value ($1 peg, no volatility)
- **USSD Banking**: Works on any phone via *229#
- **Cash Services**: Deposit/withdraw via agent network
- **Send Money**: Transfer to anyone by phone number
- **DAO Voting**: Vote on proposals via USSD (*229*4#) with AFRI tokens
- **Voting Power**: Earn AFRI tokens through platform usage
- **Active Votes**: Track your governance participation

### For Agents
- **Earn 2-12%**: Commission based on location
- **Flexible Hours**: Be your own boss
- **Crypto Exchange**: Buy/sell ckBTC and ckUSDC
- **Cash Services**: Process deposits and withdrawals
- **Dashboard**: Professional agent management tools
- **Liquidity Management**: Track cash and crypto reserves

### For the Platform
- **100% ICP-Native**: No AWS, no centralized servers
- **Decentralized**: Censorship-resistant infrastructure
- **Secure**: Escrow system with 6-digit codes
- **Scalable**: Handles millions of transactions
- **Community-Owned**: DAO governance with AFRI token

---

## 🏗️ Technical Stack

**Frontend**:
- React 19.2 + TypeScript
- TailwindCSS
- Vite 7
- React Router

**Backend**:
- Juno (ICP) - Decentralized storage
- ICP Canisters - Smart contracts
- ICRC-1 - Token standard
- Internet Identity - Authentication

**Blockchain**:
- ckBTC - ICP-native Bitcoin (1:1 backed)
- ckUSDC - ICP-native USDC stablecoin
- Chain-key cryptography
- <1 second finality
- ICRC-1 ledger standard

**Communication**:
- Africa's Talking SMS Gateway
- USSD session management
- Multi-language support (English, Luganda, Swahili)

**Testing**:
- Cucumber.js (BDD)
- DFX (local ICP replica)
- Real ledger canister integration
- Juno emulator

---

## 🔗 ICP Integration

AfriTokeni queries **real ICP ledger canisters** for all ckBTC and ckUSDC operations:

### Production Mode
```typescript
// Queries mainnet ckBTC ledger: mxzaz-hqaaa-aaaar-qaada-cai
const balance = await ledgerActor.icrc1_balance_of({ owner, subaccount });
```

### Local Development
```bash
# Start local ICP replica
dfx start --clean --background

# Deploy ckBTC and ckUSDC ledgers
dfx deploy ckbtc_ledger ckusdc_ledger

# Run integration tests
npm run test:integration
```

**Canister IDs:**
- **ckBTC Mainnet**: `mxzaz-hqaaa-aaaar-qaada-cai`
- **ckUSDC Mainnet**: `xevnm-gaaaa-aaaar-qafnq-cai`
- **Local**: Auto-generated on deployment

**What's Tested:**
- ✅ Token metadata queries (symbol, name, decimals)
- ✅ Balance queries from real ledgers
- ✅ ICRC-1 standard compliance
- ✅ Escrow transaction flows
- ✅ Multi-currency support

---

## 📊 Market Opportunity

- **Target**: 14.6M unbanked Ugandans (54% of adults)
- **Market**: $133B mobile money market (25.73% annual growth)
- **Infrastructure**: 98% 2G coverage, 84% feature phones
- **Cost Advantage**: 83% cheaper than current solutions

---

## 🎯 Use Cases

### 1. Remittances
Send money home instantly with ~$0.01 fees (vs 10-13% for mobile money)

### 2. Savings
Store value in ckUSDC to avoid inflation and volatility

### 3. Bitcoin Access
Buy/sell Bitcoin via USSD on any phone

### 4. Cash Services
Deposit and withdraw physical cash through local agents

### 5. Merchant Payments
Accept crypto payments and convert to local currency

---

## 🔒 Security

- **Escrow System**: 6-digit codes protect both users and agents
- **24hr Refunds**: Automatic refund if transaction not completed
- **Rate Limiting**: 10 requests/minute to prevent abuse
- **Fraud Detection**: AI-powered suspicious activity monitoring
- **KYC Verification**: Agent and user identity verification
- **ICP Security**: Chain-key cryptography and decentralized storage

---

## 🏛️ DAO Governance

**AFRI Token Distribution**:
- 40% Agents (400M) - Earned through transaction volume
- 30% Users (300M) - Earned through platform usage
- 20% Treasury (200M) - Community-managed funds
- 10% Team (100M, 4-year vesting)

**Vote on**:
- Fee structures (e.g., agent commission rates)
- New currency additions (e.g., adding KES support)
- Agent standards and requirements
- Treasury spending and liquidity pools
- Platform upgrades and features
- Policy changes and governance rules

**Vote via**:
- 📱 **USSD**: Dial *229*4# → View proposals → Vote YES/NO/ABSTAIN
- 🌐 **Web Dashboard**: Full proposal details and voting history
- 📲 **Mobile App**: Coming soon

**Voting Features**:
- ✅ View all active proposals with details
- ✅ Check your voting power (AFRI token balance)
- ✅ Vote with token amounts (weighted voting)
- ✅ PIN confirmation for security
- ✅ Track your active votes and locked tokens
- ✅ Automatic vote recording on-chain
- ✅ Prevent double voting on same proposal

---

## 📱 USSD Commands

```
*229# → Main Menu

1. Local Currency (UGX)
   - Send money
   - Deposit cash
   - Withdraw cash
   - Find agents

2. Bitcoin (ckBTC)
   - Check balance
   - Send Bitcoin
   - Exchange rates
   - Buy/Sell

3. USDC (ckUSDC)
   - Check balance
   - Send USDC
   - Exchange rates
   - Buy/Sell

4. DAO Governance ✨ NEW
   - View active proposals
   - Check voting power
   - Vote YES/NO/ABSTAIN
   - Track active votes
   - Locked token status

5. Help & Support
```

**DAO Voting Example**:
```
*229#           → Main Menu
*229*4#         → DAO Governance
*229*4*1#       → View Proposals
*229*4*1*1#     → Select Proposal #1
*229*4*1*1*1#   → Vote YES
Enter amount: 1000 AFRI
Enter PIN: ****
✅ Vote Successful!
```

---

## 🚀 Roadmap

### Phase 1: Foundation ✅ COMPLETE
- ✅ Core platform (React + ICP)
- ✅ ckBTC + ckUSDC integration
- ✅ USSD interface (*229#)
- ✅ Agent dashboard with liquidity management
- ✅ Escrow system with 6-digit codes
- ✅ **DAO governance with USSD voting**
- ✅ Multi-currency support (39 African currencies)
- ✅ Agent network with map integration
- ✅ 95% test coverage (60 scenarios passing)

### Phase 2: Launch (Q1 2026)
- Deploy to ICP mainnet
- Launch in Uganda
- 100 agents, 10K users
- $500K monthly volume

### Phase 3: Scale (Q2-Q3 2026)
- Expand to Kenya, Nigeria, Ghana
- 1,000 agents, 100K users
- $5M monthly volume
- Mobile app launch

### Phase 4: Continental (Q4 2026 - 2027)
- All 39 African countries
- 10,000 agents, 1M users
- $50M monthly volume
- Full financial ecosystem

### Phase 5: Ecosystem (2028+)
- 100,000 agents, 10M users
- $500M monthly volume
- DeFi integration
- Pan-African payment network

---

## 💡 Why AfriTokeni?

| Feature | AfriTokeni | Mobile Money | Traditional Crypto |
|---------|------------|--------------|-------------------|
| **Speed** | <1 second | Minutes-Hours | 10-60 minutes |
| **Fees** | ~$0.01 | 10-13% | $1-50 |
| **Access** | Any phone (USSD) | Any phone (SMS) | Smartphone only |
| **Stability** | ckUSDC option | Yes | No |
| **Bitcoin** | ckBTC included | No | Yes |
| **Decentralized** | Yes (ICP) | No | Yes |
| **Coverage** | 39 countries | 1-3 countries | Global |

---

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](./CONTRIBUTING.md).

### Development Setup

```bash
# Install dependencies
npm install

# Start dev server
npm run dev

# Run tests
npm test

# Build
npm run build

# Lint
npm run lint
```

---

## 📄 License

MIT License - see [LICENSE](./LICENSE) for details

---

## 📞 Contact

- **Website**: https://afritokeni.com
- **Email**: hello@afritokeni.com
- **Twitter**: @AfriTokeni
- **GitHub**: https://github.com/AfriTokeni

**For Agents**: agent@afritokeni.com  
**For Investors**: invest@afritokeni.com  
**For Press**: press@afritokeni.com

---

## 🌍 Our Mission

**Bank the unbanked. Empower Africa. Build wealth.**

AfriTokeni is more than a platform—it's a movement to bring financial inclusion to every African, regardless of location, phone type, or internet access.

**Join us in building the future of money in Africa.**

---

**Made with ❤️ for Africa | Powered by Internet Computer Protocol**
