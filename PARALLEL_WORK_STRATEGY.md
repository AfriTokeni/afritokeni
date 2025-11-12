# Parallel Work Strategy - Multiple Agents

## Overview

Instead of one agent doing all 4 canisters sequentially, we can have **4 agents working in parallel**, each responsible for one canister.

## Why This Works

### ✅ Advantages
1. **4x faster** - Complete in 1 day instead of 4
2. **Clear boundaries** - Each canister is independent
3. **No merge conflicts** - Each agent works in separate directories
4. **Parallel testing** - All canisters tested simultaneously

### ⚠️ Challenges
1. **Coordination needed** - Agents must follow same patterns
2. **Shared types** - Need to coordinate on shared_types updates
3. **Integration testing** - Need all 4 canisters to test together

## Agent Assignment

### Agent 1: user_canister (PRIORITY 1)
**Why first:** Other canisters depend on this

**Branch:** `feature/user-canister`

**Responsibilities:**
- Create user_canister structure
- Implement all user endpoints
- Write user tests
- Update shared_types with UserProfile types

**Deliverables:**
- `canisters/user_canister/` (complete)
- Tests passing
- WASM < 500KB
- Candid interface documented

**Estimated time:** 6-8 hours

---

### Agent 2: wallet_canister (PRIORITY 2)
**Why second:** Depends on user_canister

**Branch:** `feature/wallet-canister`

**Responsibilities:**
- Create wallet_canister structure
- Implement P2P transfer logic
- Include fraud detection
- Write wallet tests

**Dependencies:**
- Wait for Agent 1 to finish user_canister
- Use user_canister for PIN verification

**Deliverables:**
- `canisters/wallet_canister/` (complete)
- Tests passing
- WASM < 800KB
- Fraud detection working

**Estimated time:** 8-10 hours

---

### Agent 3: agent_canister (PRIORITY 2)
**Why second:** Depends on user_canister

**Branch:** `feature/agent-canister`

**Responsibilities:**
- Create agent_canister structure
- Merge deposit_canister code
- Merge withdrawal_canister code
- Unify agent management
- Write agent tests

**Dependencies:**
- Wait for Agent 1 to finish user_canister
- Use user_canister for PIN verification

**Deliverables:**
- `canisters/agent_canister/` (complete)
- Tests passing
- WASM < 900KB
- Unified agent balance tracking

**Estimated time:** 10-12 hours

---

### Agent 4: crypto_canister (PRIORITY 3)
**Why third:** Depends on user_canister AND wallet_canister

**Branch:** `feature/crypto-canister`

**Responsibilities:**
- Create crypto_canister structure
- Implement buy/sell logic
- Merge exchange_canister
- Add escrow operations
- Write crypto tests

**Dependencies:**
- Wait for Agent 1 (user_canister)
- Wait for Agent 2 (wallet_canister) - for fiat balance checks
- Use both for crypto operations

**Deliverables:**
- `canisters/crypto_canister/` (complete)
- Tests passing
- WASM < 1.2M
- DEX integration working

**Estimated time:** 10-12 hours

---

## Workflow

### Phase 1: Sequential Start (Hours 0-8)
```
Hour 0-8:  Agent 1 → user_canister
           (Others wait)
```

**Why sequential:** user_canister is a dependency for all others

**Agent 1 deliverables:**
- ✅ user_canister compiles
- ✅ All user tests pass
- ✅ Candid interface finalized
- ✅ Merged to main branch

### Phase 2: Parallel Work (Hours 8-20)
```
Hour 8-18:  Agent 2 → wallet_canister  (parallel)
            Agent 3 → agent_canister   (parallel)

Hour 18-20: Agent 4 → crypto_canister  (waits for wallet)
```

**Why parallel:** wallet and agent don't depend on each other

**Coordination:**
- Agent 2 & 3 work simultaneously
- Agent 4 starts when Agent 2 finishes
- All use completed user_canister

### Phase 3: Integration (Hours 20-24)
```
Hour 20-24: All Agents → Integration testing
            - Update USSD canister
            - Update Web frontend
            - Run full test suite
            - Deploy locally
```

**Coordination:**
- One agent updates USSD
- One agent updates Web
- One agent runs integration tests
- One agent handles deployment

---

## Branch Strategy

