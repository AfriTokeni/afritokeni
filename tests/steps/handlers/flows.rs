// Multi-step transaction flows

use super::super::world::UssdWorld;
use crate::mocks::juno_mock::{Balance, Transaction};

pub async fn send_money(world: &mut UssdWorld, input: &str) -> (String, bool) {
    let step = world.get_or_create_session().step;
    
    match step {
        0 => {
            let session = world.get_or_create_session();
            session.data.insert("recipient".to_string(), input.to_string());
            session.step = 1;
            ("Enter amount:".to_string(), true)
        }
        1 => {
            let amount: f64 = input.parse().unwrap_or(0.0);
            if amount <= 0.0 {
                return ("Invalid amount".to_string(), false);
            }
            
            let recipient = world.get_or_create_session().data.get("recipient").unwrap().clone();
            let phone = world.phone_number.clone();
            let balance = world.juno_store.get_balance(&phone)
                .unwrap_or(Balance { kes: 0.0, ckbtc: 0.0, ckusdc: 0.0 });
            
            if balance.kes < amount {
                return ("Insufficient balance".to_string(), false);
            }
            
            world.juno_store.update_balance(&phone, -amount, 0.0, 0.0).ok();
            world.juno_store.update_balance(&recipient, amount, 0.0, 0.0).ok();
            
            use crate::mocks::juno_mock::Transaction;
            world.juno_store.add_transaction(Transaction {
                id: format!("tx_{}", uuid::Uuid::new_v4()),
                from: phone.clone(),
                to: recipient.clone(),
                amount,
                currency: "KES".to_string(),
                timestamp: 0,
                status: "completed".to_string(),
            });
            
            let session = world.get_or_create_session();
            session.current_menu = String::new();
            session.step = 0;
            (format!("Success! Sent {} KES to {}", amount, recipient), false)
        }
        _ => ("Error".to_string(), false),
    }
}

pub async fn deposit(world: &mut UssdWorld, input: &str) -> (String, bool) {
    let amount: f64 = input.parse().unwrap_or(0.0);
    if amount <= 0.0 {
        return ("Invalid amount".to_string(), false);
    }
    let phone = world.phone_number.clone();
    world.juno_store.update_balance(&phone, amount, 0.0, 0.0).ok();
    world.get_or_create_session().current_menu = String::new();
    (format!("Deposit successful! {} KES added to your account", amount), false)
}

pub async fn withdraw(world: &mut UssdWorld, input: &str) -> (String, bool) {
    let amount: f64 = input.parse().unwrap_or(0.0);
    if amount <= 0.0 {
        return ("Invalid amount".to_string(), false);
    }
    
    let phone = world.phone_number.clone();
    let balance = world.juno_store.get_balance(&phone)
        .unwrap_or(Balance { kes: 0.0, ckbtc: 0.0, ckusdc: 0.0 });
    
    if balance.kes < amount {
        return ("Insufficient balance".to_string(), false);
    }
    
    world.juno_store.update_balance(&phone, -amount, 0.0, 0.0).ok();
    world.get_or_create_session().current_menu = String::new();
    (format!("Withdrawal successful! {} KES withdrawn", amount), false)
}

pub async fn buy_bitcoin(world: &mut UssdWorld, input: &str) -> (String, bool) {
    let kes_amount: f64 = input.parse().unwrap_or(0.0);
    if kes_amount <= 0.0 {
        return ("Invalid amount".to_string(), false);
    }
    
    let phone = world.phone_number.clone();
    let balance = world.juno_store.get_balance(&phone)
        .unwrap_or(Balance { kes: 0.0, ckbtc: 0.0, ckusdc: 0.0 });
    
    if balance.kes < kes_amount {
        return ("Insufficient KES balance".to_string(), false);
    }
    
    let rate = world.ckbtc_canister.get_rate();
    let btc_amount = kes_amount / rate;
    
    world.juno_store.update_balance(&phone, -kes_amount, btc_amount, 0.0).ok();
    world.get_or_create_session().current_menu = String::new();
    (format!("Success! Bought {:.8} BTC for {} KES", btc_amount, kes_amount), false)
}

pub async fn sell_bitcoin(world: &mut UssdWorld, input: &str) -> (String, bool) {
    let btc_amount: f64 = input.parse().unwrap_or(0.0);
    if btc_amount <= 0.0 {
        return ("Invalid amount".to_string(), false);
    }
    
    let phone = world.phone_number.clone();
    let balance = world.juno_store.get_balance(&phone)
        .unwrap_or(Balance { kes: 0.0, ckbtc: 0.0, ckusdc: 0.0 });
    
    if balance.ckbtc < btc_amount {
        return ("Insufficient Bitcoin balance".to_string(), false);
    }
    
    let rate = world.ckbtc_canister.get_rate();
    let kes_amount = btc_amount * rate;
    
    world.juno_store.update_balance(&phone, kes_amount, -btc_amount, 0.0).ok();
    world.get_or_create_session().current_menu = String::new();
    (format!("Success! Sold {:.8} BTC for {:.2} KES", btc_amount, kes_amount), false)
}

