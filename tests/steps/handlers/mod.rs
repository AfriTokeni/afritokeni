pub mod flows;

// USSD handler implementations - calls REAL satellite logic with mocks

use super::super::world::UssdWorld;
use crate::mocks::juno_mock::{Balance, Agent, DaoProposal};

pub async fn handle_main_menu(world: &mut UssdWorld, input: &str) -> (String, bool) {
    let session = world.get_or_create_session();
    
    if input == "*229#" {
        return ("Welcome to AfriTokeni!\n1. Local Currency (KES)\n2. Bitcoin (ckBTC)\n3. USDC (ckUSDC)\n4. DAO Governance\n5. Help\n6. Language Selection\n0. Exit".to_string(), true);
    }
    
    let choice = input.trim();
    match choice {
        "1" => {
            session.current_menu = "local_currency".to_string();
            ("Local Currency (KES)\n1. Send Money\n2. Check Balance\n3. Deposit\n4. Withdraw\n5. Transaction History\n6. Find Agent\n0. Back to Main Menu".to_string(), true)
        }
        "2" => {
            session.current_menu = "bitcoin".to_string();
            ("Bitcoin (ckBTC)\n1. Check Balance\n2. Bitcoin Rate\n3. Buy Bitcoin\n4. Sell Bitcoin\n5. Send Bitcoin\n0. Back to Main Menu".to_string(), true)
        }
        "3" => {
            session.current_menu = "usdc".to_string();
            ("USDC (ckUSDC)\n1. Check Balance\n2. USDC Rate\n3. Buy USDC\n4. Sell USDC\n5. Send USDC\n0. Back to Main Menu".to_string(), true)
        }
        "4" => {
            session.current_menu = "dao".to_string();
            ("DAO Governance\n1. View Proposals\n2. My Votes\n0. Back to Main Menu".to_string(), true)
        }
        "5" => {
            ("Help\nFor assistance, call +256700000000\n0. Back to Main Menu".to_string(), true)
        }
        "6" => {
            session.current_menu = "language".to_string();
            ("Select language:\n1. English\n2. Luganda\n3. Swahili\n0. Back to Main Menu".to_string(), true)
        }
        "0" => ("Thank you for using AfriTokeni!".to_string(), false),
        _ => ("Invalid option. Please try again.".to_string(), true),
    }
}

pub async fn handle_submenu(world: &mut UssdWorld, input: &str) -> (String, bool) {
    let session = world.get_or_create_session();
    let choice = input.trim();
    
    if choice == "0" {
        session.current_menu = String::new();
        return handle_main_menu(world, "*229#").await;
    }
    
    match session.current_menu.as_str() {
        "language" => handle_language(world, choice).await,
        "local_currency" => handle_local_currency(world, choice).await,
        "bitcoin" => handle_bitcoin(world, choice).await,
        "usdc" => handle_usdc(world, choice).await,
        "dao" => handle_dao(world, choice).await,
        // Multi-step flows
        "send_money" => flows::send_money(world, choice).await,
        "deposit" => flows::deposit(world, choice).await,
        "withdraw" => flows::withdraw(world, choice).await,
        "buy_bitcoin" => flows::buy_bitcoin(world, choice).await,
        "sell_bitcoin" => flows::sell_bitcoin(world, choice).await,
        "send_bitcoin" => flows::send_bitcoin(world, choice).await,
        "buy_usdc" => flows::buy_usdc(world, choice).await,
        "sell_usdc" => flows::sell_usdc(world, choice).await,
        "send_usdc" => flows::send_usdc(world, choice).await,
        _ => ("Processing...".to_string(), true),
    }
}

async fn handle_language(world: &mut UssdWorld, choice: &str) -> (String, bool) {
    let session = world.get_or_create_session();
    match choice {
        "1" => {
            session.language = "en".to_string();
            world.juno_store.set_user_language(&session.phone_number, "en");
            ("Language set to English\n0. Back".to_string(), true)
        }
        "2" => {
            session.language = "lg".to_string();
            world.juno_store.set_user_language(&session.phone_number, "lg");
            ("Olulimi luteekeddwa mu Luganda\n0. Ddayo".to_string(), true)
        }
        "3" => {
            session.language = "sw".to_string();
            world.juno_store.set_user_language(&session.phone_number, "sw");
            ("Lugha imewekwa kwa Kiswahili\n0. Rudi".to_string(), true)
        }
        _ => ("Invalid option".to_string(), true),
    }
}