### Main Branches
```
main
└── feature/bdd-tests (current)
    ├── feature/user-canister (Agent 1)
    ├── feature/wallet-canister (Agent 2)
    ├── feature/agent-canister (Agent 3)
    └── feature/crypto-canister (Agent 4)
```

### Merge Order
1. **Agent 1** → Merge user_canister to feature/bdd-tests
2. **Agent 2 & 3** → Rebase on feature/bdd-tests, then merge
3. **Agent 4** → Rebase on feature/bdd-tests (with wallet), then merge
4. **All** → Merge feature/bdd-tests to main

---

## Coordination Points

### Shared Files (Need Coordination)

**1. shared_types/src/lib.rs**
- Agent 1 adds: UserProfile, ProfileUpdates
- Agent 2 adds: TransferResult, FraudAlert
- Agent 3 adds: AgentBalance, MonthlySettlement
- Agent 4 adds: SwapResult, EscrowResult

**Solution:** Each agent adds their types in their branch, merge conflicts resolved during integration

**2. dfx.json**
- All agents need to add their canister definition
- Already done in current branch ✅

**3. Cargo.toml (workspace)**
- Each agent adds their canister to workspace members
- Merge conflicts expected, easy to resolve

---

## Communication Protocol

### Agent Check-ins

**Agent 1 (user_canister):**
```
✅ Hour 2: Structure created
✅ Hour 4: Endpoints implemented
✅ Hour 6: Tests passing
✅ Hour 8: MERGED - Others can start
```

**Agent 2 (wallet_canister):**
```
✅ Hour 10: Structure created
✅ Hour 14: Fraud detection working
✅ Hour 18: Tests passing
✅ Hour 18: MERGED
```

**Agent 3 (agent_canister):**
```
✅ Hour 10: Structure created
✅ Hour 14: Deposit/withdrawal merged
✅ Hour 18: Unified agent management
✅ Hour 18: MERGED
```

**Agent 4 (crypto_canister):**
```
✅ Hour 18: Structure created (after wallet ready)
✅ Hour 20: Buy/sell working
✅ Hour 22: Swaps & escrow done
✅ Hour 22: MERGED
```

---

## Prompt Templates

### Agent 1 Prompt (user_canister)
```
You are Agent 1 responsible for creating the user_canister.

CONTEXT:
- We're splitting business_logic_canister (1.9M) into 4 domain canisters
- You're creating the FIRST canister that others depend on
- Branch: feature/user-canister
- Base: feature/bdd-tests

YOUR TASK:
1. Create canisters/user_canister/ structure
2. Implement all user management endpoints (9 total)
3. Move code from business_logic_canister/src/logic/user_logic.rs
4. Write integration tests
5. Ensure WASM < 500KB

DELIVERABLES:
- user_canister compiles
- All tests pass
- Candid interface documented
- Merged to feature/bdd-tests

REFERENCE DOCS:
- CANISTER_MIGRATION_PLAN.md (Phase 1.1)
- NEXT_SESSION_DETAILED_PROMPT.md (Step 1)

START: Create the directory structure and Cargo.toml
```

### Agent 2 Prompt (wallet_canister)
```
You are Agent 2 responsible for creating the wallet_canister.

CONTEXT:
- We're splitting business_logic_canister into 4 domain canisters
- user_canister is COMPLETE (Agent 1 finished)
- You handle P2P fiat transfers and fraud detection
- Branch: feature/wallet-canister
- Base: feature/bdd-tests (with user_canister merged)

YOUR TASK:
1. Create canisters/wallet_canister/ structure
2. Implement P2P transfer logic
3. Include fraud detection
4. Call user_canister for PIN verification
5. Write integration tests

DEPENDENCIES:
- user_canister (for verify_pin)
- data_canister (for balances)

DELIVERABLES:
- wallet_canister compiles
- All tests pass
- WASM < 800KB
- Fraud detection working

REFERENCE DOCS:
- CANISTER_MIGRATION_PLAN.md (Phase 1.2)

START: Create the directory structure and Cargo.toml
```

