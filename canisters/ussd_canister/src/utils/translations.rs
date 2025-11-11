#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Luganda,
    Swahili,
}

impl Language {
    pub fn from_code(code: &str) -> Self {
        match code {
            "en" => Language::English,
            "lg" => Language::Luganda,
            "sw" => Language::Swahili,
            _ => Language::English,
        }
    }
    
    pub fn to_code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Luganda => "lg",
            Language::Swahili => "sw",
        }
    }
}

pub struct TranslationService;

impl TranslationService {
    pub fn translate(key: &str, lang: Language) -> &'static str {
        match (key, lang) {
            ("welcome", Language::English) => "Welcome to AfriTokeni!",
            ("welcome", Language::Luganda) => "Tukusanyukidde ku AfriTokeni!",
            ("welcome", Language::Swahili) => "Karibu AfriTokeni!",

            ("account_ready", Language::English) => "Your account is ready.",
            ("account_ready", Language::Luganda) => "Akawunti yo etegefu.",
            ("account_ready", Language::Swahili) => "Akaunti yako iko tayari.",

            ("local_currency", Language::English) => "Local Currency",
            ("local_currency", Language::Luganda) => "Ssente z'omu Uganda",
            ("local_currency", Language::Swahili) => "Sarafu ya Ndani",

            ("bitcoin", Language::English) => "Bitcoin",
            ("bitcoin", Language::Luganda) => "Bitcoin",
            ("bitcoin", Language::Swahili) => "Bitcoin",

            ("usdc", Language::English) => "USDC",
            ("usdc", Language::Luganda) => "USDC",
            ("usdc", Language::Swahili) => "USDC",

            ("dao_governance", Language::English) => "DAO Governance",
            ("dao_governance", Language::Luganda) => "Okufuga kwa DAO",
            ("dao_governance", Language::Swahili) => "Utawala wa DAO",

            ("balance", Language::English) => "Balance",
            ("balance", Language::Luganda) => "Ssente",
            ("balance", Language::Swahili) => "Salio",

            ("your_balance", Language::English) => "Your balance is",
            ("your_balance", Language::Luganda) => "Ssente zo",
            ("your_balance", Language::Swahili) => "Salio yako ni",

            ("send_money", Language::English) => "Send Money",
            ("send_money", Language::Luganda) => "Wereza Ssente",
            ("send_money", Language::Swahili) => "Tuma Pesa",

            ("withdraw", Language::English) => "Withdraw Cash",
            ("withdraw", Language::Luganda) => "Ggya Ssente",
            ("withdraw", Language::Swahili) => "Ondoa Pesa",

            ("transaction_success", Language::English) => "Transaction successful!",
            ("transaction_success", Language::Luganda) => "Ensimbi ziweereddwa!",
            ("transaction_success", Language::Swahili) => "Muamala umefanikiwa!",

            ("transaction_successful", Language::English) => "Transaction successful",
            ("transaction_successful", Language::Luganda) => "Ensimbi ziweereddwa",
            ("transaction_successful", Language::Swahili) => "Muamala umefanikiwa",

            ("transaction_failed", Language::English) => "Transaction failed",
            ("transaction_failed", Language::Luganda) => "Ensimbi teziweereddwa",
            ("transaction_failed", Language::Swahili) => "Muamala umeshindwa",

            ("confirm", Language::English) => "Confirm",
            ("confirm", Language::Luganda) => "Kakasa",
            ("confirm", Language::Swahili) => "Thibitisha",

            ("confirmation", Language::English) => "Confirmation",
            ("confirmation", Language::Luganda) => "Okukakasa",
            ("confirmation", Language::Swahili) => "Uthibitisho",

            ("cancel", Language::English) => "Cancel",
            ("cancel", Language::Luganda) => "Sazaamu",
            ("cancel", Language::Swahili) => "Ghairi",

            ("confirm_transaction", Language::English) => "Confirm transaction?",
            ("confirm_transaction", Language::Luganda) => "Kakasa okuweereza?",
            ("confirm_transaction", Language::Swahili) => "Thibitisha muamala?",

            ("amount", Language::English) => "Amount",
            ("amount", Language::Luganda) => "Omuwendo",
            ("amount", Language::Swahili) => "Kiasi",

            ("fee", Language::English) => "Fee",
            ("fee", Language::Luganda) => "Ssente z'okuweereza",
            ("fee", Language::Swahili) => "Ada",

            ("total", Language::English) => "Total",
            ("total", Language::Luganda) => "Omugatte",
            ("total", Language::Swahili) => "Jumla",

            ("bitcoin_balance", Language::English) => "Bitcoin balance",
            ("bitcoin_balance", Language::Luganda) => "Ssente za Bitcoin",
            ("bitcoin_balance", Language::Swahili) => "Salio la Bitcoin",

            ("bitcoin_rate", Language::English) => "Bitcoin rate",
            ("bitcoin_rate", Language::Luganda) => "Omuwendo gwa Bitcoin",
            ("bitcoin_rate", Language::Swahili) => "Bei ya Bitcoin",

            ("buy_bitcoin", Language::English) => "Buy Bitcoin",
            ("buy_bitcoin", Language::Luganda) => "Gula Bitcoin",
            ("buy_bitcoin", Language::Swahili) => "Nunua Bitcoin",

            ("sell_bitcoin", Language::English) => "Sell Bitcoin",
            ("sell_bitcoin", Language::Luganda) => "Tunda Bitcoin",
            ("sell_bitcoin", Language::Swahili) => "Uza Bitcoin",

            ("error", Language::English) => "Error",
            ("error", Language::Luganda) => "Kiremya",
            ("error", Language::Swahili) => "Kosa",

            ("invalid_option", Language::English) => "Invalid option. Please try again:",
            ("invalid_option", Language::Luganda) => "Ekiragiro si kituufu. Gezaako nate:",
            ("invalid_option", Language::Swahili) => "Chaguo si sahihi. Jaribu tena:",

            ("insufficient_balance", Language::English) => "Insufficient balance",
            ("insufficient_balance", Language::Luganda) => "Ssente tezimala",
            ("insufficient_balance", Language::Swahili) => "Salio haitoshi",

            ("invalid_amount", Language::English) => "Invalid amount",
            ("invalid_amount", Language::Luganda) => "Omuwendo si mutuufu",
            ("invalid_amount", Language::Swahili) => "Kiasi si sahihi",

            ("invalid_phone", Language::English) => "Invalid phone number",
            ("invalid_phone", Language::Luganda) => "Namba ya simu si ntuufu",
            ("invalid_phone", Language::Swahili) => "Nambari ya simu si sahihi",

            ("enter_recipient_phone", Language::English) => "Enter recipient phone number:",
            ("enter_recipient_phone", Language::Luganda) => "Yingiza namba ya simu y'omuntu:",
            ("enter_recipient_phone", Language::Swahili) => "Weka nambari ya simu ya mpokeaji:",

            ("phone_format_example", Language::English) => "(e.g. 256700123456)",
            ("phone_format_example", Language::Luganda) => "(okugeza: 256700123456)",
            ("phone_format_example", Language::Swahili) => "(mfano: 256700123456)",

            ("too_many_requests", Language::English) => "Too many requests. Please wait.",
            ("too_many_requests", Language::Luganda) => "Osabidde emirundi mingi. Linda.",
            ("too_many_requests", Language::Swahili) => "Maombi mengi sana. Tafadhali subiri.",

            ("wait_minute", Language::English) => "Wait 1 minute",
            ("wait_minute", Language::Luganda) => "Linda dakiika 1",
            ("wait_minute", Language::Swahili) => "Subiri dakika 1",

            ("daily_limit_reached", Language::English) => "Daily limit reached",
            ("daily_limit_reached", Language::Luganda) => "Omugendo gw'olunaku gutuuse",
            ("daily_limit_reached", Language::Swahili) => "Kikomo cha siku kimefikiwa",

            ("suspicious_activity", Language::English) => "Suspicious activity detected",
            ("suspicious_activity", Language::Luganda) => "Ebikolwa ebitali bya bulijjo bizuuliddwa",
            ("suspicious_activity", Language::Swahili) => "Shughuli ya kutilia shaka imegunduliwa",

            ("transaction_blocked", Language::English) => "Transaction blocked for security",
            ("transaction_blocked", Language::Luganda) => "Okuweereza kuziyiddwa olw'obukuumi",
            ("transaction_blocked", Language::Swahili) => "Muamala umezuiwa kwa usalama",

            ("verification_required", Language::English) => "Verification required",
            ("verification_required", Language::Luganda) => "Okukakasa kwetaagisa",
            ("verification_required", Language::Swahili) => "Uthibitisho unahitajika",

            ("enter_pin", Language::English) => "Enter PIN",
            ("enter_pin", Language::Luganda) => "Yingiza PIN",
            ("enter_pin", Language::Swahili) => "Weka PIN",

            ("wrong_pin", Language::English) => "Wrong PIN",
            ("wrong_pin", Language::Luganda) => "PIN si ntuufu",
            ("wrong_pin", Language::Swahili) => "PIN si sahihi",

            ("pin_set", Language::English) => "PIN set successfully",
            ("pin_set", Language::Luganda) => "PIN etegekeddwa bulungi",
            ("pin_set", Language::Swahili) => "PIN imewekwa kwa mafanikio",

            ("account_locked", Language::English) => "Account locked. Too many attempts.",
            ("account_locked", Language::Luganda) => "Akawunti eziyiddwa. Ogezezzaako emirundi mingi.",
            ("account_locked", Language::Swahili) => "Akaunti imefungwa. Majaribio mengi sana.",

            ("session_expired", Language::English) => "Session expired. Please dial *384*22948# again to start a new session.",
            ("session_expired", Language::Luganda) => "Obudde buweddeyo. Kuba *384*22948# okutandika omulimu omupya.",
            ("session_expired", Language::Swahili) => "Kipindi kimemalizika. Tafadhali piga *384*22948# tena kuanza kipindi kipya.",

            ("check_balance", Language::English) => "Check Balance",
            ("check_balance", Language::Luganda) => "Kebera Ssente",
            ("check_balance", Language::Swahili) => "Angalia Salio",

            ("transaction_history", Language::English) => "Transaction History",
            ("transaction_history", Language::Luganda) => "Ebyafaayo by'ensimbi",
            ("transaction_history", Language::Swahili) => "Historia ya Muamala",

            ("bitcoin_services", Language::English) => "Bitcoin Services",
            ("bitcoin_services", Language::Luganda) => "Empeereza za Bitcoin",
            ("bitcoin_services", Language::Swahili) => "Huduma za Bitcoin",

            ("help", Language::English) => "Help",
            ("help", Language::Luganda) => "Obuyambi",
            ("help", Language::Swahili) => "Msaada",

            ("help_commands", Language::English) => "Commands: BAL, SEND, WITHDRAW, BTC BAL, HELP",
            ("help_commands", Language::Luganda) => "Ebiragiro: BAL, SEND, WITHDRAW, BTC BAL, HELP",
            ("help_commands", Language::Swahili) => "Amri: BAL, SEND, WITHDRAW, BTC BAL, HELP",

            ("help_ussd", Language::English) => "Dial *22948# for menu",
            ("help_ussd", Language::Luganda) => "Kuba *22948# okufuna menu",
            ("help_ussd", Language::Swahili) => "Piga *22948# kwa menyu",

            ("language_selection", Language::English) => "Language Selection",
            ("language_selection", Language::Luganda) => "Olulimi",
            ("language_selection", Language::Swahili) => "Lugha",

            ("select_language", Language::English) => "Select language:",
            ("select_language", Language::Luganda) => "Londa olulimi:",
            ("select_language", Language::Swahili) => "Chagua lugha:",

            ("language_set", Language::English) => "Language set to English",
            ("language_set", Language::Luganda) => "Olulimi lutegekeddwa ku Luganda",
            ("language_set", Language::Swahili) => "Lugha imewekwa kwa Kiswahili",

            ("press_zero_back", Language::English) => "Press 0 to return to main menu",
            ("press_zero_back", Language::Luganda) => "Nyiga 0 okudda ku menu enkulu",
            ("press_zero_back", Language::Swahili) => "Bonyeza 0 kurudi kwa menyu kuu",

            ("back_or_menu", Language::English) => "0. Back | 9. Menu",
            ("back_or_menu", Language::Luganda) => "0. Ddayo | 9. Menu",
            ("back_or_menu", Language::Swahili) => "0. Rudi | 9. Menyu",

            ("english", Language::English) => "English",
            ("english", Language::Luganda) => "English",
            ("english", Language::Swahili) => "English",

            ("luganda", Language::English) => "Luganda",
            ("luganda", Language::Luganda) => "Luganda",
            ("luganda", Language::Swahili) => "Luganda",

            ("swahili", Language::English) => "Swahili",
            ("swahili", Language::Luganda) => "Swahili",
            ("swahili", Language::Swahili) => "Kiswahili",

            ("enter_amount", Language::English) => "Enter amount",
            ("enter_amount", Language::Luganda) => "Yingiza omuwendo",
            ("enter_amount", Language::Swahili) => "Weka kiasi",

            ("enter_pin_4digit", Language::English) => "Enter your 4-digit PIN",
            ("enter_pin_4digit", Language::Luganda) => "Yingiza PIN yo ey'ennamba 4",
            ("enter_pin_4digit", Language::Swahili) => "Weka PIN yako ya nambari 4",

            ("invalid_pin_format", Language::English) => "Invalid PIN format",
            ("invalid_pin_format", Language::Luganda) => "Enkola ya PIN si ntuufu",
            ("invalid_pin_format", Language::Swahili) => "Muundo wa PIN si sahihi",

            ("incorrect_pin", Language::English) => "Incorrect PIN",
            ("incorrect_pin", Language::Luganda) => "PIN si ntuufu",
            ("incorrect_pin", Language::Swahili) => "PIN si sahihi",

            ("please_select_option", Language::English) => "Please select an option:",
            ("please_select_option", Language::Luganda) => "Londa ekiragiro:",
            ("please_select_option", Language::Swahili) => "Tafadhali chagua chaguo:",

            ("back_to_main_menu", Language::English) => "Back to Main Menu",
            ("back_to_main_menu", Language::Luganda) => "Ddayo ku Menu Enkulu",
            ("back_to_main_menu", Language::Swahili) => "Rudi kwa Menyu Kuu",

            ("invalid_selection", Language::English) => "Invalid selection",
            ("invalid_selection", Language::Luganda) => "Ekiragiro si kituufu",
            ("invalid_selection", Language::Swahili) => "Chaguo si sahihi",

            ("user_not_found", Language::English) => "User not found. Please contact support.",
            ("user_not_found", Language::Luganda) => "Omukozesa tazuuliddwa. Tuukirire obuyambi.",
            ("user_not_found", Language::Swahili) => "Mtumiaji hajapatikana. Tafadhali wasiliana na msaada.",

            ("error_try_again", Language::English) => "Error. Please try again later.",
            ("error_try_again", Language::Luganda) => "Kiremya. Gezaako oluvannyuma.",
            ("error_try_again", Language::Swahili) => "Kosa. Tafadhali jaribu tena baadaye.",

            ("thank_you", Language::English) => "Thank you for using AfriTokeni!",
            ("thank_you", Language::Luganda) => "Webale okukozesa AfriTokeni!",
            ("thank_you", Language::Swahili) => "Asante kwa kutumia AfriTokeni!",

            ("dial_to_start_new_session", Language::English) => "\n\nDial *384*22948# to start a new session",
            ("dial_to_start_new_session", Language::Luganda) => "\n\nKuba *384*22948# okutandika omulundi omupya",
            ("dial_to_start_new_session", Language::Swahili) => "\n\nPiga *384*22948# kuanza kipindi kipya",

            ("transaction_cancelled", Language::English) => "Transaction cancelled",
            ("transaction_cancelled", Language::Luganda) => "Okuweereza kusaziddwamu",
            ("transaction_cancelled", Language::Swahili) => "Muamala umeghairiwa",

            ("minimum_amount", Language::English) => "Minimum",
            ("minimum_amount", Language::Luganda) => "Omuwendo omutono",
            ("minimum_amount", Language::Swahili) => "Kiwango cha chini",

            ("enter_pin_confirm", Language::English) => "Enter your PIN to confirm",
            ("enter_pin_confirm", Language::Luganda) => "Yingiza PIN yo okukakasa",
            ("enter_pin_confirm", Language::Swahili) => "Weka PIN yako kuthibitisha",

            ("no_agents_available", Language::English) => "No agents available at this time. Please try again later.",
            ("no_agents_available", Language::Luganda) => "Tewali ba-agent kati. Gezaako oluvannyuma.",
            ("no_agents_available", Language::Swahili) => "Hakuna mawakala kwa sasa. Tafadhali jaribu tena baadaye.",

            ("select_agent", Language::English) => "Select an agent",
            ("select_agent", Language::Luganda) => "Londa agent",
            ("select_agent", Language::Swahili) => "Chagua wakala",

            ("choose_amount_type", Language::English) => "Choose amount type",
            ("choose_amount_type", Language::Luganda) => "Londa ekika ky'omuwendo",
            ("choose_amount_type", Language::Swahili) => "Chagua aina ya kiasi",

            ("transaction_complete", Language::English) => "Transaction complete!",
            ("transaction_complete", Language::Luganda) => "Okuweereza kuwedde!",
            ("transaction_complete", Language::Swahili) => "Muamala umekamilika!",

            ("meet_agent", Language::English) => "Meet your selected agent to complete the transaction.",
            ("meet_agent", Language::Luganda) => "Sisinkana ne agent wo okumaliriza.",
            ("meet_agent", Language::Swahili) => "Kutana na wakala wako kukamilisha muamala.",

            ("receive_cash", Language::English) => "Meet your selected agent to receive cash.",
            ("receive_cash", Language::Luganda) => "Sisinkana ne agent wo okufuna ssente.",
            ("receive_cash", Language::Swahili) => "Kutana na wakala wako kupokea pesa.",

            ("send_bitcoin", Language::English) => "Send Bitcoin",
            ("send_bitcoin", Language::Luganda) => "Wereza Bitcoin",
            ("send_bitcoin", Language::Swahili) => "Tuma Bitcoin",

            ("bitcoin_menu_title", Language::English) => "Bitcoin (ckBTC)",
            ("bitcoin_menu_title", Language::Luganda) => "Bitcoin (ckBTC)",
            ("bitcoin_menu_title", Language::Swahili) => "Bitcoin (ckBTC)",

            ("buy_btc_enter_amount", Language::English) => "Buy Bitcoin",
            ("buy_btc_enter_amount", Language::Luganda) => "Gula Bitcoin",
            ("buy_btc_enter_amount", Language::Swahili) => "Nunua Bitcoin",

            ("enter_amount_to_spend", Language::English) => "Enter amount to spend",
            ("enter_amount_to_spend", Language::Luganda) => "Yingiza omuwendo ogw'okusaasaanya",
            ("enter_amount_to_spend", Language::Swahili) => "Weka kiasi cha kutumia",

            ("minimum_purchase", Language::English) => "Minimum purchase",
            ("minimum_purchase", Language::Luganda) => "Omuwendo omutono ogw'okugula",
            ("minimum_purchase", Language::Swahili) => "Ununuzi wa chini",

            ("choose_agent", Language::English) => "Choose an agent",
            ("choose_agent", Language::Luganda) => "Londa agent",
            ("choose_agent", Language::Swahili) => "Chagua wakala",

            ("or_cancel", Language::English) => "or 0 to cancel",
            ("or_cancel", Language::Luganda) => "oba 0 okusazaamu",
            ("or_cancel", Language::Swahili) => "au 0 kughairi",

            ("purchase_summary", Language::English) => "Purchase Summary",
            ("purchase_summary", Language::Luganda) => "Ebikwata ku Kugula",
            ("purchase_summary", Language::Swahili) => "Muhtasari wa Ununuzi",

            ("you_pay", Language::English) => "You pay",
            ("you_pay", Language::Luganda) => "Osasula",
            ("you_pay", Language::Swahili) => "Utalipa",

            ("you_receive", Language::English) => "You receive",
            ("you_receive", Language::Luganda) => "Ofuna",
            ("you_receive", Language::Swahili) => "Utapokea",

            ("agent", Language::English) => "Agent",
            ("agent", Language::Luganda) => "Agent",
            ("agent", Language::Swahili) => "Wakala",

            ("rate", Language::English) => "Rate",
            ("rate", Language::Luganda) => "Omuwendo",
            ("rate", Language::Swahili) => "Bei",

            ("purchase_failed", Language::English) => "Purchase failed",
            ("purchase_failed", Language::Luganda) => "Okugula kulemeddwa",
            ("purchase_failed", Language::Swahili) => "Ununuzi umeshindwa",

            ("error_processing_purchase", Language::English) => "Error processing purchase. Please try again later.",
            ("error_processing_purchase", Language::Luganda) => "Kiremya mu kugula. Gezaako oluvannyuma.",
            ("error_processing_purchase", Language::Swahili) => "Kosa katika kuchakata ununuzi. Tafadhali jaribu tena baadaye.",

            ("available", Language::English) => "Available",
            ("available", Language::Luganda) => "Ebiriwo",
            ("available", Language::Swahili) => "Inapatikana",

            ("locked", Language::English) => "Locked",
            ("locked", Language::Luganda) => "Eziyiddwa",
            ("locked", Language::Swahili) => "Imefungwa",

            ("enter_btc_amount", Language::English) => "Enter BTC amount",
            ("enter_btc_amount", Language::Luganda) => "Yingiza omuwendo gwa BTC",
            ("enter_btc_amount", Language::Swahili) => "Weka kiasi cha BTC",

            ("minimum_sale", Language::English) => "Minimum sale",
            ("minimum_sale", Language::Luganda) => "Omuwendo omutono ogw'okutunda",
            ("minimum_sale", Language::Swahili) => "Mauzo ya chini",

            ("insufficient_btc", Language::English) => "Insufficient Bitcoin balance",
            ("insufficient_btc", Language::Luganda) => "Bitcoin tezimala",
            ("insufficient_btc", Language::Swahili) => "Salio la Bitcoin haitoshi",

            ("required", Language::English) => "Required",
            ("required", Language::Luganda) => "Ekyetaagisa",
            ("required", Language::Swahili) => "Inahitajika",

            ("enter_smaller_amount", Language::English) => "Enter a smaller amount",
            ("enter_smaller_amount", Language::Luganda) => "Yingiza omuwendo omutono",
            ("enter_smaller_amount", Language::Swahili) => "Weka kiasi kidogo",

            ("sale_summary", Language::English) => "Sale Summary",
            ("sale_summary", Language::Luganda) => "Ebikwata ku Kutunda",
            ("sale_summary", Language::Swahili) => "Muhtasari wa Mauzo",

            ("you_sell", Language::English) => "You sell",
            ("you_sell", Language::Luganda) => "Otunda",
            ("you_sell", Language::Swahili) => "Unauza",

            ("sale_failed", Language::English) => "Sale failed",
            ("sale_failed", Language::Luganda) => "Okutunda kulemeddwa",
            ("sale_failed", Language::Swahili) => "Mauzo yameshindwa",

            ("error_processing_sale", Language::English) => "Error processing sale. Please try again later.",
            ("error_processing_sale", Language::Luganda) => "Kiremya mu kutunda. Gezaako oluvannyuma.",
            ("error_processing_sale", Language::Swahili) => "Kosa katika kuchakata mauzo. Tafadhali jaribu tena baadaye.",

            ("enter_recipient_btc", Language::English) => "Enter recipient phone number",
            ("enter_recipient_btc", Language::Luganda) => "Yingiza namba ya simu y'omuntu",
            ("enter_recipient_btc", Language::Swahili) => "Weka nambari ya simu ya mpokeaji",

            ("recipient_not_found", Language::English) => "Recipient not found",
            ("recipient_not_found", Language::Luganda) => "Omuntu tazuuliddwa",
            ("recipient_not_found", Language::Swahili) => "Mpokeaji hajapatikana",

            ("they_need_register", Language::English) => "They need to register first",
            ("they_need_register", Language::Luganda) => "Balina okwewandiisa",
            ("they_need_register", Language::Swahili) => "Wanahitaji kusajili kwanza",

            ("enter_different_phone", Language::English) => "Enter different phone",
            ("enter_different_phone", Language::Luganda) => "Yingiza namba endala",
            ("enter_different_phone", Language::Swahili) => "Weka nambari nyingine",

            ("send_summary", Language::English) => "Send Summary",
            ("send_summary", Language::Luganda) => "Ebikwata ku Kuweereza",
            ("send_summary", Language::Swahili) => "Muhtasari wa Kutuma",

            ("to", Language::English) => "To",
            ("to", Language::Luganda) => "Eri",
            ("to", Language::Swahili) => "Kwa",
            
            ("with", Language::English) => "with",
            ("with", Language::Luganda) => "ne",
            ("with", Language::Swahili) => "na",

            ("network_fee", Language::English) => "Network Fee",
            ("network_fee", Language::Luganda) => "Ssente z'omutimbagano",
            ("network_fee", Language::Swahili) => "Ada ya Mtandao",

            ("send_failed", Language::English) => "Send failed",
            ("send_failed", Language::Luganda) => "Okuweereza kulemeddwa",
            ("send_failed", Language::Swahili) => "Kutuma kumeshindwa",

            ("error_processing_send", Language::English) => "Error processing send. Please try again later.",
            ("error_processing_send", Language::Luganda) => "Kiremya mu kuweereza. Gezaako oluvannyuma.",
            ("error_processing_send", Language::Swahili) => "Kosa katika kuchakata. Tafadhali jaribu tena baadaye.",

            ("usdc_menu_title", Language::English) => "USDC (ckUSDC)",
            ("usdc_menu_title", Language::Luganda) => "USDC (ckUSDC)",
            ("usdc_menu_title", Language::Swahili) => "USDC (ckUSDC)",

            ("usdc_rate", Language::English) => "USDC Rate",
            ("usdc_rate", Language::Luganda) => "Omuwendo gwa USDC",
            ("usdc_rate", Language::Swahili) => "Bei ya USDC",

            ("buy_usdc", Language::English) => "Buy USDC",
            ("buy_usdc", Language::Luganda) => "Gula USDC",
            ("buy_usdc", Language::Swahili) => "Nunua USDC",

            ("sell_usdc", Language::English) => "Sell USDC",
            ("sell_usdc", Language::Luganda) => "Tunda USDC",
            ("sell_usdc", Language::Swahili) => "Uza USDC",

            ("send_usdc", Language::English) => "Send USDC",
            ("send_usdc", Language::Luganda) => "Wereza USDC",
            ("send_usdc", Language::Swahili) => "Tuma USDC",

            ("current_rate", Language::English) => "Current Rate",
            ("current_rate", Language::Luganda) => "Omuwendo gwa kati",
            ("current_rate", Language::Swahili) => "Bei ya sasa",

            ("last_updated", Language::English) => "Last Updated",
            ("last_updated", Language::Luganda) => "Oluvannyuma olw'okusemba",
            ("last_updated", Language::Swahili) => "Imesasishwa mwisho",

            ("error_retrieving_rate", Language::English) => "Error retrieving rate. Please try again later.",
            ("error_retrieving_rate", Language::Luganda) => "Kiremya mu kufuna omuwendo. Gezaako oluvannyuma.",
            ("error_retrieving_rate", Language::Swahili) => "Kosa katika kupata bei. Tafadhali jaribu tena baadaye.",

            ("local_currency_menu", Language::English) => "Local Currency",
            ("local_currency_menu", Language::Luganda) => "Ssente z'omu Uganda",
            ("local_currency_menu", Language::Swahili) => "Sarafu ya Ndani",

            ("deposit", Language::English) => "Deposit",
            ("deposit", Language::Luganda) => "Teeka Ssente",
            ("deposit", Language::Swahili) => "Weka Pesa",

            ("transactions", Language::English) => "Transactions",
            ("transactions", Language::Luganda) => "Ebyafaayo",
            ("transactions", Language::Swahili) => "Miamala",

            ("recent_transactions", Language::English) => "Recent Transactions",
            ("recent_transactions", Language::Luganda) => "Ebyafaayo Ebisembayo",
            ("recent_transactions", Language::Swahili) => "Miamala ya Hivi Karibuni",

            ("find_agent", Language::English) => "Find Agent",
            ("find_agent", Language::Luganda) => "Noonya Agent",
            ("find_agent", Language::Swahili) => "Tafuta Wakala",

            ("view_proposals", Language::English) => "View Proposals",
            ("view_proposals", Language::Luganda) => "Laba Ebirowoozo",
            ("view_proposals", Language::Swahili) => "Angalia Mapendekezo",

            ("active_proposals", Language::English) => "Active Proposals",
            ("active_proposals", Language::Luganda) => "Ebirowoozo Ebikola",
            ("active_proposals", Language::Swahili) => "Mapendekezo Yanayofanya Kazi",

            ("my_voting_power", Language::English) => "My Voting Power",
            ("my_voting_power", Language::Luganda) => "Amaanyi Gange ag'Okulonda",
            ("my_voting_power", Language::Swahili) => "Nguvu Yangu ya Kupiga Kura",

            ("active_votes", Language::English) => "Active Votes",
            ("active_votes", Language::Luganda) => "Okulonda Okukola",
            ("active_votes", Language::Swahili) => "Kura Zinazofanya Kazi",

            ("registration_failed", Language::English) => "Registration failed. Please try again later.",
            ("registration_failed", Language::Luganda) => "Okwewandiisa kulemeddwa. Gezaako oluvannyuma.",
            ("registration_failed", Language::Swahili) => "Usajili umeshindwa. Tafadhali jaribu tena baadaye.",

            ("verification_failed", Language::English) => "Verification process error. Please try again.",
            ("verification_failed", Language::Luganda) => "Kiremya mu kukakasa. Gezaako nate.",
            ("verification_failed", Language::Swahili) => "Kosa katika uthibitisho. Tafadhali jaribu tena baadaye.",

            ("welcome_afritokeni", Language::English) => "Welcome to AfriTokeni!",
            ("welcome_afritokeni", Language::Luganda) => "Tukusanyukidde ku AfriTokeni!",
            ("welcome_afritokeni", Language::Swahili) => "Karibu AfriTokeni!",

            ("welcome_back", Language::English) => "Welcome back to AfriTokeni USSD Service",
            ("welcome_back", Language::Luganda) => "Tukusanyukidde nate ku mpeereza ya AfriTokeni USSD",
            ("welcome_back", Language::Swahili) => "Karibu tena kwa huduma ya AfriTokeni USSD",

            ("pin_set_success", Language::English) => "PIN set successfully!",
            ("pin_set_success", Language::Luganda) => "PIN etegekeddwa bulungi!",
            ("pin_set_success", Language::Swahili) => "PIN imewekwa kwa mafanikio!",

            ("pins_no_match", Language::English) => "PINs do not match",
            ("pins_no_match", Language::Luganda) => "PIN tezikwatagana",
            ("pins_no_match", Language::Swahili) => "PIN hazifanani",

            ("confirm_pin", Language::English) => "Please confirm your PIN by entering it again",
            ("confirm_pin", Language::Luganda) => "Kakasa PIN yo ng'oyingiza nate",
            ("confirm_pin", Language::Swahili) => "Tafadhali thibitisha PIN yako kwa kuiweka tena",

            ("error_saving_pin", Language::English) => "Error saving PIN. Please try again.",
            ("error_saving_pin", Language::Luganda) => "Kiremya mu kutereka PIN. Gezaako nate.",
            ("error_saving_pin", Language::Swahili) => "Kosa katika kuhifadhi PIN. Tafadhali jaribu tena.",

            ("enter_exactly_4_digits", Language::English) => "Please enter exactly 4 digits",
            ("enter_exactly_4_digits", Language::Luganda) => "Yingiza ennamba 4 zokka",
            ("enter_exactly_4_digits", Language::Swahili) => "Tafadhali weka nambari 4 tu",

            ("purchase_quote", Language::English) => "Purchase Quote",
            ("purchase_quote", Language::Luganda) => "Eky'okugula",
            ("purchase_quote", Language::Swahili) => "Nukuu ya Ununuzi",

            ("sale_quote", Language::English) => "Sale Quote",
            ("sale_quote", Language::Luganda) => "Eky'okutunda",
            ("sale_quote", Language::Swahili) => "Nukuu ya Mauzo",

            ("spend", Language::English) => "Spend",
            ("spend", Language::Luganda) => "Saasaanya",
            ("spend", Language::Swahili) => "Tumia",

            ("receive", Language::English) => "Receive",
            ("receive", Language::Luganda) => "Funa",
            ("receive", Language::Swahili) => "Pokea",

            ("sell", Language::English) => "Sell",
            ("sell", Language::Luganda) => "Tunda",
            ("sell", Language::Swahili) => "Uza",

            ("gross", Language::English) => "Gross",
            ("gross", Language::Luganda) => "Omugatte",
            ("gross", Language::Swahili) => "Jumla",

            ("net", Language::English) => "Net",
            ("net", Language::Luganda) => "Ekyasigadde",
            ("net", Language::Swahili) => "Safi",

            ("find_agent_title", Language::English) => "Find Agent",
            ("find_agent_title", Language::Luganda) => "Noonya Agent",
            ("find_agent_title", Language::Swahili) => "Tafuta Wakala",

            ("available_agents", Language::English) => "Available agents near you",
            ("available_agents", Language::Luganda) => "Ba-agent abali kumpi nawe",
            ("available_agents", Language::Swahili) => "Mawakala wanaopatikana karibu nawe",

            ("agent_details", Language::English) => "Agent Details",
            ("agent_details", Language::Luganda) => "Ebikwata ku Agent",
            ("agent_details", Language::Swahili) => "Maelezo ya Wakala",

            ("location", Language::English) => "Location",
            ("location", Language::Luganda) => "Ekifo",
            ("location", Language::Swahili) => "Mahali",

            ("address", Language::English) => "Address",
            ("address", Language::Luganda) => "Endagiriro",
            ("address", Language::Swahili) => "Anwani",

            ("services", Language::English) => "Services",
            ("services", Language::Luganda) => "Empeereza",
            ("services", Language::Swahili) => "Huduma",

            ("deposit_money", Language::English) => "Deposit money",
            ("deposit_money", Language::Luganda) => "Teeka ssente",
            ("deposit_money", Language::Swahili) => "Weka pesa",

            ("withdraw_money", Language::English) => "Withdraw money",
            ("withdraw_money", Language::Luganda) => "Ggya ssente",
            ("withdraw_money", Language::Swahili) => "Ondoa pesa",

            ("buy_sell_bitcoin", Language::English) => "Buy/Sell Bitcoin",
            ("buy_sell_bitcoin", Language::Luganda) => "Gula/Tunda Bitcoin",
            ("buy_sell_bitcoin", Language::Swahili) => "Nunua/Uza Bitcoin",

            ("visit_agent", Language::English) => "Visit the agent at their location for assistance",
            ("visit_agent", Language::Luganda) => "Kyalira agent ku kifo kye okufuna obuyambi",
            ("visit_agent", Language::Swahili) => "Tembelea wakala mahali pake kwa msaada",

            ("for_directions", Language::English) => "For directions or to contact agents directly, visit them at their listed locations",
            ("for_directions", Language::Luganda) => "Okufuna ekkubo oba okutumira ba-agent, bakyalire ku bifo byabwe",
            ("for_directions", Language::Swahili) => "Kwa maelekezo au kuwasiliana na mawakala moja kwa moja, watembelee mahali walipoorodheshwa",

            ("select_an_agent", Language::English) => "Select an agent",
            ("select_an_agent", Language::Luganda) => "Londa agent",
            ("select_an_agent", Language::Swahili) => "Chagua wakala",

            ("please_try_again_later", Language::English) => "Please try again later",
            ("please_try_again_later", Language::Luganda) => "Gezaako oluvannyuma",
            ("please_try_again_later", Language::Swahili) => "Tafadhali jaribu tena baadaye",

            ("source", Language::English) => "Source",
            ("source", Language::Luganda) => "Ensibuko",
            ("source", Language::Swahili) => "Chanzo",

            ("rates_include_fees", Language::English) => "Rates include platform fees",
            ("rates_include_fees", Language::Luganda) => "Emiwendo girimu ssente z'omutimbagano",
            ("rates_include_fees", Language::Swahili) => "Bei zinajumuisha ada za jukwaa",

            ("instant_transfers", Language::English) => "provides instant transfers with minimal fees",
            ("instant_transfers", Language::Luganda) => "ewa okuweereza okw'amangu n'essente ntono",
            ("instant_transfers", Language::Swahili) => "inatoa uhamisho wa haraka na ada ndogo",

            ("your_ckbtc_balance", Language::English) => "Your ckBTC Balance",
            ("your_ckbtc_balance", Language::Luganda) => "Ssente zo za ckBTC",
            ("your_ckbtc_balance", Language::Swahili) => "Salio yako ya ckBTC",

            ("bitcoin_exchange_rate", Language::English) => "Bitcoin Exchange Rate",
            ("bitcoin_exchange_rate", Language::Luganda) => "Omuwendo gwa Bitcoin",
            ("bitcoin_exchange_rate", Language::Swahili) => "Bei ya Kubadilishana Bitcoin",

            ("buy_sell_spreads", Language::English) => "Buy/Sell spreads may apply",
            ("buy_sell_spreads", Language::Luganda) => "Enjawulo y'okugula/okutunda esobola okubaawo",
            ("buy_sell_spreads", Language::Swahili) => "Tofauti za ununuzi/mauzo zinaweza kutumika",

            ("selected_agent", Language::English) => "Selected Agent",
            ("selected_agent", Language::Luganda) => "Agent alondeddwa",
            ("selected_agent", Language::Swahili) => "Wakala Aliyechaguliwa",

            ("purchase_details", Language::English) => "Purchase Details",
            ("purchase_details", Language::Luganda) => "Ebikwata ku Kugula",
            ("purchase_details", Language::Swahili) => "Maelezo ya Ununuzi",

            ("enter_pin_to_confirm", Language::English) => "Enter your PIN to confirm",
            ("enter_pin_to_confirm", Language::Luganda) => "Yingiza PIN yo okukakasa",
            ("enter_pin_to_confirm", Language::Swahili) => "Weka PIN yako kuthibitisha",

            ("purchase_initiated", Language::English) => "Purchase Initiated!",
            ("purchase_initiated", Language::Luganda) => "Okugula kutandise!",
            ("purchase_initiated", Language::Swahili) => "Ununuzi Umeanzishwa!",

            ("purchase_code", Language::English) => "Purchase Code",
            ("purchase_code", Language::Luganda) => "Koodi y'okugula",
            ("purchase_code", Language::Swahili) => "Nambari ya Ununuzi",

            ("transaction_id", Language::English) => "Transaction ID",
            ("transaction_id", Language::Luganda) => "ID y'ensaasaanya",
            ("transaction_id", Language::Swahili) => "Kitambulisho cha Muamala",

            ("you_will_receive", Language::English) => "You will receive",
            ("you_will_receive", Language::Luganda) => "Ojja kufuna",
            ("you_will_receive", Language::Swahili) => "Utapokea",

            ("amount_to_pay", Language::English) => "Amount to pay",
            ("amount_to_pay", Language::Luganda) => "Omuwendo ogw'okusasula",
            ("amount_to_pay", Language::Swahili) => "Kiasi cha kulipa",

            ("meet_agent_with_code", Language::English) => "Meet your selected agent with this code",
            ("meet_agent_with_code", Language::Luganda) => "Sisinkana ne agent wo n'okoodi eno",
            ("meet_agent_with_code", Language::Swahili) => "Kutana na wakala wako na nambari hii",

            ("no_ckbtc_available", Language::English) => "No ckBTC available to sell",
            ("no_ckbtc_available", Language::Luganda) => "Tewali ckBTC ezitundibwa",
            ("no_ckbtc_available", Language::Swahili) => "Hakuna ckBTC inayopatikana kuuza",

            ("sell_ckbtc", Language::English) => "Sell ckBTC",
            ("sell_ckbtc", Language::Luganda) => "Tunda ckBTC",
            ("sell_ckbtc", Language::Swahili) => "Uza ckBTC",

            ("btc_to_sell", Language::English) => "BTC to sell",
            ("btc_to_sell", Language::Luganda) => "BTC ezitundibwa",
            ("btc_to_sell", Language::Swahili) => "BTC ya kuuza",

            ("selling", Language::English) => "Selling",
            ("selling", Language::Luganda) => "Otunda",
            ("selling", Language::Swahili) => "Unauza",

            ("sale_code", Language::English) => "Sale Code",
            ("sale_code", Language::Luganda) => "Koodi y'okutunda",
            ("sale_code", Language::Swahili) => "Nambari ya Mauzo",

            ("sale_initiated", Language::English) => "Sale Initiated!",
            ("sale_initiated", Language::Luganda) => "Okutunda kutandise!",
            ("sale_initiated", Language::Swahili) => "Mauzo Yameanzishwa!",

            ("give_code_to_agent", Language::English) => "Give this code to the agent to complete your sale and collect cash",
            ("give_code_to_agent", Language::Luganda) => "Wa agent koodi eno okumaliriza okutunda kwo n'okufuna ssente",
            ("give_code_to_agent", Language::Swahili) => "Mpe wakala nambari hii kukamilisha mauzo yako na kupokea pesa",

            ("sms_sent", Language::English) => "SMS sent with details",
            ("sms_sent", Language::Luganda) => "SMS eweereddwa n'ebikwata ku nsaasaanya",
            ("sms_sent", Language::Swahili) => "SMS imetumwa na maelezo",

            ("code", Language::English) => "Code",
            ("code", Language::Luganda) => "Koodi",
            ("code", Language::Swahili) => "Nambari",

            ("ckbtc_to_receive", Language::English) => "ckBTC to receive",
            ("ckbtc_to_receive", Language::Luganda) => "ckBTC gy'ojja kufuna",
            ("ckbtc_to_receive", Language::Swahili) => "ckBTC utakayopokea",

            ("give_code_and_payment", Language::English) => "Give this code and payment to the agent to complete your ckBTC purchase",
            ("give_code_and_payment", Language::Luganda) => "Wa agent koodi eno n'essente okumaliriza okugula kwo ckBTC",
            ("give_code_and_payment", Language::Swahili) => "Mpe wakala nambari hii na malipo kukamilisha ununuzi wako wa ckBTC",

            ("ckbtc_balance", Language::English) => "ckBTC Balance",
            ("ckbtc_balance", Language::Luganda) => "Ssente za ckBTC",
            ("ckbtc_balance", Language::Swahili) => "Salio ya ckBTC",

            ("you_need_ckbtc", Language::English) => "You need ckBTC to send. Please buy some first",
            ("you_need_ckbtc", Language::Luganda) => "Weetaaga ckBTC okuweereza. Gula ezisooka",
            ("you_need_ckbtc", Language::Swahili) => "Unahitaji ckBTC kutuma. Tafadhali nunua kwanza",

            ("total_needed", Language::English) => "Total needed",
            ("total_needed", Language::Luganda) => "Omugatte ogwetaagisa",
            ("total_needed", Language::Swahili) => "Jumla inayohitajika",

            ("insufficient_for_fee", Language::English) => "Insufficient balance for amount + network fee!",
            ("insufficient_for_fee", Language::Luganda) => "Ssente tezimala omuwendo n'essente z'omutimbagano!",
            ("insufficient_for_fee", Language::Swahili) => "Salio haitoshi kwa kiasi + ada ya mtandao!",

            ("name", Language::English) => "Name",
            ("name", Language::Luganda) => "Erinnya",
            ("name", Language::Swahili) => "Jina",

            ("sent", Language::English) => "Sent",
            ("sent", Language::Luganda) => "Eweereddwa",
            ("sent", Language::Swahili) => "Imetumwa",

            ("received", Language::English) => "Received",
            ("received", Language::Luganda) => "Efuniddwa",
            ("received", Language::Swahili) => "Imepokelewa",

            ("from", Language::English) => "From",
            ("from", Language::Luganda) => "Okuva",
            ("from", Language::Swahili) => "Kutoka",

            ("new_balance_updated", Language::English) => "Your new balance will be updated shortly",
            ("new_balance_updated", Language::Luganda) => "Ssente zo empya zijja kukyusibwa mu bbanga ttono",
            ("new_balance_updated", Language::Swahili) => "Salio yako mpya itasasishwa hivi karibuni",

            ("sms_notifications_sent", Language::English) => "SMS notifications sent to both parties",
            ("sms_notifications_sent", Language::Luganda) => "SMS ziweereddwa eri bombi",
            ("sms_notifications_sent", Language::Swahili) => "Arifa za SMS zimetumwa kwa pande zote mbili",

            ("btc_sent_successfully", Language::English) => "BTC Sent Successfully!",
            ("btc_sent_successfully", Language::Luganda) => "BTC eweereddwa bulungi!",
            ("btc_sent_successfully", Language::Swahili) => "BTC Imetumwa kwa Mafanikio!",

            ("btc_received", Language::English) => "BTC Received",
            ("btc_received", Language::Luganda) => "BTC Efuniddwa",
            ("btc_received", Language::Swahili) => "BTC Imepokelewa",

            ("check_balance_dial", Language::English) => "Check your balance by dialing",
            ("check_balance_dial", Language::Luganda) => "Kebera ssente zo ng'okuba",
            ("check_balance_dial", Language::Swahili) => "Angalia salio yako kwa kupiga",

            ("format", Language::English) => "Format",
            ("format", Language::Luganda) => "Enkola",
            ("format", Language::Swahili) => "Muundo",

            ("invalid_phone_format", Language::English) => "Invalid phone number format",
            ("invalid_phone_format", Language::Luganda) => "Enkola y'enamba ya ssimu si ntuufu",
            ("invalid_phone_format", Language::Swahili) => "Muundo wa nambari ya simu si sahihi",

            ("ensure_account", Language::English) => "Please ensure they have an AfriTokeni account",
            ("ensure_account", Language::Luganda) => "Kakasa nti alina akawunti ya AfriTokeni",
            ("ensure_account", Language::Swahili) => "Tafadhali hakikisha ana akaunti ya AfriTokeni",

            ("cannot_send_to_self", Language::English) => "Cannot send to your own number",
            ("cannot_send_to_self", Language::Luganda) => "Tosobola kuweereza ku namba yo",
            ("cannot_send_to_self", Language::Swahili) => "Huwezi kutuma kwa nambari yako mwenyewe",

            ("time", Language::English) => "Time",
            ("time", Language::Luganda) => "Obudde",
            ("time", Language::Swahili) => "Wakati",

            ("sms_confirmations_sent", Language::English) => "SMS confirmations sent",
            ("sms_confirmations_sent", Language::Luganda) => "SMS z'okukakasa ziweereddwa",
            ("sms_confirmations_sent", Language::Swahili) => "Uthibitisho wa SMS umetumwa",

            ("too_many_attempts", Language::English) => "Too many incorrect PIN attempts",
            ("too_many_attempts", Language::Luganda) => "Okugezaako PIN embi emirundi mingi",
            ("too_many_attempts", Language::Swahili) => "Majaribio mengi ya PIN isiyo sahihi",

            ("attempts_remaining", Language::English) => "attempts remaining",
            ("attempts_remaining", Language::Luganda) => "okugezaako okusigadde",
            ("attempts_remaining", Language::Swahili) => "majaribio yaliyobaki",

            ("money_sent_successfully", Language::English) => "Money sent successfully!",
            ("money_sent_successfully", Language::Luganda) => "Ssente ziweereddwa bulungi!",
            ("money_sent_successfully", Language::Swahili) => "Pesa zimetumwa kwa mafanikio!",

            ("you_received_money", Language::English) => "You received money!",
            ("you_received_money", Language::Luganda) => "Ofunye ssente!",
            ("you_received_money", Language::Swahili) => "Umepokea pesa!",

            ("reference", Language::English) => "Reference",
            ("reference", Language::Luganda) => "Refelensi",
            ("reference", Language::Swahili) => "Kumbukumbu",

            ("withdrawal", Language::English) => "Withdrawal",
            ("withdrawal", Language::Luganda) => "Okusasula",
            ("withdrawal", Language::Swahili) => "Uondoaji",

            ("valid", Language::English) => "Valid",
            ("valid", Language::Luganda) => "Kikola",
            ("valid", Language::Swahili) => "Halali",

            ("hours", Language::English) => "hours",
            ("hours", Language::Luganda) => "essaawa",
            ("hours", Language::Swahili) => "masaa",

            ("show_code_to_agent", Language::English) => "Show this code to the agent with your ID to collect cash",
            ("show_code_to_agent", Language::Luganda) => "Laga agent koodi eno n'ekiraga naawe okufuna ssente",
            ("show_code_to_agent", Language::Swahili) => "Onyeshe wakala nambari hii na kitambulisho chako kupokea pesa",

            ("withdrawal_created", Language::English) => "Withdrawal Created!",
            ("withdrawal_created", Language::Luganda) => "Okusasula kutondeddwa!",
            ("withdrawal_created", Language::Swahili) => "Uondoaji Umeundwa!",

            ("try_again", Language::English) => "Try again",
            ("try_again", Language::Luganda) => "Gezaako nate",
            ("try_again", Language::Swahili) => "Jaribu tena",

            ("please_try_again", Language::English) => "Please try again",
            ("please_try_again", Language::Luganda) => "Gezaako nate",
            ("please_try_again", Language::Swahili) => "Tafadhali jaribu tena",

            ("already_voted", Language::English) => "Already voted on this proposal!",
            ("already_voted", Language::Luganda) => "Omaze okulonda ku kirowoozo kino!",
            ("already_voted", Language::Swahili) => "Umeshapiga kura kwenye pendekezo hili!",

            ("your_vote", Language::English) => "Your vote",
            ("your_vote", Language::Luganda) => "Okulonda kwo",
            ("your_vote", Language::Swahili) => "Kura yako",

            ("thank_you_governance", Language::English) => "Thank you for participating in AfriTokeni governance!",
            ("thank_you_governance", Language::Luganda) => "Webale okwetaba mu kufuga AfriTokeni!",
            ("thank_you_governance", Language::Swahili) => "Asante kwa kushiriki katika utawala wa AfriTokeni!",

            ("voting_ends_in", Language::English) => "Voting ends in",
            ("voting_ends_in", Language::Luganda) => "Okulonda kuggwawo mu",
            ("voting_ends_in", Language::Swahili) => "Kupiga kura kunaisha ndani ya",

            ("days", Language::English) => "days",
            ("days", Language::Luganda) => "ennaku",
            ("days", Language::Swahili) => "siku",

            ("current_votes", Language::English) => "Current votes",
            ("current_votes", Language::Luganda) => "Okulonda kwa kati",
            ("current_votes", Language::Swahili) => "Kura za sasa",

            ("vote_yes", Language::English) => "Vote YES",
            ("vote_yes", Language::Luganda) => "Londa YE",
            ("vote_yes", Language::Swahili) => "Piga kura NDIO",

            ("vote_no", Language::English) => "Vote NO",
            ("vote_no", Language::Luganda) => "Londa NEDDA",
            ("vote_no", Language::Swahili) => "Piga kura HAPANA",

            ("vote_abstain", Language::English) => "Vote ABSTAIN",
            ("vote_abstain", Language::Luganda) => "Londa NZIYIZA",
            ("vote_abstain", Language::Swahili) => "Piga kura ZUIA",

            ("vote", Language::English) => "Vote",
            ("vote", Language::Luganda) => "Londa",
            ("vote", Language::Swahili) => "Piga kura",

            ("enter_vote_amount", Language::English) => "Enter amount to vote (min 1 AFRI)",
            ("enter_vote_amount", Language::Luganda) => "Yingiza omuwendo ogw'okulonda (omutono 1 AFRI)",
            ("enter_vote_amount", Language::Swahili) => "Weka kiasi cha kupiga kura (chini 1 AFRI)",

            ("insufficient_tokens", Language::English) => "Insufficient tokens!",
            ("insufficient_tokens", Language::Luganda) => "Tokens tezimala!",
            ("insufficient_tokens", Language::Swahili) => "Tokens hazitoshi!",

            ("requested", Language::English) => "Requested",
            ("requested", Language::Luganda) => "Esabiddwa",
            ("requested", Language::Swahili) => "Imeombwa",

            ("confirm_vote", Language::English) => "Confirm Vote",
            ("confirm_vote", Language::Luganda) => "Kakasa Okulonda",
            ("confirm_vote", Language::Swahili) => "Thibitisha Kura",

            ("proposal", Language::English) => "Proposal",
            ("proposal", Language::Luganda) => "Ekirowoozo",
            ("proposal", Language::Swahili) => "Pendekezo",

            ("vote_successful", Language::English) => "Vote Successful!",
            ("vote_successful", Language::Luganda) => "Okulonda kuwangudde!",
            ("vote_successful", Language::Swahili) => "Kura Imefanikiwa!",

            ("voted", Language::English) => "Voted",
            ("voted", Language::Luganda) => "Olonze",
            ("voted", Language::Swahili) => "Umepiga kura",

            ("vote_locked_message", Language::English) => "Your vote is locked until proposal ends.",
            ("vote_locked_message", Language::Luganda) => "Okulonda kwo kuziyiddwa okutuusa ekirowoozo lwe kiggwawo.",
            ("vote_locked_message", Language::Swahili) => "Kura yako imefungwa hadi pendekezo liishe.",

            ("your_voting_power", Language::English) => "Your Voting Power",
            ("your_voting_power", Language::Luganda) => "Amaanyi Go ag'Okulonda",
            ("your_voting_power", Language::Swahili) => "Nguvu Yako ya Kupiga Kura",

            ("locked_tokens_message", Language::English) => "Locked tokens released when proposals end.",
            ("locked_tokens_message", Language::Luganda) => "Tokens eziziyiddwa zisumulukibwa ekirowoozo lwe kiggwawo.",
            ("locked_tokens_message", Language::Swahili) => "Tokens zilizofungwa zitatolewa mapendekezo yatakapoisha.",

            ("your_active_votes", Language::English) => "Your Active Votes",
            ("your_active_votes", Language::Luganda) => "Okulonda Kwo Okukola",
            ("your_active_votes", Language::Swahili) => "Kura Zako Zinazofanya Kazi",

            ("no_active_votes", Language::English) => "You have no active votes.",
            ("no_active_votes", Language::Luganda) => "Tolina kulonda kukola.",
            ("no_active_votes", Language::Swahili) => "Huna kura zinazofanya kazi.",

            ("vote_to_participate", Language::English) => "Vote on proposals to participate in governance!",
            ("vote_to_participate", Language::Luganda) => "Londa ku birowoozo okwetaba mu kufuga!",
            ("vote_to_participate", Language::Swahili) => "Piga kura kwenye mapendekezo kushiriki katika utawala!",

            ("total_locked", Language::English) => "Total locked",
            ("total_locked", Language::Luganda) => "Omugatte oguziyiddwa",
            ("total_locked", Language::Swahili) => "Jumla iliyofungwa",

            ("btc_purchase_quote", Language::English) => "Bitcoin Purchase Quote",
            ("btc_purchase_quote", Language::Luganda) => "Eky'okugula Bitcoin",
            ("btc_purchase_quote", Language::Swahili) => "Nukuu ya Ununuzi wa Bitcoin",

            ("btc_sale_quote", Language::English) => "Bitcoin Sale Quote",
            ("btc_sale_quote", Language::Luganda) => "Eky'okutunda Bitcoin",
            ("btc_sale_quote", Language::Swahili) => "Nukuu ya Mauzo ya Bitcoin",

            ("recipient", Language::English) => "Recipient",
            ("recipient", Language::Luganda) => "Omufuna",
            ("recipient", Language::Swahili) => "Mpokeaji",

            ("sale_details", Language::English) => "Sale Details",
            ("sale_details", Language::Luganda) => "Ebikwata ku Kutunda",
            ("sale_details", Language::Swahili) => "Maelezo ya Mauzo",

            ("verification_code_sent", Language::English) => "We've sent a verification code to your phone.",
            ("verification_code_sent", Language::Luganda) => "Tuweerezza koodi y'okukakasa ku ssimu yo.",
            ("verification_code_sent", Language::Swahili) => "Tumetuma nambari ya uthibitisho kwa simu yako.",

            ("enter_verification_code", Language::English) => "Please enter the 6-digit code",
            ("enter_verification_code", Language::Luganda) => "Yingiza koodi y'ennamba 6",
            ("enter_verification_code", Language::Swahili) => "Weka nambari ya nambari 6",

            ("invalid_code_format", Language::English) => "Invalid code format",
            ("invalid_code_format", Language::Luganda) => "Enkola ya koodi si ntuufu",
            ("invalid_code_format", Language::Swahili) => "Muundo wa nambari si sahihi",

            ("invalid_verification_code", Language::English) => "Invalid verification code",
            ("invalid_verification_code", Language::Luganda) => "Koodi y'okukakasa si ntuufu",
            ("invalid_verification_code", Language::Swahili) => "Nambari ya uthibitisho si sahihi",

            ("setup_pin_message", Language::English) => "To secure your account, please set up a 4-digit PIN",
            ("setup_pin_message", Language::Luganda) => "Okukuuma akawunti yo, tegeka PIN ya namba 4",
            ("setup_pin_message", Language::Swahili) => "Kulinda akaunti yako, weka PIN ya nambari 4",

            ("enter_new_pin", Language::English) => "Enter your new PIN",
            ("enter_new_pin", Language::Luganda) => "Yingiza PIN yo empya",
            ("enter_new_pin", Language::Swahili) => "Weka PIN yako mpya",

            ("invalid_name", Language::English) => "Invalid name",
            ("invalid_name", Language::Luganda) => "Erinnya si lituufu",
            ("invalid_name", Language::Swahili) => "Jina si sahihi",

            ("enter_full_name", Language::English) => "Please enter your full name (first and last name)",
            ("enter_full_name", Language::Luganda) => "Yingiza erinnya lyo lyonna (erinnya ly'omu maaso n'ery'oluvannyuma)",
            ("enter_full_name", Language::Swahili) => "Weka jina lako kamili (jina la kwanza na la mwisho)",

            ("verification_successful", Language::English) => "Verification successful",
            ("verification_successful", Language::Luganda) => "Okukakasa kugenze bulungi",
            ("verification_successful", Language::Swahili) => "Uthibitisho umefanikiwa",

            ("account_created", Language::English) => "Account created successfully",
            ("account_created", Language::Luganda) => "Akawunti etondeddwa bulungi",
            ("account_created", Language::Swahili) => "Akaunti imeundwa kwa mafanikio",

            ("not_registered_yet", Language::English) => "You are not registered yet",
            ("not_registered_yet", Language::Luganda) => "Tonnawandiisibwa",
            ("not_registered_yet", Language::Swahili) => "Bado hujajiandikisha",

            ("please_wait", Language::English) => "Please wait",
            ("please_wait", Language::Luganda) => "Lindawo",
            ("please_wait", Language::Swahili) => "Tafadhali subiri",

            ("enter_both_names", Language::English) => "Please enter both first and last name separated by space",
            ("enter_both_names", Language::Luganda) => "Yingiza erinnya ly'omu maaso n'ery'oluvannyuma nga biriko ekifo wakati",
            ("enter_both_names", Language::Swahili) => "Weka jina la kwanza na la mwisho likitengwa na nafasi",

            ("coming_soon", Language::English) => "coming soon",
            ("coming_soon", Language::Luganda) => "ejja mu biseera ebitali bya wala",
            ("coming_soon", Language::Swahili) => "inakuja hivi karibuni",

            ("vote_on_proposals", Language::English) => "Vote on proposals",
            ("vote_on_proposals", Language::Luganda) => "Londa ku byogerwa",
            ("vote_on_proposals", Language::Swahili) => "Piga kura kwenye mapendekezo",

            ("for_support", Language::English) => "For support",
            ("for_support", Language::Luganda) => "Okufuna obuyambi",
            ("for_support", Language::Swahili) => "Kwa msaada",

            ("call", Language::English) => "Call",
            ("call", Language::Luganda) => "Kuba",
            ("call", Language::Swahili) => "Piga simu",

            ("visit", Language::English) => "Visit",
            ("visit", Language::Luganda) => "Kyalira",
            ("visit", Language::Swahili) => "Tembelea",

            ("no_transactions", Language::English) => "No transactions found",
            ("no_transactions", Language::Luganda) => "Tewali nsimbi ezisindikiddwa",
            ("no_transactions", Language::Swahili) => "Hakuna miamala iliyopatikana",

            ("to_start_using", Language::English) => "To start using AfriTokeni, send money or make a deposit through an agent",
            ("to_start_using", Language::Luganda) => "Okutandika okukozesa AfriTokeni, wereza ssente oba teeka ssente ng'oyita mu agent",
            ("to_start_using", Language::Swahili) => "Ili kuanza kutumia AfriTokeni, tuma pesa au fanya amana kupitia wakala",

            ("last", Language::English) => "Last",
            ("last", Language::Luganda) => "Ezisembayo",
            ("last", Language::Swahili) => "Za mwisho",

            ("your_account_balance", Language::English) => "Your Account Balance",
            ("your_account_balance", Language::Luganda) => "Ssente zo ku Akawunti",
            ("your_account_balance", Language::Swahili) => "Salio yako ya Akaunti",

            ("error_retrieving_history", Language::English) => "Error retrieving transaction history.",
            ("error_retrieving_history", Language::Luganda) => "Kiremya mu kufuna ebyafaayo by'ensimbi.",
            ("error_retrieving_history", Language::Swahili) => "Kosa katika kupata historia ya muamala.",

            ("thank_you_using_afritokeni", Language::English) => "Thank you for using AfriTokeni!",
            ("thank_you_using_afritokeni", Language::Luganda) => "Webale okukozesa AfriTokeni!",
            ("thank_you_using_afritokeni", Language::Swahili) => "Asante kwa kutumia AfriTokeni!",
            
            // Default fallback - return empty string for unknown keys
            _ => "",
        }
    }
    
    pub fn get_main_menu(lang: Language, currency: &str) -> String {
        format!(
            "{}\n1. {} ({})\n2. {} (ckBTC)\n3. {} (ckUSDC)\n4. Swap Crypto\n5. {}\n6. {}\n7. {}",
            Self::translate("welcome", lang),
            Self::translate("local_currency", lang),
            currency,
            Self::translate("bitcoin", lang),
            Self::translate("usdc", lang),
            Self::translate("dao_governance", lang),
            Self::translate("help", lang),
            Self::translate("language_selection", lang)
        )
    }
}
