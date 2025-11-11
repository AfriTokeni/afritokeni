# USSD Session Management - Implementation Guide

## 🎯 Current Status

### ✅ What's Already Built
1. **USSD Canister Structure** - Complete
   - HTTP request routing (`http_request`, `http_request_update`)
   - Session management (create, save, delete)
   - Business Logic Canister integration setup
   - Multi-language support (en, lg, sw)

2. **Session Model** - Complete
   ```rust
   pub struct UssdSession {
       pub session_id: String,
       pub phone_number: String,
       pub current_menu: String,
       pub step: u32,
       pub language: String,
       pub last_activity: u64,
       pub data: HashMap<String, String>, // Flow data
   }
   ```

3. **Session Features** - Complete
   - 5-minute timeout
   - Activity tracking
   - Data storage (recipient, amount, etc.)
   - Language preference

### 📂 Current Structure
```
canisters/ussd_canister/src/
├── api/                    # HTTP routing
│   └── http.rs
├── core/                   # Core logic
│   ├── routing.rs         # Menu routing (27KB - LARGE!)
│   └── session.rs         # Session management ✅
├── flows/                  # Transaction flows (21 items)
├── services/              # External services
│   └── business_logic.rs  # Business Logic Canister client
└── utils/                 # Utilities
```

---

## 🚀 What Needs to Be Done

### Priority 1: Session Persistence & Recovery

#### Issue: Sessions are in-memory only
**Problem:** Sessions stored in `thread_local!` are lost on canister upgrade.

**Solution:** Implement stable storage for sessions

```rust
// Add to session.rs
use ic_stable_structures::{StableBTreeMap, memory_manager::MemoryManager};

// Replace thread_local with stable storage
static SESSIONS: RefCell<StableBTreeMap<String, UssdSession, Memory>> = ...;
```

**Files to modify:**
- `src/core/session.rs` - Add stable storage
- `src/lib.rs` - Add pre/post upgrade hooks

**Tests needed:**
- Session survives canister upgrade
- Session timeout after 5 minutes
- Session data persists across calls

---

### Priority 2: Business Logic Integration

#### Issue: USSD flows need to call Business Logic Canister

**Current:** Flows are defined but not connected to backend
**Needed:** Inter-canister calls for all operations

**Operations to integrate:**
1. **User Registration**
   ```rust
   // In flows/registration.rs
   let result = business_logic::register_user(phone, pin, currency).await?;
   ```

2. **Money Transfer**
   ```rust
   // In flows/send_money.rs
   let result = business_logic::transfer_money(
       sender_phone,
       recipient_phone,
       amount,
       currency,
       pin
   ).await?;
   ```

3. **Deposit/Withdrawal**
   ```rust
   // In flows/deposit.rs
   let result = business_logic::create_deposit_request(
       user_phone,
       agent_id,
       amount
   ).await?;
   ```

4. **Crypto Operations**
   ```rust
   // In flows/buy_crypto.rs
   let result = business_logic::buy_crypto(
       phone,
       crypto_type,
       amount_ugx,
       pin
   ).await?;
   ```

**Files to modify:**
- `src/services/business_logic.rs` - Add all client methods
- `src/flows/*.rs` - Replace mock logic with real calls

**Tests needed:**
- Each flow calls correct Business Logic method
- Error handling for failed calls
- Session state updates correctly

---

### Priority 3: Session State Management

#### Issue: Complex flows need multi-step state tracking

**Example: Send Money Flow**
```
Step 1: Enter recipient phone
Step 2: Enter amount
Step 3: Confirm details
Step 4: Enter PIN
Step 5: Show result
```

**Current:** Basic step tracking exists
**Needed:** Robust state machine

**Implementation:**
```rust
// Add to session.rs
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FlowState {
    SendMoney { recipient: Option<String>, amount: Option<u64> },
    BuyCrypto { crypto_type: Option<String>, amount: Option<u64> },
    Deposit { agent: Option<String>, amount: Option<u64> },
    // ... other flows
}

impl UssdSession {
    pub fn set_flow_state(&mut self, state: FlowState) {
        // Store flow state
    }
    
    pub fn get_flow_state(&self) -> Option<FlowState> {
        // Retrieve flow state
    }
}
```

**Tests needed:**
- Flow state persists across steps
- Invalid input doesn't corrupt state
- User can cancel mid-flow

---

### Priority 4: Error Handling & User Experience

#### Issue: Need graceful error handling

**Scenarios to handle:**
1. Business Logic Canister unavailable
2. Invalid user input
3. Insufficient balance
4. Network timeout
5. Session expired

**Implementation:**
```rust
// Add to utils/errors.rs
pub enum UssdError {
    SessionExpired,
    InvalidInput(String),
    InsufficientBalance,
    CanisterUnavailable,
    NetworkTimeout,
}

impl UssdError {
    pub fn to_user_message(&self, language: &str) -> String {
        match self {
            UssdError::SessionExpired => {
                translate("session_expired", language)
            }
            UssdError::InvalidInput(msg) => {
                format!("{}: {}", translate("invalid_input", language), msg)
            }
            // ... other errors
        }
    }
}
```