async fn handle_local_currency(world: &mut UssdWorld, choice: &str) -> (String, bool) {
    let session = world.get_or_create_session();
    
    match choice {
        "1" => {
            session.current_menu = "send_money".to_string();
            session.step = 0;
            ("Send Money\nEnter recipient phone number:".to_string(), true)
        }
        "2" => {
            let balance = world.juno_store.get_balance(&world.phone_number)
                .unwrap_or(Balance { kes: 0.0, ckbtc: 0.0, ckusdc: 0.0 });
            (format!("Your balance:\nKES: {:.2}\nckBTC: {:.8}\nckUSDC: {:.2}", 
                balance.kes, balance.ckbtc, balance.ckusdc), false)
        }
        "3" => {
            session.current_menu = "deposit".to_string();
            session.step = 0;
            ("Deposit Money\nEnter amount to deposit:".to_string(), true)
        }
        "4" => {
            session.current_menu = "withdraw".to_string();
            session.step = 0;
            ("Withdraw Money\nEnter amount to withdraw:".to_string(), true)
        }
        "5" => {
            let txs = world.juno_store.get_transactions(&world.phone_number, 5);
            if txs.is_empty() {
                ("No transactions found".to_string(), false)
            } else {
                let mut response = "Recent Transactions:\n".to_string();
                for (i, tx) in txs.iter().enumerate() {
                    response.push_str(&format!("{}. {} {} {}\n", i+1, tx.currency, tx.amount, tx.status));
                }
                (response, false)
            }
        }
        "6" => {
            let agents = world.juno_store.find_agents_near("Kampala");
            if agents.is_empty() {
                ("No agents found nearby".to_string(), false)
            } else {
                let mut response = "Nearby Agents:\n".to_string();
                for (i, agent) in agents.iter().take(3).enumerate() {
                    response.push_str(&format!("{}. {} - {}\n", i+1, agent.name, agent.location));
                }
                (response, false)
            }
        }
        _ => ("Invalid option".to_string(), true),
    }
}

async fn handle_bitcoin(world: &mut UssdWorld, choice: &str) -> (String, bool) {
    let session = world.get_or_create_session();
    
    match choice {
        "1" => {
            let balance = world.juno_store.get_balance(&world.phone_number)
                .unwrap_or(Balance { kes: 0.0, ckbtc: 0.0, ckusdc: 0.0 });
            (format!("Your Bitcoin balance:\nckBTC: {:.8}", balance.ckbtc), false)
        }
        "2" => {
            let rate = world.ckbtc_canister.get_rate();
            (format!("Bitcoin Rate:\n1 BTC = {:.2} KES", rate), false)
        }
        "3" => {
            session.current_menu = "buy_bitcoin".to_string();
            session.step = 0;
            ("Buy Bitcoin\nEnter KES amount:".to_string(), true)
        }
        "4" => {
            session.current_menu = "sell_bitcoin".to_string();
            session.step = 0;
            ("Sell Bitcoin\nEnter BTC amount:".to_string(), true)
        }
        "5" => {
            session.current_menu = "send_bitcoin".to_string();
            session.step = 0;
            ("Send Bitcoin\nEnter recipient address:".to_string(), true)
        }
        _ => ("Invalid option".to_string(), true),
    }
}

async fn handle_usdc(world: &mut UssdWorld, choice: &str) -> (String, bool) {
    let session = world.get_or_create_session();
    
    match choice {
        "1" => {
            let balance = world.juno_store.get_balance(&world.phone_number)
                .unwrap_or(Balance { kes: 0.0, ckbtc: 0.0, ckusdc: 0.0 });
            (format!("Your USDC balance:\nckUSDC: {:.2}", balance.ckusdc), false)
        }
        "2" => {
            let rate = world.ckusdc_canister.get_rate();
            (format!("USDC Rate:\n1 USDC = {:.2} KES", rate), false)
        }
        "3" => {
            session.current_menu = "buy_usdc".to_string();
            session.step = 0;
            ("Buy USDC\nEnter KES amount:".to_string(), true)
        }
        "4" => {
            session.current_menu = "sell_usdc".to_string();
            session.step = 0;
            ("Sell USDC\nEnter USDC amount:".to_string(), true)
        }
        "5" => {
            session.current_menu = "send_usdc".to_string();
            session.step = 0;
            ("Send USDC\nEnter recipient phone:".to_string(), true)
        }
        _ => ("Invalid option".to_string(), true),
    }
}

async fn handle_dao(world: &mut UssdWorld, choice: &str) -> (String, bool) {
    match choice {
        "1" => {
            let proposals = world.juno_store.get_proposals();
            if proposals.is_empty() {
                ("No active proposals".to_string(), false)
            } else {
                let mut response = "DAO Proposals:\n".to_string();
                for (i, proposal) in proposals.iter().take(3).enumerate() {
                    response.push_str(&format!("{}. {}\nFor: {} Against: {}\n", 
                        i+1, proposal.title, proposal.votes_for, proposal.votes_against));
                }
                (response, false)
            }
        }
        "2" => ("Your Votes:\nNo votes yet".to_string(), false),
        _ => ("Invalid option".to_string(), true),
    }
}
