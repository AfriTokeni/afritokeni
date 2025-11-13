/// Pure presentation logic for menu navigation
/// No I/O, no async, no IC calls - fully testable

/// Main menu options
#[derive(Debug, Clone, PartialEq)]
pub enum MainMenuOption {
    LocalCurrency,
    Bitcoin,
    USDC,
    CryptoSwap,
    DAO,
    Language,
    Invalid,
}

impl MainMenuOption {
    pub fn from_choice(choice: &str) -> Self {
        match choice {
            "1" => MainMenuOption::LocalCurrency,
            "2" => MainMenuOption::Bitcoin,
            "3" => MainMenuOption::USDC,
            "4" => MainMenuOption::CryptoSwap,
            "5" => MainMenuOption::DAO,
            "6" | "9" => MainMenuOption::Language,
            _ => MainMenuOption::Invalid,
        }
    }
}

/// Local currency submenu options
#[derive(Debug, Clone, PartialEq)]
pub enum LocalCurrencyOption {
    SendMoney,
    CheckBalance,
    Deposit,
    Withdraw,
    Transactions,
    FindAgent,
    Back,
    Invalid,
}

impl LocalCurrencyOption {
    pub fn from_choice(choice: &str) -> Self {
        match choice {
            "1" => LocalCurrencyOption::SendMoney,
            "2" => LocalCurrencyOption::CheckBalance,
            "3" => LocalCurrencyOption::Deposit,
            "4" => LocalCurrencyOption::Withdraw,
            "5" => LocalCurrencyOption::Transactions,
            "6" => LocalCurrencyOption::FindAgent,
            "0" => LocalCurrencyOption::Back,
            _ => LocalCurrencyOption::Invalid,
        }
    }
}

/// Bitcoin submenu options
#[derive(Debug, Clone, PartialEq)]
pub enum BitcoinOption {
    CheckBalance,
    Rate,
    Buy,
    Sell,
    Send,
    Back,
    Invalid,
}

impl BitcoinOption {
    pub fn from_choice(choice: &str) -> Self {
        match choice {
            "1" => BitcoinOption::CheckBalance,
            "2" => BitcoinOption::Rate,
            "3" => BitcoinOption::Buy,
            "4" => BitcoinOption::Sell,
            "5" => BitcoinOption::Send,
            "0" => BitcoinOption::Back,
            _ => BitcoinOption::Invalid,
        }
    }
}

/// USDC submenu options
#[derive(Debug, Clone, PartialEq)]
pub enum USDCOption {
    CheckBalance,
    Rate,
    Buy,
    Sell,
    Send,
    Back,
    Invalid,
}

impl USDCOption {
    pub fn from_choice(choice: &str) -> Self {
        match choice {
            "1" => USDCOption::CheckBalance,
            "2" => USDCOption::Rate,
            "3" => USDCOption::Buy,
            "4" => USDCOption::Sell,
            "5" => USDCOption::Send,
            "0" => USDCOption::Back,
            _ => USDCOption::Invalid,
        }
    }
}

/// Language options
#[derive(Debug, Clone, PartialEq)]
pub enum LanguageOption {
    English,
    Luganda,
    Swahili,
    Back,
    Invalid,
}

impl LanguageOption {
    pub fn from_choice(choice: &str) -> Self {
        match choice {
            "1" => LanguageOption::English,
            "2" => LanguageOption::Luganda,
            "3" => LanguageOption::Swahili,
            "0" => LanguageOption::Back,
            _ => LanguageOption::Invalid,
        }
    }

    pub fn to_code(&self) -> &str {
        match self {
            LanguageOption::English => "en",
            LanguageOption::Luganda => "lg",
            LanguageOption::Swahili => "sw",
            _ => "en",
        }
    }
}

/// Parse USSD input to extract menu choices
pub fn parse_ussd_input(text: &str) -> Vec<String> {
    text.split('*')
        .map(|s| s.to_string())
        .collect()
}

/// Get main menu choice from USSD input
pub fn get_main_menu_choice(text: &str) -> Option<String> {
    let parts = parse_ussd_input(text);
    parts.first().cloned()
}