**Tests needed:**
- Each error type shows correct message
- Errors in user's language
- User can retry after error

---

### Priority 5: Session Cleanup & Monitoring

#### Issue: Need to clean up expired sessions

**Implementation:**
```rust
// Add to lib.rs
#[update]
async fn cleanup_expired_sessions() -> u32 {
    let mut cleaned = 0;
    SESSIONS.with(|sessions| {
        let mut sessions_map = sessions.borrow_mut();
        sessions_map.retain(|_, session| {
            if session.is_expired() {
                cleaned += 1;
                false
            } else {
                true
            }
        });
    });
    cleaned
}

// Add heartbeat for automatic cleanup
#[heartbeat]
async fn heartbeat() {
    // Run cleanup every 10 minutes
    cleanup_expired_sessions().await;
}
```

**Tests needed:**
- Expired sessions are removed
- Active sessions are preserved
- Cleanup doesn't affect performance

---

## 📋 Implementation Checklist

### Phase 1: Core Session Management (Week 1)
- [ ] Implement stable storage for sessions
- [ ] Add pre/post upgrade hooks
- [ ] Test session persistence across upgrades
- [ ] Implement session cleanup
- [ ] Add session monitoring

### Phase 2: Business Logic Integration (Week 2)
- [ ] Add all Business Logic client methods
- [ ] Integrate registration flow
- [ ] Integrate money transfer flow
- [ ] Integrate deposit/withdrawal flows
- [ ] Integrate crypto operations

### Phase 3: State Management (Week 3)
- [ ] Implement FlowState enum
- [ ] Add state machine for each flow
- [ ] Test multi-step flows
- [ ] Add flow cancellation
- [ ] Test state persistence

### Phase 4: Error Handling (Week 4)
- [ ] Create UssdError enum
- [ ] Add error-to-message translation
- [ ] Implement retry logic
- [ ] Add timeout handling
- [ ] Test all error scenarios

### Phase 5: Testing & Polish (Week 5)
- [ ] Integration tests for all flows
- [ ] Load testing (concurrent sessions)
- [ ] Security audit (PIN handling)
- [ ] Performance optimization
- [ ] Documentation

---

## 🧪 Testing Strategy

### Unit Tests
```rust
// Test session creation
#[test]
fn test_create_session() { ... }

// Test session expiry
#[test]
fn test_session_expires_after_timeout() { ... }

// Test data storage
#[test]
fn test_session_data_storage() { ... }
```

### Integration Tests
```rust
// Test full flow
#[tokio::test]
async fn test_send_money_flow() {
    // Step 1: Enter recipient
    let (response, continues) = test_ussd_direct(
        "session1",
        "+256700123456",
        "1" // Send money
    ).await;
    assert!(continues);
    
    // Step 2: Enter amount
    let (response, continues) = test_ussd_direct(
        "session1",
        "+256700123456",
        "+256700999888" // Recipient
    ).await;
    assert!(continues);
    
    // ... continue flow
}
```

### Load Tests
```rust
// Test concurrent sessions
#[tokio::test]
async fn test_100_concurrent_sessions() {
    let mut handles = vec![];
    for i in 0..100 {
        let handle = tokio::spawn(async move {
            test_ussd_direct(
                &format!("session{}", i),
                &format!("+25670012{:04}", i),
                "1"
            ).await
        });
        handles.push(handle);
    }
    
    for handle in handles {
        assert!(handle.await.is_ok());
    }
}
```

---

## 🎯 Success Criteria

### Functional
- ✅ Sessions persist across canister upgrades
- ✅ All flows call Business Logic Canister
- ✅ Multi-step flows maintain state correctly
- ✅ Errors show user-friendly messages
- ✅ Sessions clean up automatically

### Performance
- ✅ Handle 100+ concurrent sessions
- ✅ Response time < 2 seconds
- ✅ Session lookup < 100ms
- ✅ Memory usage < 100MB for 1000 sessions

### Security
- ✅ PINs never logged or stored
- ✅ Session IDs are unique
- ✅ Expired sessions are inaccessible
- ✅ Rate limiting prevents abuse

---

## 📚 Resources

### Current Documentation
- `USSD_CANISTER_SUMMARY.md` - Architecture overview
- `USSD_JUNO_INTEGRATION.md` - Integration guide
- `canisters/ussd_canister/README.md` - Basic setup

### Business Logic API
- See `COMMISSION_INTEGRATION_IMPLEMENTATION.md` for available methods
- All 102 integration tests show correct usage patterns

### IC Documentation
- [Stable Structures](https://docs.rs/ic-stable-structures/)
- [Inter-Canister Calls](https://internetcomputer.org/docs/current/developer-docs/backend/rust/inter-canister-calls)
- [Canister Upgrades](https://internetcomputer.org/docs/current/developer-docs/backend/rust/upgrading)

---

## 🚀 Next Steps

1. **Start with Priority 1** - Session persistence is critical
2. **Then Priority 2** - Connect to Business Logic
3. **Iterate on Priorities 3-5** - Polish and test

**Estimated Timeline:** 5 weeks for complete implementation

**Ready to start?** Let's begin with session persistence! 🎊
