/// Pure business logic for USSD flows
/// No I/O, no async, no IC calls - fully testable

pub mod validation;
pub mod menu_logic;
pub mod send_money_logic;
pub mod crypto_logic;
pub mod agent_logic;
pub mod swap_logic;