/// Get submenu choice from USSD input
pub fn get_submenu_choice(text: &str) -> Option<String> {
    let parts = parse_ussd_input(text);
    parts.get(1).cloned()
}

/// Check if user wants to go back (pressed 0)
pub fn is_back_action(choice: &str) -> bool {
    choice == "0"
}

/// Check if user wants main menu (pressed 9)
pub fn is_main_menu_action(choice: &str) -> bool {
    choice == "9"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_menu_option_from_choice() {
        assert_eq!(MainMenuOption::from_choice("1"), MainMenuOption::LocalCurrency);
        assert_eq!(MainMenuOption::from_choice("2"), MainMenuOption::Bitcoin);
        assert_eq!(MainMenuOption::from_choice("3"), MainMenuOption::USDC);
        assert_eq!(MainMenuOption::from_choice("4"), MainMenuOption::CryptoSwap);
        assert_eq!(MainMenuOption::from_choice("5"), MainMenuOption::DAO);
        assert_eq!(MainMenuOption::from_choice("6"), MainMenuOption::Language);
        assert_eq!(MainMenuOption::from_choice("9"), MainMenuOption::Language);
        assert_eq!(MainMenuOption::from_choice("99"), MainMenuOption::Invalid);
    }

    #[test]
    fn test_local_currency_option_from_choice() {
        assert_eq!(LocalCurrencyOption::from_choice("1"), LocalCurrencyOption::SendMoney);
        assert_eq!(LocalCurrencyOption::from_choice("2"), LocalCurrencyOption::CheckBalance);
        assert_eq!(LocalCurrencyOption::from_choice("0"), LocalCurrencyOption::Back);
        assert_eq!(LocalCurrencyOption::from_choice("99"), LocalCurrencyOption::Invalid);
    }

    #[test]
    fn test_bitcoin_option_from_choice() {
        assert_eq!(BitcoinOption::from_choice("1"), BitcoinOption::CheckBalance);
        assert_eq!(BitcoinOption::from_choice("3"), BitcoinOption::Buy);
        assert_eq!(BitcoinOption::from_choice("0"), BitcoinOption::Back);
    }

    #[test]
    fn test_usdc_option_from_choice() {
        assert_eq!(USDCOption::from_choice("1"), USDCOption::CheckBalance);
        assert_eq!(USDCOption::from_choice("3"), USDCOption::Buy);
        assert_eq!(USDCOption::from_choice("0"), USDCOption::Back);
    }

    #[test]
    fn test_language_option_from_choice() {
        assert_eq!(LanguageOption::from_choice("1"), LanguageOption::English);
        assert_eq!(LanguageOption::from_choice("2"), LanguageOption::Luganda);
        assert_eq!(LanguageOption::from_choice("3"), LanguageOption::Swahili);
        assert_eq!(LanguageOption::from_choice("0"), LanguageOption::Back);
    }

    #[test]
    fn test_language_option_to_code() {
        assert_eq!(LanguageOption::English.to_code(), "en");
        assert_eq!(LanguageOption::Luganda.to_code(), "lg");
        assert_eq!(LanguageOption::Swahili.to_code(), "sw");
    }

    #[test]
    fn test_parse_ussd_input() {
        assert_eq!(parse_ussd_input("1*2*3"), vec!["1", "2", "3"]);
        assert_eq!(parse_ussd_input("1"), vec!["1"]);
        assert_eq!(parse_ussd_input(""), vec![""]);
    }

    #[test]
    fn test_get_main_menu_choice() {
        assert_eq!(get_main_menu_choice("1*2*3"), Some("1".to_string()));
        assert_eq!(get_main_menu_choice("5"), Some("5".to_string()));
    }

    #[test]
    fn test_get_submenu_choice() {
        assert_eq!(get_submenu_choice("1*2*3"), Some("2".to_string()));
        assert_eq!(get_submenu_choice("1"), None);
    }

    #[test]
    fn test_is_back_action() {
        assert!(is_back_action("0"));
        assert!(!is_back_action("1"));
        assert!(!is_back_action("9"));
    }

    #[test]
    fn test_is_main_menu_action() {
        assert!(is_main_menu_action("9"));
        assert!(!is_main_menu_action("0"));
        assert!(!is_main_menu_action("1"));
    }
}
