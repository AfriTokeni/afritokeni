use crate::models::*;
use crate::DataCanisterState;
use ic_cdk::api::time;
use shared_types::audit;

/// Deposit fiat currency
pub fn deposit_fiat(
    state: &mut DataCanisterState,
    user_id: String,
    amount: u64,
    currency: FiatCurrency,
    description: Option<String>,
) -> Result<Transaction, String> {
    // Verify user exists
    if !state.users.contains_key(&user_id) {
        return Err("User not found".to_string());
    }
    
    let now = time() / 1_000_000_000;
    let balance_key = format!("{}:{}", user_id, currency.code());
    
    // Get or create balance
    let mut balance = state.fiat_balances.get(&balance_key).cloned().unwrap_or(FiatBalance {
        user_id: user_id.clone(),
        currency,
        balance: 0,
        updated_at: now,
    });
    
    // Add amount
    balance.balance = balance.balance.checked_add(amount)
        .ok_or("Balance overflow")?;
    balance.updated_at = now;
    
    // Store updated balance
    state.fiat_balances.insert(balance_key, balance);
    
    // Create transaction record
    let tx_id = format!("tx_{}", now);
    let transaction = Transaction {
        id: tx_id.clone(),
        transaction_type: TransactionType::DepositFiat,
        from_user: None,
        to_user: Some(user_id.clone()),
        amount,
        currency_type: CurrencyType::Fiat(currency),
        status: TransactionStatus::Completed,
        created_at: now,
        completed_at: Some(now),
        description,
    };
    
    state.transactions.insert(tx_id, transaction.clone());
    
    // Log audit using shared library
    audit::log_success(
        "deposit_fiat",
        Some(user_id),
        format!("Deposited {} {}", amount, currency.code())
    );
    
    Ok(transaction)
}

/// Transfer fiat between users
pub fn transfer_fiat(
    state: &mut DataCanisterState,
    from_user: String,
    to_user: String,
    amount: u64,
    currency: FiatCurrency,
    description: Option<String>,
) -> Result<Transaction, String> {
    // Verify users exist
    if !state.users.contains_key(&from_user) {
        return Err("Sender not found".to_string());
    }
    if !state.users.contains_key(&to_user) {
        return Err("Recipient not found".to_string());
    }
    
    let now = time() / 1_000_000_000;
    let from_balance_key = format!("{}:{}", from_user, currency.code());
    let to_balance_key = format!("{}:{}", to_user, currency.code());
    
    // Get sender balance
    let mut from_balance = state.fiat_balances.get(&from_balance_key)
        .ok_or("Sender has no balance in this currency")?
        .clone();
    
    // Check sufficient balance
    if from_balance.balance < amount {
        return Err(format!("Insufficient balance. Have: {}, Need: {}", 
                          from_balance.balance, amount));
    }
    
    // Get or create recipient balance
    let mut to_balance = state.fiat_balances.get(&to_balance_key).cloned().unwrap_or(FiatBalance {
        user_id: to_user.clone(),
        currency,
        balance: 0,
        updated_at: now,
    });
    
    // Perform transfer
    from_balance.balance = from_balance.balance.checked_sub(amount)
        .ok_or("Balance underflow")?;
    from_balance.updated_at = now;
    
    to_balance.balance = to_balance.balance.checked_add(amount)
        .ok_or("Balance overflow")?;
    to_balance.updated_at = now;
    
    // Store updated balances
    state.fiat_balances.insert(from_balance_key, from_balance);
    state.fiat_balances.insert(to_balance_key, to_balance);
    
    // Create transaction record
    let tx_id = format!("tx_{}", now);
    let transaction = Transaction {
        id: tx_id.clone(),
        transaction_type: TransactionType::TransferFiat,
        from_user: Some(from_user.clone()),
        to_user: Some(to_user.clone()),
        amount,
        currency_type: CurrencyType::Fiat(currency),
        status: TransactionStatus::Completed,
        created_at: now,
        completed_at: Some(now),
        description,
    };
    
    state.transactions.insert(tx_id, transaction.clone());
    
    // Log audit using shared library
    audit::log_success(
        "transfer_fiat",
        Some(from_user.clone()),
        format!("Transferred {} {} to {}", amount, currency.code(), to_user)
    );
    
    Ok(transaction)
}

