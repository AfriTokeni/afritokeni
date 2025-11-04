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
# Run all tests
npm run test:all

# Run unit tests (USSD logic, mocks)
npm run test:unit

# Run integration tests (real blockchain - ckBTC/ckUSDC ledgers)
npm run test:integration
```

**Test Coverage:**
- ✅ **58 Rust canister tests** (deposit, withdrawal, exchange)
- ✅ **162 USSD unit test scenarios** (Bitcoin, USDC, local currency, DAO)
- ✅ **19 ICP integration scenarios** (real ckBTC/ckUSDC ledger on local replica)
- 📊 **Total: 239 tests - 100% passing**

**What's Tested:**
- ✅ **USSD Flows**: Menu navigation, Bitcoin/USDC buy/sell/send, local currency ops
- ✅ **DAO Governance**: Proposals, voting, token locking, voting power
- ✅ **ICP Integration**: Real ckBTC/ckUSDC ledger queries on local replica
- ✅ **Revenue Model**: Platform fees (0.5%), agent commissions (2-12%), on-chain tracking
- ✅ **Multi-currency**: 39 African currencies with real exchange rates
- ✅ **Error Handling**: Balance checks, invalid amounts, PIN verification
- ✅ **Security**: Escrow codes, transaction expiry, fraud prevention

**Test Structure:**
```
tests/
├── unit/              # USSD service unit tests (15 features, 162 scenarios)
│   ├── ussd-bitcoin.feature
│   ├── ussd-usdc.feature
│   ├── ussd-dao.feature
│   ├── ussd-handlers.feature
│   └── ... (11 more)
│
├── integration/       # ICP canister integration (2 features, 19 scenarios)
│   ├── integration-ckbtc.feature
│   └── integration-ckusdc.feature
│
├── e2e/              # End-to-end tests (5 features, 36 scenarios)
│   ├── e2e-deposit-flow.feature
│   ├── e2e-withdrawal-flow.feature
│   ├── e2e-exchange-flow.feature
│   ├── e2e-api-routes.feature
│   └── e2e-revenue-tracking.feature
│
├── helpers/          # Shared test utilities
└── mocks/            # Mock implementations
```

**Rust Canister Tests:**
```bash
# Test all canisters
cargo test --release

# Results:
# ✅ Deposit canister: 20 tests
# ✅ Withdrawal canister: 19 tests
# ✅ Exchange canister: 19 tests
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
- SvelteKit 2.x + TypeScript
- Svelte 5 (Runes)
- TailwindCSS 4
- Vite 7

**Backend**:
- SvelteKit API Routes (+server.ts)
- Juno (ICP) - Decentralized storage
- ICP Canisters - Smart contracts (Rust)
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
- Cucumber.js (BDD) - 275 tests
- Cargo (Rust canister tests) - 58 tests
- DFX (local ICP replica)
- Real ledger canister integration

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

We welcome contributions! Please follow our coding standards below.

### Development Setup

```bash
# Install dependencies
pnpm install

# Install Gitleaks (for pre-commit secret detection)
brew install gitleaks

# Start dev server
pnpm run dev

# Run tests
pnpm run test:unit
pnpm run test:integration

# Build
pnpm run build

# Lint
pnpm run lint
```

**Pre-Commit Hooks:** Automatically run on every commit:
- 🔐 Secret detection (Gitleaks)
- 📝 Code formatting (Prettier)
- 🔎 Linting (ESLint)
- 🔧 Type checking (svelte-check)

**CI Pipeline:** Full security scans on every PR:
- CodeQL (GitHub security scanner)
- Semgrep (AI-powered bug detection)
- Trivy (dependency vulnerabilities)
- pnpm audit (npm vulnerabilities)

### Coding Standards

#### 🔥 CRITICAL RULES - NEVER VIOLATE

**1. NO HARDCODED DATA**
```typescript
// ❌ WRONG - Hardcoded fallbacks
const userData = {
  firstName: data?.firstName || '',
  balance: data?.balance || 0
};

// ✅ CORRECT - Use exact data or show error
if (!doc) {
  console.error('❌ DATA ERROR:', error);
  toast.show('error', 'Data not found');
  return;
}
const userData = doc.data;
```

**2. NO localStorage FOR BUSINESS DATA**
```typescript
// ❌ WRONG - Business data in localStorage
localStorage.setItem('kycStatus', 'verified');

// ✅ CORRECT - Business data in Juno
await setDoc({
  collection: 'users',
  doc: { key: principalId, data: userData }
});

// ✅ OK - UI preferences only
localStorage.setItem('theme', 'dark');
```

**3. REUSE COMPONENTS**
```typescript
// ❌ WRONG - Duplicating logic
<input type="file" onchange={handleUpload} />
// ... 50 lines of upload logic

// ✅ CORRECT - Reuse existing
import KYCModal from '$lib/components/shared/KYCModal.svelte';
<KYCModal onSubmit={handleKYCSubmit} />
```

**4. SINGLE RESPONSIBILITY**
```typescript
// ❌ WRONG - God component
function Dashboard() {
  // 500 lines doing everything
}

// ✅ CORRECT - Separated concerns
<BalanceCard balance={balance} />
<TransactionHistory transactions={txs} />
<KYCBanner kycStatus={status} />
```

**5. CONSISTENT DATA FLOW**
```
Juno DB → Component → UI
NO fallbacks, NO localStorage for business data
```

#### Checklist Before Committing
- [ ] No hardcoded fallback values (`||`, `??`)
- [ ] No localStorage for business data
- [ ] Checked if component already exists
- [ ] Each component has single responsibility
- [ ] Error handling with console.error + toast
- [ ] TypeScript interfaces for props
- [ ] Data flows from Juno → Component → UI

---

## 🚀 Deployment

### PR Previews (Vercel)
- Automatic unique preview URL per PR
- Auto-comments on PR with preview link
- Uses development Juno satellite: `atbka-rp777-77775-aaaaq-cai`
- Free tier: Unlimited previews

**Setup (2 min):**
1. Sign up at [vercel.com](https://vercel.com) with GitHub
2. Import `afritokeni-mvp` repo
3. Add environment variables:
   - `AT_USERNAME`, `AT_API_KEY`, `AT_SHORT_CODE` (all environments)
   - `VITE_JUNO_SATELLITE_ID=atbka-rp777-77775-aaaaq-cai` (preview only)
   - `VITE_JUNO_SATELLITE_ID=dkk74-oyaaa-aaaal-askxq-cai` (production only)

### Production (Juno/ICP)
- Deploys on merge to `main`
- URL: https://dkk74-oyaaa-aaaal-askxq-cai.icp0.io
- Decentralized blockchain hosting
- Uses production Juno satellite

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
