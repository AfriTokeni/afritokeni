# USSD Menu Structure and Correct Test Paths

## Main Menu
```
Welcome to AfriTokeni!
1. Local Currency (UGX)
2. Bitcoin (ckBTC)
3. USDC (ckUSDC)
4. Swap Crypto
5. DAO Governance
6. Help
7. Language
```

## Local Currency Submenu (1)
```
Local Currency Menu (UGX)
Please select an option:
1. Send Money
2. Check Balance
3. Deposit
4. Withdraw
5. Transactions
6. Find Agent
```

### Correct Paths:
- **Send Money**: `1*1*{recipient_phone}*{amount}*{pin}`
- **Check Balance**: `1*2`
- **Deposit**: `1*3*{amount}*{agent}*{pin}`
- **Withdraw**: `1*4*{amount}*{agent}*{pin}`
- **Transactions**: `1*5`
- **Find Agent**: `1*6`

## Bitcoin Submenu (2)
```
Bitcoin Menu
Please select an option:
1. Check Balance
2. Bitcoin Rate
3. Buy Bitcoin
4. Sell Bitcoin
5. Send Bitcoin
```

### Correct Paths:
- **Check Balance**: `2*1`
- **Bitcoin Rate**: `2*2`
- **Buy Bitcoin**: `2*3*{amount}*{pin}` (amount in fiat)
- **Sell Bitcoin**: `2*4*{amount}*{pin}` (amount in BTC satoshis)
- **Send Bitcoin**: `2*5*{recipient_phone}*{amount}*{pin}` (amount in BTC satoshis)

## USDC Submenu (3)
```
USDC Menu
Please select an option:
1. Check Balance
2. USDC Rate
3. Buy USDC
4. Sell USDC
5. Send USDC
```

### Correct Paths:
- **Check Balance**: `3*1`
- **USDC Rate**: `3*2`
- **Buy USDC**: `3*3*{amount}*{pin}` (amount in fiat)
- **Sell USDC**: `3*4*{amount}*{pin}` (amount in USDC cents)
- **Send USDC**: `3*5*{recipient_phone}*{amount}*{pin}` (amount in USDC cents)

## Crypto Swap (4)
```
Swap Crypto
1. BTC to USDC
2. USDC to BTC
```

### Correct Paths:
- **BTC to USDC**: `4*1*{amount}*{pin}` (amount in BTC satoshis)
- **USDC to BTC**: `4*2*{amount}*{pin}` (amount in USDC cents)

## DAO Governance (5)
```
DAO Governance
1. View Proposals
2. Vote on Proposal
3. Create Proposal
```

### Correct Paths:
- **View Proposals**: `5*1`
- **Vote**: `5*2*{proposal_id}*{vote}*{pin}`
- **Create Proposal**: `5*3*{title}*{description}*{pin}`

## Common Mistakes in Tests

### ❌ WRONG:
```rust
// Missing submenu selection
let input = format!("1*{}*50000*1111", receiver_phone); // WRONG!
let input = format!("2*2*10000*1111"); // WRONG! (2*2 is Bitcoin Rate, not Buy)
let input = format!("1*10000*agent*1111"); // WRONG! (missing submenu)
```

### ✅ CORRECT:
```rust
// Include submenu selection
let input = format!("1*1*{}*50000*1111", receiver_phone); // Send Money
let input = format!("2*3*10000*1111"); // Buy Bitcoin
let input = format!("1*4*10000*agent*1111"); // Withdraw
```

## Test Path Formula

**Format**: `{main_menu}*{submenu}*{param1}*{param2}*...`

- **Main menu** (required): Which top-level menu (1-7)
- **Submenu** (required for most): Which submenu option (1-6)
- **Parameters** (flow-specific): Recipient, amount, PIN, etc.

## Examples by Test Type

### Send Money Tests
```rust
// Complete path: Main(1) -> Submenu(1) -> Recipient -> Amount -> PIN
let input = format!("1*1*{}*50000*1111", receiver_phone);
```

### Withdraw Tests
```rust
// Complete path: Main(1) -> Submenu(4) -> Amount -> Agent -> PIN
let input = format!("1*4*10000*agent_001*1111");
```

### Buy Bitcoin Tests
```rust
// Complete path: Main(2) -> Submenu(3) -> Amount -> PIN
let input = format!("2*3*100000*1111");
```

### Buy USDC Tests
```rust
// Complete path: Main(3) -> Submenu(3) -> Amount -> PIN
let input = format!("3*3*100000*1111");
```

### Crypto Swap Tests
```rust
// Complete path: Main(4) -> Submenu(1=BTC->USDC or 2=USDC->BTC) -> Amount -> PIN
let input = format!("4*1*1000000*1111"); // BTC to USDC
let input = format!("4*2*10000*1111");   // USDC to BTC
```

### Balance Check Tests
```rust
// No parameters needed, just menu navigation
let input = "1*2"; // Check fiat balance
let input = "2*1"; // Check Bitcoin balance
let input = "3*1"; // Check USDC balance
```