/// Withdraw fiat (reduce balance)
pub fn withdraw_fiat(
    state: &mut DataCanisterState,
    user_id: String,
    amount: u64,
    currency: FiatCurrency,
    description: Option<String>,
) -> Result<Transaction, String> {
    // Verify user exists
    if !state.users.contains_key(&user_id) {
        return Err("User not found".to_string());
    }
    
    let now = time() / 1_000_000_000;
    let balance_key = format!("{}:{}", user_id, currency.code());
    
    // Get balance
    let mut balance = state.fiat_balances.get(&balance_key)
        .ok_or("No balance in this currency")?
        .clone();
    
    // Check sufficient balance
    if balance.balance < amount {
        return Err(format!("Insufficient balance. Have: {}, Need: {}", 
                          balance.balance, amount));
    }
    
    // Deduct amount
    balance.balance = balance.balance.checked_sub(amount)
        .ok_or("Balance underflow")?;
    balance.updated_at = now;
    
    // Store updated balance
    state.fiat_balances.insert(balance_key, balance);
    
    // Create transaction record
    let tx_id = format!("tx_{}", now);
    let transaction = Transaction {
        id: tx_id.clone(),
        transaction_type: TransactionType::WithdrawFiat,
        from_user: Some(user_id.clone()),
        to_user: None,
        amount,
        currency_type: CurrencyType::Fiat(currency),
        status: TransactionStatus::Completed,
        created_at: now,
        completed_at: Some(now),
        description,
    };
    
    state.transactions.insert(tx_id, transaction.clone());
    
    // Log audit using shared library
    audit::log_success(
        "withdraw_fiat",
        Some(user_id),
        format!("Withdrew {} {}", amount, currency.code())
    );
    
    Ok(transaction)
}

/// Update crypto balance (called after ckBTC/ckUSDC ledger operations)
pub fn update_crypto_balance(
    state: &mut DataCanisterState,
    user_id: String,
    ckbtc_delta: i64,
    ckusdc_delta: i64,
) -> Result<(), String> {
    // Verify user exists
    if !state.users.contains_key(&user_id) {
        return Err("User not found".to_string());
    }
    
    let now = time() / 1_000_000_000;
    
    // Get or create crypto balance
    let mut balance = state.crypto_balances.get(&user_id).cloned().unwrap_or(CryptoBalance {
        user_id: user_id.clone(),
        ckbtc: 0,
        ckusdc: 0,
        updated_at: now,
    });
    
    // Update ckBTC
    if ckbtc_delta != 0 {
        balance.ckbtc = if ckbtc_delta > 0 {
            balance.ckbtc.checked_add(ckbtc_delta as u64)
                .ok_or("ckBTC balance overflow")?
        } else {
            balance.ckbtc.checked_sub((-ckbtc_delta) as u64)
                .ok_or("Insufficient ckBTC balance")?
        };
    }
    
    // Update ckUSDC
    if ckusdc_delta != 0 {
        balance.ckusdc = if ckusdc_delta > 0 {
            balance.ckusdc.checked_add(ckusdc_delta as u64)
                .ok_or("ckUSDC balance overflow")?
        } else {
            balance.ckusdc.checked_sub((-ckusdc_delta) as u64)
                .ok_or("Insufficient ckUSDC balance")?
        };
    }
    
    balance.updated_at = now;
    
    // Store updated balance
    state.crypto_balances.insert(user_id.clone(), balance);
    
    // Log audit using shared library
    audit::log_success(
        "crypto_balance_updated",
        Some(user_id),
        format!("ckBTC: {:+}, ckUSDC: {:+}", ckbtc_delta, ckusdc_delta)
    );
    
    Ok(())
}