pub async fn send_bitcoin(world: &mut UssdWorld, input: &str) -> (String, bool) {
    let step = world.get_or_create_session().step;
    
    match step {
        0 => {
            let session = world.get_or_create_session();
            session.data.insert("btc_recipient".to_string(), input.to_string());
            session.step = 1;
            ("Enter BTC amount:".to_string(), true)
        }
        1 => {
            let amount: f64 = input.parse().unwrap_or(0.0);
            if amount <= 0.0 {
                return ("Invalid amount".to_string(), false);
            }
            
            let phone = world.phone_number.clone();
            let balance = world.juno_store.get_balance(&phone)
                .unwrap_or(Balance { kes: 0.0, ckbtc: 0.0, ckusdc: 0.0 });
            
            if balance.ckbtc < amount {
                return ("Insufficient Bitcoin balance".to_string(), false);
            }
            
            world.juno_store.update_balance(&phone, 0.0, -amount, 0.0).ok();
            let session = world.get_or_create_session();
            session.current_menu = String::new();
            session.step = 0;
            (format!("Success! Sent {:.8} BTC", amount), false)
        }
        _ => ("Error".to_string(), false),
    }
}

pub async fn buy_usdc(world: &mut UssdWorld, input: &str) -> (String, bool) {
    let kes_amount: f64 = input.parse().unwrap_or(0.0);
    if kes_amount <= 0.0 {
        return ("Invalid amount".to_string(), false);
    }
    
    let phone = world.phone_number.clone();
    let balance = world.juno_store.get_balance(&phone)
        .unwrap_or(Balance { kes: 0.0, ckbtc: 0.0, ckusdc: 0.0 });
    
    if balance.kes < kes_amount {
        return ("Insufficient KES balance".to_string(), false);
    }
    
    let rate = world.ckusdc_canister.get_rate();
    let usdc_amount = kes_amount / rate;
    
    world.juno_store.update_balance(&phone, -kes_amount, 0.0, usdc_amount).ok();
    world.get_or_create_session().current_menu = String::new();
    (format!("Success! Bought {:.2} USDC for {} KES", usdc_amount, kes_amount), false)
}

pub async fn sell_usdc(world: &mut UssdWorld, input: &str) -> (String, bool) {
    let usdc_amount: f64 = input.parse().unwrap_or(0.0);
    if usdc_amount <= 0.0 {
        return ("Invalid amount".to_string(), false);
    }
    
    let phone = world.phone_number.clone();
    let balance = world.juno_store.get_balance(&phone)
        .unwrap_or(Balance { kes: 0.0, ckbtc: 0.0, ckusdc: 0.0 });
    
    if balance.ckusdc < usdc_amount {
        return ("Insufficient USDC balance".to_string(), false);
    }
    
    let rate = world.ckusdc_canister.get_rate();
    let kes_amount = usdc_amount * rate;
    
    world.juno_store.update_balance(&phone, kes_amount, 0.0, -usdc_amount).ok();
    world.get_or_create_session().current_menu = String::new();
    (format!("Success! Sold {:.2} USDC for {:.2} KES", usdc_amount, kes_amount), false)
}

pub async fn send_usdc(world: &mut UssdWorld, input: &str) -> (String, bool) {
    let step = world.get_or_create_session().step;
    
    match step {
        0 => {
            let session = world.get_or_create_session();
            session.data.insert("usdc_recipient".to_string(), input.to_string());
            session.step = 1;
            ("Enter USDC amount:".to_string(), true)
        }
        1 => {
            let amount: f64 = input.parse().unwrap_or(0.0);
            if amount <= 0.0 {
                return ("Invalid amount".to_string(), false);
            }
            
            let phone = world.phone_number.clone();
            let balance = world.juno_store.get_balance(&phone)
                .unwrap_or(Balance { kes: 0.0, ckbtc: 0.0, ckusdc: 0.0 });
            
            if balance.ckusdc < amount {
                return ("Insufficient USDC balance".to_string(), false);
            }
            
            let recipient = world.get_or_create_session().data.get("usdc_recipient").unwrap().clone();
            world.juno_store.update_balance(&phone, 0.0, 0.0, -amount).ok();
            world.juno_store.update_balance(&recipient, 0.0, 0.0, amount).ok();
            
            let session = world.get_or_create_session();
            session.current_menu = String::new();
            session.step = 0;
            (format!("Success! Sent {:.2} USDC to {}", amount, recipient), false)
        }
        _ => ("Error".to_string(), false),
    }
}