### Agent 3 Prompt (agent_canister)
```
You are Agent 3 responsible for creating the agent_canister.

CONTEXT:
- We're splitting business_logic_canister into 4 domain canisters
- user_canister is COMPLETE (Agent 1 finished)
- You merge deposit_canister + withdrawal_canister
- Branch: feature/agent-canister
- Base: feature/bdd-tests (with user_canister merged)

YOUR TASK:
1. Create canisters/agent_canister/ structure
2. Merge ENTIRE deposit_canister code
3. Merge ENTIRE withdrawal_canister code
4. Create UNIFIED agent management (no duplicates!)
5. Write integration tests

KEY FEATURE:
- Single agent balance tracking (merge from both canisters)
- Unified commission calculation
- Unified settlement generation

DELIVERABLES:
- agent_canister compiles
- All tests pass
- WASM < 900KB
- No duplicate agent code

REFERENCE DOCS:
- CANISTER_MIGRATION_PLAN.md (Phase 1.3)

START: Create the directory structure and Cargo.toml
```

### Agent 4 Prompt (crypto_canister)
```
You are Agent 4 responsible for creating the crypto_canister.

CONTEXT:
- We're splitting business_logic_canister into 4 domain canisters
- user_canister is COMPLETE (Agent 1 finished)
- wallet_canister is COMPLETE (Agent 2 finished)
- You merge exchange_canister and handle all crypto ops
- Branch: feature/crypto-canister
- Base: feature/bdd-tests (with user + wallet merged)

YOUR TASK:
1. Create canisters/crypto_canister/ structure
2. Implement buy/sell logic
3. Merge ENTIRE exchange_canister for swaps
4. Add escrow operations
5. Write integration tests

DEPENDENCIES:
- user_canister (for verify_pin)
- wallet_canister (for fiat balance checks)
- data_canister (for crypto balances)
- Sonic DEX (external)

DELIVERABLES:
- crypto_canister compiles
- All tests pass
- WASM < 1.2M
- DEX integration working

REFERENCE DOCS:
- CANISTER_MIGRATION_PLAN.md (Phase 1.4)

START: Create the directory structure and Cargo.toml
```

---

## Integration Agent Prompt
```
You are the Integration Agent responsible for connecting all 4 canisters.

CONTEXT:
- All 4 domain canisters are COMPLETE
- user_canister ✅
- wallet_canister ✅
- agent_canister ✅
- crypto_canister ✅

YOUR TASK:
1. Update USSD canister to call new canisters
2. Update Web frontend services
3. Update all integration tests
4. Deploy locally and configure
5. Run full test suite

DELIVERABLES:
- USSD tests pass (312 tests)
- Web frontend works
- All canisters deployed
- Critical flows verified

REFERENCE DOCS:
- CANISTER_MIGRATION_PLAN.md (Phase 5 & 6)

START: Update USSD canister service clients
```

---

## Timeline Comparison

### Sequential (1 Agent)
```
Day 1: user_canister
Day 2: wallet_canister + agent_canister
Day 3: crypto_canister
Day 4: Integration
Total: 4 days
```

### Parallel (4 Agents)
```
Hour 0-8:   Agent 1 → user_canister
Hour 8-18:  Agent 2 → wallet_canister  (parallel)
            Agent 3 → agent_canister   (parallel)
Hour 18-22: Agent 4 → crypto_canister
Hour 22-24: All → Integration
Total: 1 day (24 hours)
```

**4x faster!** 🚀

---

## Risk Mitigation

### Risk 1: Merge Conflicts
**Solution:** 
- Each agent works in separate directory
- Only shared_types has conflicts
- Easy to resolve (just add types)

### Risk 2: Integration Issues
**Solution:**
- Reserve 4 hours for integration
- All agents available for debugging
- Rollback plan in place

### Risk 3: Agent Coordination
**Solution:**
- Clear check-in schedule
- Shared Slack/Discord channel
- One person as coordinator

### Risk 4: Dependency Delays
**Solution:**
- Agent 1 is critical path - prioritize
- Agents 2 & 3 can start together
- Agent 4 can prepare while waiting

---

## Recommendation

### For Speed: Parallel (4 Agents)
- ✅ 4x faster
- ✅ Clear boundaries
- ⚠️ Needs coordination

### For Safety: Sequential (1 Agent)
- ✅ No coordination needed
- ✅ Easier to debug
- ⚠️ 4x slower

**My recommendation:** **Parallel with 4 agents** if you have the resources. The time savings are massive and the coordination overhead is minimal since each canister is independent.

---

## Next Steps

1. **Decide:** Parallel or Sequential?
2. **If Parallel:** Assign agents to canisters
3. **Create branches:** One per canister
4. **Start Agent 1:** user_canister (critical path)
5. **Coordinate:** Use check-ins and shared docs

Good luck! 🚀
