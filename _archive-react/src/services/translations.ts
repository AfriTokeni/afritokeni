/**
 * Multi-Language Translation Service
 * Supports English, Luganda, and Swahili
 */

export type Language = 'en' | 'lg' | 'sw';

interface Translations {
  [key: string]: {
    en: string;
    lg: string; // Luganda
    sw: string; // Swahili
  };
}

export class TranslationService {
  private static translations: Translations = {
    // Welcome messages
    'welcome': {
      en: 'Welcome to AfriTokeni!',
      lg: 'Tukusanyukidde ku AfriTokeni!',
      sw: 'Karibu AfriTokeni!'
    },
    'account_ready': {
      en: 'Your account is ready.',
      lg: 'Akawunti yo etegefu.',
      sw: 'Akaunti yako iko tayari.'
    },
    
    // Menu items
    'local_currency': {
      en: 'Local Currency',
      lg: 'Ssente z\'omu Uganda',
      sw: 'Sarafu ya Ndani'
    },
    'bitcoin': {
      en: 'Bitcoin',
      lg: 'Bitcoin',
      sw: 'Bitcoin'
    },
    'usdc': {
      en: 'USDC',
      lg: 'USDC',
      sw: 'USDC'
    },
    'dao_governance': {
      en: 'DAO Governance',
      lg: 'Okufuga kwa DAO',
      sw: 'Utawala wa DAO'
    },
    
    // Balance
    'balance': {
      en: 'Balance',
      lg: 'Ssente',
      sw: 'Salio'
    },
    'your_balance': {
      en: 'Your balance is',
      lg: 'Ssente zo',
      sw: 'Salio yako ni'
    },
    
    // Transactions
    'send_money': {
      en: 'Send Money',
      lg: 'Wereza Ssente',
      sw: 'Tuma Pesa'
    },
    'withdraw': {
      en: 'Withdraw Cash',
      lg: 'Ggya Ssente',
      sw: 'Ondoa Pesa'
    },
    'transaction_success': {
      en: 'Transaction successful!',
      lg: 'Ensimbi ziweereddwa!',
      sw: 'Muamala umefanikiwa!'
    },
    'transaction_successful': {
      en: 'Transaction successful',
      lg: 'Ensimbi ziweereddwa',
      sw: 'Muamala umefanikiwa'
    },
    'transaction_failed': {
      en: 'Transaction failed',
      lg: 'Ensimbi teziweereddwa',
      sw: 'Muamala umeshindwa'
    },
    
    // Confirmations
    'confirm': {
      en: 'Confirm',
      lg: 'Kakasa',
      sw: 'Thibitisha'
    },
    'confirmation': {
      en: 'Confirmation',
      lg: 'Okukakasa',
      sw: 'Uthibitisho'
    },
    'cancel': {
      en: 'Cancel',
      lg: 'Sazaamu',
      sw: 'Ghairi'
    },
    'confirm_transaction': {
      en: 'Confirm transaction?',
      lg: 'Kakasa okuweereza?',
      sw: 'Thibitisha muamala?'
    },
    
    // Amounts
    'amount': {
      en: 'Amount',
      lg: 'Omuwendo',
      sw: 'Kiasi'
    },
    'fee': {
      en: 'Fee',
      lg: 'Ssente z\'okuweereza',
      sw: 'Ada'
    },
    'total': {
      en: 'Total',
      lg: 'Omugatte',
      sw: 'Jumla'
    },
    
    // Bitcoin
    'bitcoin_balance': {
      en: 'Bitcoin balance',
      lg: 'Ssente za Bitcoin',
      sw: 'Salio la Bitcoin'
    },
    'bitcoin_rate': {
      en: 'Bitcoin rate',
      lg: 'Omuwendo gwa Bitcoin',
      sw: 'Bei ya Bitcoin'
    },
    'buy_bitcoin': {
      en: 'Buy Bitcoin',
      lg: 'Gula Bitcoin',
      sw: 'Nunua Bitcoin'
    },
    'sell_bitcoin': {
      en: 'Sell Bitcoin',
      lg: 'Tunda Bitcoin',
      sw: 'Uza Bitcoin'
    },
    
    // Errors
    'error': {
      en: 'Error',
      lg: 'Kiremya',
      sw: 'Kosa'
    },
    'invalid_option': {
      en: 'Invalid option. Please try again:',
      lg: 'Ekiragiro si kituufu. Gezaako nate:',
      sw: 'Chaguo si sahihi. Jaribu tena:'
    },
    'insufficient_balance': {
      en: 'Insufficient balance',
      lg: 'Ssente tezimala',
      sw: 'Salio haitoshi'
    },
    'invalid_amount': {
      en: 'Invalid amount',
      lg: 'Omuwendo si mutuufu',
      sw: 'Kiasi si sahihi'
    },
    'invalid_phone': {
      en: 'Invalid phone number',
      lg: 'Namba ya simu si ntuufu',
      sw: 'Nambari ya simu si sahihi'
    },
    'enter_recipient_phone': {
      en: 'Enter recipient phone number:',
      lg: 'Yingiza namba ya simu y\'omuntu:',
      sw: 'Weka nambari ya simu ya mpokeaji:'
    },
    'phone_format_example': {
      en: '(e.g. 256700123456)',
      lg: '(okugeza: 256700123456)',
      sw: '(mfano: 256700123456)'
    },
    
    // Rate limiting
    'too_many_requests': {
      en: 'Too many requests. Please wait.',
      lg: 'Osabidde emirundi mingi. Linda.',
      sw: 'Maombi mengi sana. Tafadhali subiri.'
    },
    'wait_minute': {
      en: 'Wait 1 minute',
      lg: 'Linda dakiika 1',
      sw: 'Subiri dakika 1'
    },
    'daily_limit_reached': {
      en: 'Daily limit reached',
      lg: 'Omugendo gw\'olunaku gutuuse',
      sw: 'Kikomo cha siku kimefikiwa'
    },
    
    // Fraud detection
    'suspicious_activity': {
      en: 'Suspicious activity detected',
      lg: 'Ebikolwa ebitali bya bulijjo bizuuliddwa',
      sw: 'Shughuli ya kutilia shaka imegunduliwa'
    },
    'transaction_blocked': {
      en: 'Transaction blocked for security',
      lg: 'Okuweereza kuziyiddwa olw\'obukuumi',
      sw: 'Muamala umezuiwa kwa usalama'
    },
    'verification_required': {
      en: 'Verification required',
      lg: 'Okukakasa kwetaagisa',
      sw: 'Uthibitisho unahitajika'
    },
    
    // PIN
    'enter_pin': {
      en: 'Enter PIN',
      lg: 'Yingiza PIN',
      sw: 'Weka PIN'
    },
    'wrong_pin': {
      en: 'Wrong PIN',
      lg: 'PIN si ntuufu',
      sw: 'PIN si sahihi'
    },
    'pin_set': {
      en: 'PIN set successfully',
      lg: 'PIN etegekeddwa bulungi',
      sw: 'PIN imewekwa kwa mafanikio'
    },
    'account_locked': {
      en: 'Account locked. Too many attempts.',
      lg: 'Akawunti eziyiddwa. Ogezezzaako emirundi mingi.',
      sw: 'Akaunti imefungwa. Majaribio mengi sana.'
    },
    
    // Session management
    'session_expired': {
      en: 'Session expired. Please dial *384*22948# again to start a new session.',
      lg: 'Obudde buweddeyo. Kuba *384*22948# okutandika omulimu omupya.',
      sw: 'Kipindi kimemalizika. Tafadhali piga *384*22948# tena kuanza kipindi kipya.'
    },
    
    // Menu items
    'check_balance': {
      en: 'Check Balance',
      lg: 'Kebera Ssente',
      sw: 'Angalia Salio'
    },
    'transaction_history': {
      en: 'Transaction History',
      lg: 'Ebyafaayo by\'ensimbi',
      sw: 'Historia ya Muamala'
    },
    'bitcoin_services': {
      en: 'Bitcoin Services',
      lg: 'Empeereza za Bitcoin',
      sw: 'Huduma za Bitcoin'
    },
    'help': {
      en: 'Help',
      lg: 'Obuyambi',
      sw: 'Msaada'
    },
    
    // Help text
    'help_commands': {
      en: 'Commands: BAL, SEND, WITHDRAW, BTC BAL, HELP',
      lg: 'Ebiragiro: BAL, SEND, WITHDRAW, BTC BAL, HELP',
      sw: 'Amri: BAL, SEND, WITHDRAW, BTC BAL, HELP'
    },
    'help_ussd': {
      en: 'Dial *22948# for menu',
      lg: 'Kuba *22948# okufuna menu',
      sw: 'Piga *22948# kwa menyu'
    },
    
    // Language selection
    'select_language': {
      en: 'Select language:\n1. English\n2. Luganda\n3. Swahili',
      lg: 'Londa olulimi:\n1. English\n2. Luganda\n3. Swahili',
      sw: 'Chagua lugha:\n1. English\n2. Luganda\n3. Swahili'
    },
    'language_set': {
      en: 'Language set to English',
      lg: 'Olulimi lutegekeddwa ku Luganda',
      sw: 'Lugha imewekwa kwa Kiswahili'
    },
    'press_zero_back': {
      en: 'Press 0 to return to main menu',
      lg: 'Nyiga 0 okudda ku menu enkulu',
      sw: 'Bonyeza 0 kurudi kwa menyu kuu'
    },
    'back_or_menu': {
      en: '0. Back | 9. Menu',
      lg: '0. Ddayo | 9. Menu',
      sw: '0. Rudi | 9. Menyu'
    },
    
    // Common prompts
    'enter_amount': {
      en: 'Enter amount',
      lg: 'Yingiza omuwendo',
      sw: 'Weka kiasi'
    },
    'enter_pin_4digit': {
      en: 'Enter your 4-digit PIN',
      lg: 'Yingiza PIN yo ey\'ennamba 4',
      sw: 'Weka PIN yako ya nambari 4'
    },
    'invalid_pin_format': {
      en: 'Invalid PIN format',
      lg: 'Enkola ya PIN si ntuufu',
      sw: 'Muundo wa PIN si sahihi'
    },
    'incorrect_pin': {
      en: 'Incorrect PIN',
      lg: 'PIN si ntuufu',
      sw: 'PIN si sahihi'
    },
    'please_select_option': {
      en: 'Please select an option:',
      lg: 'Londa ekiragiro:',
      sw: 'Tafadhali chagua chaguo:'
    },
    'back_to_main_menu': {
      en: 'Back to Main Menu',
      lg: 'Ddayo ku Menu Enkulu',
      sw: 'Rudi kwa Menyu Kuu'
    },
    'invalid_selection': {
      en: 'Invalid selection',
      lg: 'Ekiragiro si kituufu',
      sw: 'Chaguo si sahihi'
    },
    'user_not_found': {
      en: 'User not found. Please contact support.',
      lg: 'Omukozesa tazuuliddwa. Tuukirire obuyambi.',
      sw: 'Mtumiaji hajapatikana. Tafadhali wasiliana na msaada.'
    },
    'error_try_again': {
      en: 'Error. Please try again later.',
      lg: 'Kiremya. Gezaako oluvannyuma.',
      sw: 'Kosa. Tafadhali jaribu tena baadaye.'
    },
    'thank_you': {
      en: 'Thank you for using AfriTokeni!',
      lg: 'Webale okukozesa AfriTokeni!',
      sw: 'Asante kwa kutumia AfriTokeni!'
    },
    'dial_to_start_new_session': {
      en: '\n\nDial *384*22948# to start a new session',
      lg: '\n\nKuba *384*22948# okutandika omulundi omupya',
      sw: '\n\nPiga *384*22948# kuanza kipindi kipya'
    },
    'transaction_cancelled': {
      en: 'Transaction cancelled',
      lg: 'Okuweereza kusaziddwamu',
      sw: 'Muamala umeghairiwa'
    },
    'minimum_amount': {
      en: 'Minimum',
      lg: 'Omuwendo omutono',
      sw: 'Kiwango cha chini'
    },
    'enter_pin_confirm': {
      en: 'Enter your PIN to confirm',
      lg: 'Yingiza PIN yo okukakasa',
      sw: 'Weka PIN yako kuthibitisha'
    },
    'no_agents_available': {
      en: 'No agents available at this time. Please try again later.',
      lg: 'Tewali ba-agent kati. Gezaako oluvannyuma.',
      sw: 'Hakuna mawakala kwa sasa. Tafadhali jaribu tena baadaye.'
    },
    'select_agent': {
      en: 'Select an agent',
      lg: 'Londa agent',
      sw: 'Chagua wakala'
    },
    'choose_amount_type': {
      en: 'Choose amount type',
      lg: 'Londa ekika ky\'omuwendo',
      sw: 'Chagua aina ya kiasi'
    },
    'transaction_complete': {
      en: 'Transaction complete!',
      lg: 'Okuweereza kuwedde!',
      sw: 'Muamala umekamilika!'
    },
    'meet_agent': {
      en: 'Meet your selected agent to complete the transaction.',
      lg: 'Sisinkana ne agent wo okumaliriza.',
      sw: 'Kutana na wakala wako kukamilisha muamala.'
    },
    'receive_cash': {
      en: 'Meet your selected agent to receive cash.',
      lg: 'Sisinkana ne agent wo okufuna ssente.',
      sw: 'Kutana na wakala wako kupokea pesa.'
    },
    'send_bitcoin': {
      en: 'Send Bitcoin',
      lg: 'Wereza Bitcoin',
      sw: 'Tuma Bitcoin'
    },
    'bitcoin_menu_title': {
      en: 'Bitcoin (ckBTC)',
      lg: 'Bitcoin (ckBTC)',
      sw: 'Bitcoin (ckBTC)'
    },
    'buy_btc_enter_amount': {
      en: 'Buy Bitcoin',
      lg: 'Gula Bitcoin',
      sw: 'Nunua Bitcoin'
    },
    'enter_amount_to_spend': {
      en: 'Enter amount to spend',
      lg: 'Yingiza omuwendo ogw\'okusaasaanya',
      sw: 'Weka kiasi cha kutumia'
    },
    'minimum_purchase': {
      en: 'Minimum purchase',
      lg: 'Omuwendo omutono ogw\'okugula',
      sw: 'Ununuzi wa chini'
    },
    'choose_agent': {
      en: 'Choose an agent',
      lg: 'Londa agent',
      sw: 'Chagua wakala'
    },
    'or_cancel': {
      en: 'or 0 to cancel',
      lg: 'oba 0 okusazaamu',
      sw: 'au 0 kughairi'
    },
    'purchase_summary': {
      en: 'Purchase Summary',
      lg: 'Ebikwata ku Kugula',
      sw: 'Muhtasari wa Ununuzi'
    },
    'you_pay': {
      en: 'You pay',
      lg: 'Osasula',
      sw: 'Utalipa'
    },
    'you_receive': {
      en: 'You receive',
      lg: 'Ofuna',
      sw: 'Utapokea'
    },
    'agent': {
      en: 'Agent',
      lg: 'Agent',
      sw: 'Wakala'
    },
    'rate': {
      en: 'Rate',
      lg: 'Omuwendo',
      sw: 'Bei'
    },
    'purchase_failed': {
      en: 'Purchase failed',
      lg: 'Okugula kulemeddwa',
      sw: 'Ununuzi umeshindwa'
    },
    'error_processing_purchase': {
      en: 'Error processing purchase. Please try again later.',
      lg: 'Kiremya mu kugula. Gezaako oluvannyuma.',
      sw: 'Kosa katika kuchakata ununuzi. Tafadhali jaribu tena baadaye.'
    },
    'available': {
      en: 'Available',
      lg: 'Ebiriwo',
      sw: 'Inapatikana'
    },
    'locked': {
      en: 'Locked',
      lg: 'Eziyiddwa',
      sw: 'Imefungwa'
    },
    'enter_btc_amount': {
      en: 'Enter BTC amount',
      lg: 'Yingiza omuwendo gwa BTC',
      sw: 'Weka kiasi cha BTC'
    },
    'minimum_sale': {
      en: 'Minimum sale',
      lg: 'Omuwendo omutono ogw\'okutunda',
      sw: 'Mauzo ya chini'
    },
    'insufficient_btc': {
      en: 'Insufficient Bitcoin balance',
      lg: 'Bitcoin tezimala',
      sw: 'Salio la Bitcoin haitoshi'
    },
    'required': {
      en: 'Required',
      lg: 'Ekyetaagisa',
      sw: 'Inahitajika'
    },
    'enter_smaller_amount': {
      en: 'Enter a smaller amount',
      lg: 'Yingiza omuwendo omutono',
      sw: 'Weka kiasi kidogo'
    },
    'sale_summary': {
      en: 'Sale Summary',
      lg: 'Ebikwata ku Kutunda',
      sw: 'Muhtasari wa Mauzo'
    },
    'you_sell': {
      en: 'You sell',
      lg: 'Otunda',
      sw: 'Unauza'
    },
    'sale_failed': {
      en: 'Sale failed',
      lg: 'Okutunda kulemeddwa',
      sw: 'Mauzo yameshindwa'
    },
    'error_processing_sale': {
      en: 'Error processing sale. Please try again later.',
      lg: 'Kiremya mu kutunda. Gezaako oluvannyuma.',
      sw: 'Kosa katika kuchakata mauzo. Tafadhali jaribu tena baadaye.'
    },
    'enter_recipient_btc': {
      en: 'Enter recipient phone number',
      lg: 'Yingiza namba ya simu y\'omuntu',
      sw: 'Weka nambari ya simu ya mpokeaji'
    },
    'recipient_not_found': {
      en: 'Recipient not found',
      lg: 'Omuntu tazuuliddwa',
      sw: 'Mpokeaji hajapatikana'
    },
    'they_need_register': {
      en: 'They need to register first',
      lg: 'Balina okwewandiisa',
      sw: 'Wanahitaji kusajili kwanza'
    },
    'enter_different_phone': {
      en: 'Enter different phone',
      lg: 'Yingiza namba endala',
      sw: 'Weka nambari nyingine'
    },
    'send_summary': {
      en: 'Send Summary',
      lg: 'Ebikwata ku Kuweereza',
      sw: 'Muhtasari wa Kutuma'
    },
    'to': {
      en: 'To',
      lg: 'Eri',
      sw: 'Kwa'
    },
    'network_fee': {
      en: 'Network Fee',
      lg: 'Ssente z\'omutimbagano',
      sw: 'Ada ya Mtandao'
    },
    'send_failed': {
      en: 'Send failed',
      lg: 'Okuweereza kulemeddwa',
      sw: 'Kutuma kumeshindwa'
    },
    'error_processing_send': {
      en: 'Error processing send. Please try again later.',
      lg: 'Kiremya mu kuweereza. Gezaako oluvannyuma.',
      sw: 'Kosa katika kuchakata. Tafadhali jaribu tena baadaye.'
    },
    'usdc_menu_title': {
      en: 'USDC (ckUSDC)',
      lg: 'USDC (ckUSDC)',
      sw: 'USDC (ckUSDC)'
    },
    'usdc_rate': {
      en: 'USDC Rate',
      lg: 'Omuwendo gwa USDC',
      sw: 'Bei ya USDC'
    },
    'buy_usdc': {
      en: 'Buy USDC',
      lg: 'Gula USDC',
      sw: 'Nunua USDC'
    },
    'sell_usdc': {
      en: 'Sell USDC',
      lg: 'Tunda USDC',
      sw: 'Uza USDC'
    },
    'send_usdc': {
      en: 'Send USDC',
      lg: 'Wereza USDC',
      sw: 'Tuma USDC'
    },
    'current_rate': {
      en: 'Current Rate',
      lg: 'Omuwendo gwa kati',
      sw: 'Bei ya sasa'
    },
    'last_updated': {
      en: 'Last Updated',
      lg: 'Oluvannyuma olw\'okusemba',
      sw: 'Imesasishwa mwisho'
    },
    'error_retrieving_rate': {
      en: 'Error retrieving rate. Please try again later.',
      lg: 'Kiremya mu kufuna omuwendo. Gezaako oluvannyuma.',
      sw: 'Kosa katika kupata bei. Tafadhali jaribu tena baadaye.'
    },
    'local_currency_menu': {
      en: 'Local Currency',
      lg: 'Ssente z\'omu Uganda',
      sw: 'Sarafu ya Ndani'
    },
    'deposit': {
      en: 'Deposit',
      lg: 'Teeka Ssente',
      sw: 'Weka Pesa'
    },
    'transactions': {
      en: 'Transactions',
      lg: 'Ebyafaayo',
      sw: 'Miamala'
    },
    'recent_transactions': {
      en: 'Recent Transactions',
      lg: 'Ebyafaayo Ebisembayo',
      sw: 'Miamala ya Hivi Karibuni'
    },
    'find_agent': {
      en: 'Find Agent',
      lg: 'Noonya Agent',
      sw: 'Tafuta Wakala'
    },
    'view_proposals': {
      en: 'View Proposals',
      lg: 'Laba Ebirowoozo',
      sw: 'Angalia Mapendekezo'
    },
    'active_proposals': {
      en: 'Active Proposals',
      lg: 'Ebirowoozo Ebikola',
      sw: 'Mapendekezo Yanayofanya Kazi'
    },
    'my_voting_power': {
      en: 'My Voting Power',
      lg: 'Amaanyi Gange ag\'Okulonda',
      sw: 'Nguvu Yangu ya Kupiga Kura'
    },
    'active_votes': {
      en: 'Active Votes',
      lg: 'Okulonda Okukola',
      sw: 'Kura Zinazofanya Kazi'
    },
    'registration_failed': {
      en: 'Registration failed. Please try again later.',
      lg: 'Okwewandiisa kulemeddwa. Gezaako oluvannyuma.',
      sw: 'Usajili umeshindwa. Tafadhali jaribu tena baadaye.'
    },
    'verification_failed': {
      en: 'Verification process error. Please try again.',
      lg: 'Kiremya mu kukakasa. Gezaako nate.',
      sw: 'Kosa katika uthibitisho. Tafadhali jaribu tena baadaye.'
    },
    'welcome_afritokeni': {
      en: 'Welcome to AfriTokeni!',
      lg: 'Tukusanyukidde ku AfriTokeni!',
      sw: 'Karibu AfriTokeni!'
    },
    'welcome_back': {
      en: 'Welcome back to AfriTokeni USSD Service',
      lg: 'Tukusanyukidde nate ku mpeereza ya AfriTokeni USSD',
      sw: 'Karibu tena kwa huduma ya AfriTokeni USSD'
    },
    'pin_set_success': {
      en: 'PIN set successfully!',
      lg: 'PIN etegekeddwa bulungi!',
      sw: 'PIN imewekwa kwa mafanikio!'
    },
    'pins_no_match': {
      en: 'PINs do not match',
      lg: 'PIN tezikwatagana',
      sw: 'PIN hazifanani'
    },
    'confirm_pin': {
      en: 'Please confirm your PIN by entering it again',
      lg: 'Kakasa PIN yo ng\'oyingiza nate',
      sw: 'Tafadhali thibitisha PIN yako kwa kuiweka tena'
    },
    'error_saving_pin': {
      en: 'Error saving PIN. Please try again.',
      lg: 'Kiremya mu kutereka PIN. Gezaako nate.',
      sw: 'Kosa katika kuhifadhi PIN. Tafadhali jaribu tena.'
    },
    'enter_exactly_4_digits': {
      en: 'Please enter exactly 4 digits',
      lg: 'Yingiza ennamba 4 zokka',
      sw: 'Tafadhali weka nambari 4 tu'
    },
    
    // Transaction quotes
    'purchase_quote': {
      en: 'Purchase Quote',
      lg: 'Eky\'okugula',
      sw: 'Nukuu ya Ununuzi'
    },
    'sale_quote': {
      en: 'Sale Quote',
      lg: 'Eky\'okutunda',
      sw: 'Nukuu ya Mauzo'
    },
    'spend': {
      en: 'Spend',
      lg: 'Saasaanya',
      sw: 'Tumia'
    },
    'receive': {
      en: 'Receive',
      lg: 'Funa',
      sw: 'Pokea'
    },
    'sell': {
      en: 'Sell',
      lg: 'Tunda',
      sw: 'Uza'
    },
    'gross': {
      en: 'Gross',
      lg: 'Omugatte',
      sw: 'Jumla'
    },
    'net': {
      en: 'Net',
      lg: 'Ekyasigadde',
      sw: 'Safi'
    },
    
    // Agent finder
    'find_agent_title': {
      en: 'Find Agent',
      lg: 'Noonya Agent',
      sw: 'Tafuta Wakala'
    },
    'available_agents': {
      en: 'Available agents near you',
      lg: 'Ba-agent abali kumpi nawe',
      sw: 'Mawakala wanaopatikana karibu nawe'
    },
    'agent_details': {
      en: 'Agent Details',
      lg: 'Ebikwata ku Agent',
      sw: 'Maelezo ya Wakala'
    },
    'location': {
      en: 'Location',
      lg: 'Ekifo',
      sw: 'Mahali'
    },
    'address': {
      en: 'Address',
      lg: 'Endagiriro',
      sw: 'Anwani'
    },
    'services': {
      en: 'Services',
      lg: 'Empeereza',
      sw: 'Huduma'
    },
    'deposit_money': {
      en: 'Deposit money',
      lg: 'Teeka ssente',
      sw: 'Weka pesa'
    },
    'withdraw_money': {
      en: 'Withdraw money',
      lg: 'Ggya ssente',
      sw: 'Ondoa pesa'
    },
    'buy_sell_bitcoin': {
      en: 'Buy/Sell Bitcoin',
      lg: 'Gula/Tunda Bitcoin',
      sw: 'Nunua/Uza Bitcoin'
    },
    'visit_agent': {
      en: 'Visit the agent at their location for assistance',
      lg: 'Kyalira agent ku kifo kye okufuna obuyambi',
      sw: 'Tembelea wakala mahali pake kwa msaada'
    },
    'for_directions': {
      en: 'For directions or to contact agents directly, visit them at their listed locations',
      lg: 'Okufuna ekkubo oba okutumira ba-agent, bakyalire ku bifo byabwe',
      sw: 'Kwa maelekezo au kuwasiliana na mawakala moja kwa moja, watembelee mahali walipoorodheshwa'
    },
    'select_an_agent': {
      en: 'Select an agent',
      lg: 'Londa agent',
      sw: 'Chagua wakala'
    },
    'please_try_again_later': {
      en: 'Please try again later',
      lg: 'Gezaako oluvannyuma',
      sw: 'Tafadhali jaribu tena baadaye'
    },
    'source': {
      en: 'Source',
      lg: 'Ensibuko',
      sw: 'Chanzo'
    },
    'rates_include_fees': {
      en: 'Rates include platform fees',
      lg: 'Emiwendo girimu ssente z\'omutimbagano',
      sw: 'Bei zinajumuisha ada za jukwaa'
    },
    'instant_transfers': {
      en: 'provides instant transfers with minimal fees',
      lg: 'ewa okuweereza okw\'amangu n\'essente ntono',
      sw: 'inatoa uhamisho wa haraka na ada ndogo'
    },
    'your_ckbtc_balance': {
      en: 'Your ckBTC Balance',
      lg: 'Ssente zo za ckBTC',
      sw: 'Salio yako ya ckBTC'
    },
    'bitcoin_exchange_rate': {
      en: 'Bitcoin Exchange Rate',
      lg: 'Omuwendo gwa Bitcoin',
      sw: 'Bei ya Kubadilishana Bitcoin'
    },
    'buy_sell_spreads': {
      en: 'Buy/Sell spreads may apply',
      lg: 'Enjawulo y\'okugula/okutunda esobola okubaawo',
      sw: 'Tofauti za ununuzi/mauzo zinaweza kutumika'
    },
    'selected_agent': {
      en: 'Selected Agent',
      lg: 'Agent alondeddwa',
      sw: 'Wakala Aliyechaguliwa'
    },
    'purchase_details': {
      en: 'Purchase Details',
      lg: 'Ebikwata ku Kugula',
      sw: 'Maelezo ya Ununuzi'
    },
    'enter_pin_to_confirm': {
      en: 'Enter your PIN to confirm',
      lg: 'Yingiza PIN yo okukakasa',
      sw: 'Weka PIN yako kuthibitisha'
    },
    'purchase_initiated': {
      en: 'Purchase Initiated!',
      lg: 'Okugula kutandise!',
      sw: 'Ununuzi Umeanzishwa!'
    },
    'purchase_code': {
      en: 'Purchase Code',
      lg: 'Koodi y\'okugula',
      sw: 'Nambari ya Ununuzi'
    },
    'transaction_id': {
      en: 'Transaction ID',
      lg: 'ID y\'ensaasaanya',
      sw: 'Kitambulisho cha Muamala'
    },
    'you_will_receive': {
      en: 'You will receive',
      lg: 'Ojja kufuna',
      sw: 'Utapokea'
    },
    'amount_to_pay': {
      en: 'Amount to pay',
      lg: 'Omuwendo ogw\'okusasula',
      sw: 'Kiasi cha kulipa'
    },
    'meet_agent_with_code': {
      en: 'Meet your selected agent with this code',
      lg: 'Sisinkana ne agent wo n\'okoodi eno',
      sw: 'Kutana na wakala wako na nambari hii'
    },
    'no_ckbtc_available': {
      en: 'No ckBTC available to sell',
      lg: 'Tewali ckBTC ezitundibwa',
      sw: 'Hakuna ckBTC inayopatikana kuuza'
    },
    'sell_ckbtc': {
      en: 'Sell ckBTC',
      lg: 'Tunda ckBTC',
      sw: 'Uza ckBTC'
    },
    'btc_to_sell': {
      en: 'BTC to sell',
      lg: 'BTC ezitundibwa',
      sw: 'BTC ya kuuza'
    },
    'selling': {
      en: 'Selling',
      lg: 'Otunda',
      sw: 'Unauza'
    },
    'sale_code': {
      en: 'Sale Code',
      lg: 'Koodi y\'okutunda',
      sw: 'Nambari ya Mauzo'
    },
    'sale_initiated': {
      en: 'Sale Initiated!',
      lg: 'Okutunda kutandise!',
      sw: 'Mauzo Yameanzishwa!'
    },
    'give_code_to_agent': {
      en: 'Give this code to the agent to complete your sale and collect cash',
      lg: 'Wa agent koodi eno okumaliriza okutunda kwo n\'okufuna ssente',
      sw: 'Mpe wakala nambari hii kukamilisha mauzo yako na kupokea pesa'
    },
    'sms_sent': {
      en: 'SMS sent with details',
      lg: 'SMS eweereddwa n\'ebikwata ku nsaasaanya',
      sw: 'SMS imetumwa na maelezo'
    },
    'code': {
      en: 'Code',
      lg: 'Koodi',
      sw: 'Nambari'
    },
    'ckbtc_to_receive': {
      en: 'ckBTC to receive',
      lg: 'ckBTC gy\'ojja kufuna',
      sw: 'ckBTC utakayopokea'
    },
    'give_code_and_payment': {
      en: 'Give this code and payment to the agent to complete your ckBTC purchase',
      lg: 'Wa agent koodi eno n\'essente okumaliriza okugula kwo ckBTC',
      sw: 'Mpe wakala nambari hii na malipo kukamilisha ununuzi wako wa ckBTC'
    },
    'ckbtc_balance': {
      en: 'ckBTC Balance',
      lg: 'Ssente za ckBTC',
      sw: 'Salio ya ckBTC'
    },
    'you_need_ckbtc': {
      en: 'You need ckBTC to send. Please buy some first',
      lg: 'Weetaaga ckBTC okuweereza. Gula ezisooka',
      sw: 'Unahitaji ckBTC kutuma. Tafadhali nunua kwanza'
    },
    'total_needed': {
      en: 'Total needed',
      lg: 'Omugatte ogwetaagisa',
      sw: 'Jumla inayohitajika'
    },
    'insufficient_for_fee': {
      en: 'Insufficient balance for amount + network fee!',
      lg: 'Ssente tezimala omuwendo n\'essente z\'omutimbagano!',
      sw: 'Salio haitoshi kwa kiasi + ada ya mtandao!'
    },
    'name': {
      en: 'Name',
      lg: 'Erinnya',
      sw: 'Jina'
    },
    'sent': {
      en: 'Sent',
      lg: 'Eweereddwa',
      sw: 'Imetumwa'
    },
    'received': {
      en: 'Received',
      lg: 'Efuniddwa',
      sw: 'Imepokelewa'
    },
    'from': {
      en: 'From',
      lg: 'Okuva',
      sw: 'Kutoka'
    },
    'new_balance_updated': {
      en: 'Your new balance will be updated shortly',
      lg: 'Ssente zo empya zijja kukyusibwa mu bbanga ttono',
      sw: 'Salio yako mpya itasasishwa hivi karibuni'
    },
    'sms_notifications_sent': {
      en: 'SMS notifications sent to both parties',
      lg: 'SMS ziweereddwa eri bombi',
      sw: 'Arifa za SMS zimetumwa kwa pande zote mbili'
    },
    'btc_sent_successfully': {
      en: 'BTC Sent Successfully!',
      lg: 'BTC eweereddwa bulungi!',
      sw: 'BTC Imetumwa kwa Mafanikio!'
    },
    'btc_received': {
      en: 'BTC Received',
      lg: 'BTC Efuniddwa',
      sw: 'BTC Imepokelewa'
    },
    'check_balance_dial': {
      en: 'Check your balance by dialing',
      lg: 'Kebera ssente zo ng\'okuba',
      sw: 'Angalia salio yako kwa kupiga'
    },
    'format': {
      en: 'Format',
      lg: 'Enkola',
      sw: 'Muundo'
    },
    'invalid_phone_format': {
      en: 'Invalid phone number format',
      lg: 'Enkola y\'enamba ya ssimu si ntuufu',
      sw: 'Muundo wa nambari ya simu si sahihi'
    },
    'ensure_account': {
      en: 'Please ensure they have an AfriTokeni account',
      lg: 'Kakasa nti alina akawunti ya AfriTokeni',
      sw: 'Tafadhali hakikisha ana akaunti ya AfriTokeni'
    },
    'cannot_send_to_self': {
      en: 'Cannot send to your own number',
      lg: 'Tosobola kuweereza ku namba yo',
      sw: 'Huwezi kutuma kwa nambari yako mwenyewe'
    },
    'time': {
      en: 'Time',
      lg: 'Obudde',
      sw: 'Wakati'
    },
    'sms_confirmations_sent': {
      en: 'SMS confirmations sent',
      lg: 'SMS z\'okukakasa ziweereddwa',
      sw: 'Uthibitisho wa SMS umetumwa'
    },
    'too_many_attempts': {
      en: 'Too many incorrect PIN attempts',
      lg: 'Okugezaako PIN embi emirundi mingi',
      sw: 'Majaribio mengi ya PIN isiyo sahihi'
    },
    'attempts_remaining': {
      en: 'attempts remaining',
      lg: 'okugezaako okusigadde',
      sw: 'majaribio yaliyobaki'
    },
    'money_sent_successfully': {
      en: 'Money sent successfully!',
      lg: 'Ssente ziweereddwa bulungi!',
      sw: 'Pesa zimetumwa kwa mafanikio!'
    },
    'you_received_money': {
      en: 'You received money!',
      lg: 'Ofunye ssente!',
      sw: 'Umepokea pesa!'
    },
    'reference': {
      en: 'Reference',
      lg: 'Refelensi',
      sw: 'Kumbukumbu'
    },
    'withdrawal': {
      en: 'Withdrawal',
      lg: 'Okusasula',
      sw: 'Uondoaji'
    },
    'valid': {
      en: 'Valid',
      lg: 'Kikola',
      sw: 'Halali'
    },
    'hours': {
      en: 'hours',
      lg: 'essaawa',
      sw: 'masaa'
    },
    'show_code_to_agent': {
      en: 'Show this code to the agent with your ID to collect cash',
      lg: 'Laga agent koodi eno n\'ekiraga naawe okufuna ssente',
      sw: 'Onyeshe wakala nambari hii na kitambulisho chako kupokea pesa'
    },
    'withdrawal_created': {
      en: 'Withdrawal Created!',
      lg: 'Okusasula kutondeddwa!',
      sw: 'Uondoaji Umeundwa!'
    },
    'try_again': {
      en: 'Try again',
      lg: 'Gezaako nate',
      sw: 'Jaribu tena'
    },
    'please_try_again': {
      en: 'Please try again',
      lg: 'Gezaako nate',
      sw: 'Tafadhali jaribu tena'
    },
    'already_voted': {
      en: 'Already voted on this proposal!',
      lg: 'Omaze okulonda ku kirowoozo kino!',
      sw: 'Umeshapiga kura kwenye pendekezo hili!'
    },
    'your_vote': {
      en: 'Your vote',
      lg: 'Okulonda kwo',
      sw: 'Kura yako'
    },
    'thank_you_governance': {
      en: 'Thank you for participating in AfriTokeni governance!',
      lg: 'Webale okwetaba mu kufuga AfriTokeni!',
      sw: 'Asante kwa kushiriki katika utawala wa AfriTokeni!'
    },
    'voting_ends_in': {
      en: 'Voting ends in',
      lg: 'Okulonda kuggwawo mu',
      sw: 'Kupiga kura kunaisha ndani ya'
    },
    'days': {
      en: 'days',
      lg: 'ennaku',
      sw: 'siku'
    },
    'current_votes': {
      en: 'Current votes',
      lg: 'Okulonda kwa kati',
      sw: 'Kura za sasa'
    },
    'vote_yes': {
      en: 'Vote YES',
      lg: 'Londa YE',
      sw: 'Piga kura NDIO'
    },
    'vote_no': {
      en: 'Vote NO',
      lg: 'Londa NEDDA',
      sw: 'Piga kura HAPANA'
    },
    'vote_abstain': {
      en: 'Vote ABSTAIN',
      lg: 'Londa NZIYIZA',
      sw: 'Piga kura ZUIA'
    },
    'vote': {
      en: 'Vote',
      lg: 'Londa',
      sw: 'Piga kura'
    },
    'enter_vote_amount': {
      en: 'Enter amount to vote (min 1 AFRI)',
      lg: 'Yingiza omuwendo ogw\'okulonda (omutono 1 AFRI)',
      sw: 'Weka kiasi cha kupiga kura (chini 1 AFRI)'
    },
    'insufficient_tokens': {
      en: 'Insufficient tokens!',
      lg: 'Tokens tezimala!',
      sw: 'Tokens hazitoshi!'
    },
    'requested': {
      en: 'Requested',
      lg: 'Esabiddwa',
      sw: 'Imeombwa'
    },
    'confirm_vote': {
      en: 'Confirm Vote',
      lg: 'Kakasa Okulonda',
      sw: 'Thibitisha Kura'
    },
    'proposal': {
      en: 'Proposal',
      lg: 'Ekirowoozo',
      sw: 'Pendekezo'
    },
    'vote_successful': {
      en: 'Vote Successful!',
      lg: 'Okulonda kuwangudde!',
      sw: 'Kura Imefanikiwa!'
    },
    'voted': {
      en: 'Voted',
      lg: 'Olonze',
      sw: 'Umepiga kura'
    },
    'vote_locked_message': {
      en: 'Your vote is locked until proposal ends.',
      lg: 'Okulonda kwo kuziyiddwa okutuusa ekirowoozo lwe kiggwawo.',
      sw: 'Kura yako imefungwa hadi pendekezo liishe.'
    },
    'your_voting_power': {
      en: 'Your Voting Power',
      lg: 'Amaanyi Go ag\'Okulonda',
      sw: 'Nguvu Yako ya Kupiga Kura'
    },
    'locked_tokens_message': {
      en: 'Locked tokens released when proposals end.',
      lg: 'Tokens eziziyiddwa zisumulukibwa ekirowoozo lwe kiggwawo.',
      sw: 'Tokens zilizofungwa zitatolewa mapendekezo yatakapoisha.'
    },
    'your_active_votes': {
      en: 'Your Active Votes',
      lg: 'Okulonda Kwo Okukola',
      sw: 'Kura Zako Zinazofanya Kazi'
    },
    'no_active_votes': {
      en: 'You have no active votes.',
      lg: 'Tolina kulonda kukola.',
      sw: 'Huna kura zinazofanya kazi.'
    },
    'vote_to_participate': {
      en: 'Vote on proposals to participate in governance!',
      lg: 'Londa ku birowoozo okwetaba mu kufuga!',
      sw: 'Piga kura kwenye mapendekezo kushiriki katika utawala!'
    },
    'total_locked': {
      en: 'Total locked',
      lg: 'Omugatte oguziyiddwa',
      sw: 'Jumla iliyofungwa'
    },
    'btc_purchase_quote': {
      en: 'Bitcoin Purchase Quote',
      lg: 'Eky\'okugula Bitcoin',
      sw: 'Nukuu ya Ununuzi wa Bitcoin'
    },
    'btc_sale_quote': {
      en: 'Bitcoin Sale Quote',
      lg: 'Eky\'okutunda Bitcoin',
      sw: 'Nukuu ya Mauzo ya Bitcoin'
    },
    'recipient': {
      en: 'Recipient',
      lg: 'Omufuna',
      sw: 'Mpokeaji'
    },
    'sale_details': {
      en: 'Sale Details',
      lg: 'Ebikwata ku Kutunda',
      sw: 'Maelezo ya Mauzo'
    },
    'verification_code_sent': {
      en: 'We\'ve sent a verification code to your phone.',
      lg: 'Tuweerezza koodi y\'okukakasa ku ssimu yo.',
      sw: 'Tumetuma nambari ya uthibitisho kwa simu yako.'
    },
    'enter_verification_code': {
      en: 'Please enter the 6-digit code',
      lg: 'Yingiza koodi y\'ennamba 6',
      sw: 'Weka nambari ya nambari 6'
    },
    'invalid_code_format': {
      en: 'Invalid code format',
      lg: 'Enkola ya koodi si ntuufu',
      sw: 'Muundo wa nambari si sahihi'
    },
    'invalid_verification_code': {
      en: 'Invalid verification code',
      lg: 'Koodi y\'okukakasa si ntuufu',
      sw: 'Nambari ya uthibitisho si sahihi'
    },
    'setup_pin_message': {
      en: 'To secure your account, please set up a 4-digit PIN',
      lg: 'Okukuuma akawunti yo, tegeka PIN ya namba 4',
      sw: 'Kulinda akaunti yako, weka PIN ya nambari 4'
    },
    'enter_new_pin': {
      en: 'Enter your new PIN',
      lg: 'Yingiza PIN yo empya',
      sw: 'Weka PIN yako mpya'
    },
    'invalid_name': {
      en: 'Invalid name',
      lg: 'Erinnya si lituufu',
      sw: 'Jina si sahihi'
    },
    'enter_full_name': {
      en: 'Please enter your full name (first and last name)',
      lg: 'Yingiza erinnya lyo lyonna (erinnya ly\'omu maaso n\'ery\'oluvannyuma)',
      sw: 'Weka jina lako kamili (jina la kwanza na la mwisho)'
    },
    'verification_successful': {
      en: 'Verification successful',
      lg: 'Okukakasa kugenze bulungi',
      sw: 'Uthibitisho umefanikiwa'
    },
    'account_created': {
      en: 'Account created successfully',
      lg: 'Akawunti etondeddwa bulungi',
      sw: 'Akaunti imeundwa kwa mafanikio'
    },
    'not_registered_yet': {
      en: 'You are not registered yet',
      lg: 'Tonnawandiisibwa',
      sw: 'Bado hujajiandikisha'
    },
    'please_wait': {
      en: 'Please wait',
      lg: 'Lindawo',
      sw: 'Tafadhali subiri'
    },
    'enter_both_names': {
      en: 'Please enter both first and last name separated by space',
      lg: 'Yingiza erinnya ly\'omu maaso n\'ery\'oluvannyuma nga biriko ekifo wakati',
      sw: 'Weka jina la kwanza na la mwisho likitengwa na nafasi'
    },
    'coming_soon': {
      en: 'coming soon',
      lg: 'ejja mu biseera ebitali bya wala',
      sw: 'inakuja hivi karibuni'
    },
    'vote_on_proposals': {
      en: 'Vote on proposals',
      lg: 'Londa ku byogerwa',
      sw: 'Piga kura kwenye mapendekezo'
    },
    'for_support': {
      en: 'For support',
      lg: 'Okufuna obuyambi',
      sw: 'Kwa msaada'
    },
    'call': {
      en: 'Call',
      lg: 'Kuba',
      sw: 'Piga simu'
    },
    'visit': {
      en: 'Visit',
      lg: 'Kyalira',
      sw: 'Tembelea'
    },
    'no_transactions': {
      en: 'No transactions found',
      lg: 'Tewali nsimbi ezisindikiddwa',
      sw: 'Hakuna miamala iliyopatikana'
    },
    'to_start_using': {
      en: 'To start using AfriTokeni, send money or make a deposit through an agent',
      lg: 'Okutandika okukozesa AfriTokeni, wereza ssente oba teeka ssente ng\'oyita mu agent',
      sw: 'Ili kuanza kutumia AfriTokeni, tuma pesa au fanya amana kupitia wakala'
    },
    'last': {
      en: 'Last',
      lg: 'Ezisembayo',
      sw: 'Za mwisho'
    },
    'your_account_balance': {
      en: 'Your Account Balance',
      lg: 'Ssente zo ku Akawunti',
      sw: 'Salio yako ya Akaunti'
    },
    'error_retrieving_history': {
      en: 'Error retrieving transaction history.',
      lg: 'Kiremya mu kufuna ebyafaayo by\'ensimbi.',
      sw: 'Kosa katika kupata historia ya muamala.'
    },
    'thank_you_using_afritokeni': {
      en: 'Thank you for using AfriTokeni!',
      lg: 'Webale okukozesa AfriTokeni!',
      sw: 'Asante kwa kutumia AfriTokeni!'
    },
  };

  /**
   * Get translation for a key
   */
  static translate(key: string, language: Language = 'en'): string {
    const translation = this.translations[key];
    if (!translation) {
      console.warn(`Translation missing for key: ${key}`);
      return key;
    }
    return translation[language] || translation.en;
  }

  /**
   * Get translation with variable substitution
   */
  static translateWithVars(
    key: string,
    vars: Record<string, string | number>,
    language: Language = 'en'
  ): string {
    let text = this.translate(key, language);
    
    Object.entries(vars).forEach(([varKey, value]) => {
      text = text.replace(`{${varKey}}`, String(value));
    });
    
    return text;
  }

  /**
   * Detect language from phone number or user preference
   */
  static async detectLanguage(phoneNumber: string): Promise<Language> {
    // Default to English
    // In production, this would check user preferences in database
    // For now, we can detect by country code:
    // +256 (Uganda) -> Luganda preferred
    // +254 (Kenya) -> Swahili preferred
    // +255 (Tanzania) -> Swahili preferred
    
    if (phoneNumber.startsWith('+256')) {
      return 'lg'; // Luganda for Uganda
    } else if (phoneNumber.startsWith('+254') || phoneNumber.startsWith('+255')) {
      return 'sw'; // Swahili for Kenya/Tanzania
    }
    
    return 'en'; // Default to English
  }

  /**
   * Format amount with currency in local language
   */
  static formatAmount(amount: number, currency: string, language: Language = 'en'): string {
    const formatted = amount.toLocaleString();
    
    switch (language) {
      case 'lg':
        return `${formatted} ${currency}`;
      case 'sw':
        return `${currency} ${formatted}`;
      default:
        return `${formatted} ${currency}`;
    }
  }

  /**
   * Get all supported languages
   */
  static getSupportedLanguages(): { code: Language; name: string; nativeName: string }[] {
    return [
      { code: 'en', name: 'English', nativeName: 'English' },
      { code: 'lg', name: 'Luganda', nativeName: 'Luganda' },
      { code: 'sw', name: 'Swahili', nativeName: 'Kiswahili' }
    ];
  }

  /**
   * Format demo transaction messages
   */
  static getDemoMessage(type: 'btc_buy' | 'btc_sell' | 'btc_send' | 'usdc_buy' | 'usdc_sell' | 'usdc_send' | 'btc_balance' | 'usdc_balance', lang: Language, data: any): string {
    const messages = {
      btc_buy: {
        en: `Bitcoin Purchase Successful! (Demo)\n\nAmount: {currency} {amount}\nReceived: ₿{btc} BTC\n\nMeet your selected agent to complete the transaction.\n\nThank you for using AfriTokeni!`,
        lg: `Okugula Bitcoin Kuwangudde! (Demo)\n\nOmuwendo: {currency} {amount}\nOfunye: ₿{btc} BTC\n\nSisinkana ne agent wo okumaliriza.\n\nWebale okukozesa AfriTokeni!`,
        sw: `Ununuzi wa Bitcoin Umefanikiwa! (Demo)\n\nKiasi: {currency} {amount}\nUmepokea: ₿{btc} BTC\n\nKutana na wakala wako kukamilisha.\n\nAsante kwa kutumia AfriTokeni!`
      },
      btc_sell: {
        en: `Bitcoin Sale Successful! (Demo)\n\nSold: ₿{btc} BTC\nReceived: {currency} {amount}\n\nMeet your selected agent to receive cash.\n\nThank you for using AfriTokeni!`,
        lg: `Okutunda Bitcoin Kuwangudde! (Demo)\n\nOtunze: ₿{btc} BTC\nOfunye: {currency} {amount}\n\nSisinkana ne agent wo okufuna ssente.\n\nWebale okukozesa AfriTokeni!`,
        sw: `Mauzo ya Bitcoin Yamefanikiwa! (Demo)\n\nUmeuz: ₿{btc} BTC\nUmepokea: {currency} {amount}\n\nKutana na wakala wako kupokea pesa.\n\nAsante kwa kutumia AfriTokeni!`
      },
      btc_send: {
        en: `Bitcoin Sent Successfully! (Demo)\n\nAmount: ₿{btc} BTC\nTo: {phone}\n\nTransaction complete!\n\nThank you for using AfriTokeni!`,
        lg: `Okuweereza Bitcoin Kuwangudde! (Demo)\n\nOmuwendo: ₿{btc} BTC\nEri: {phone}\n\nOmulimu guwedde!\n\nWebale okukozesa AfriTokeni!`,
        sw: `Bitcoin Imetumwa! (Demo)\n\nKiasi: ₿{btc} BTC\n\nKwa: {phone}\n\nMuamala umekamilika!\n\nAsante kwa kutumia AfriTokeni!`
      },
      usdc_buy: {
        en: `USDC Purchase Successful! (Demo)\n\nAmount: {currency} {amount}\nReceived: ${data.usdc} USDC\n\nMeet your selected agent to complete the transaction.\n\nThank you for using AfriTokeni!`,
        lg: `Okugula USDC Kuwangudde! (Demo)\n\nOmuwendo: {currency} {amount}\nOfunye: ${data.usdc} USDC\n\nSisinkana ne agent wo okumaliriza.\n\nWebale okukozesa AfriTokeni!`,
        sw: `Ununuzi wa USDC Umefanikiwa! (Demo)\n\nKiasi: {currency} {amount}\nUmepokea: ${data.usdc} USDC\n\nKutana na wakala wako kukamilisha.\n\nAsante kwa kutumia AfriTokeni!`
      },
      usdc_sell: {
        en: `USDC Sale Successful! (Demo)\n\nSold: ${data.usdc} USDC\nReceived: {currency} {amount}\n\nMeet your selected agent to receive cash.\n\nThank you for using AfriTokeni!`,
        lg: `Okutunda USDC Kuwangudde! (Demo)\n\nOtunze: ${data.usdc} USDC\nOfunye: {currency} {amount}\n\nSisinkana ne agent wo okufuna ssente.\n\nWebale okukozesa AfriTokeni!`,
        sw: `Mauzo ya USDC Yamefanikiwa! (Demo)\n\nUmeuz: ${data.usdc} USDC\nUmepokea: {currency} {amount}\n\nKutana na wakala wako kupokea pesa.\n\nAsante kwa kutumia AfriTokeni!`
      },
      usdc_send: {
        en: `USDC Sent Successfully! (Demo)\n\nAmount: ${data.usdc} USDC\nTo: {phone}\n\nTransaction complete!\n\nThank you for using AfriTokeni!`,
        lg: `Okuweereza USDC Kuwangudde! (Demo)\n\nOmuwendo: ${data.usdc} USDC\nEri: {phone}\n\nOmulimu guwedde!\n\nWebale okukozesa AfriTokeni!`,
        sw: `USDC Imetumwa! (Demo)\n\nKiasi: ${data.usdc} USDC\nKwa: {phone}\n\nMuamala umekamilika!\n\nAsante kwa kutumia AfriTokeni!`
      },
      btc_balance: {
        en: `Your ckBTC Balance (Demo)\n\n₿{btc} BTC\n≈ {currency} {amount}\n\nckBTC provides instant Bitcoin transfers with minimal fees on the Internet Computer blockchain.\n\nThank you for using AfriTokeni!`,
        lg: `Obugagga bwo bwa ckBTC (Demo)\n\n₿{btc} BTC\n≈ {currency} {amount}\n\nckBTC ekuwa okuweereza Bitcoin amangu n'emisolo mitono.\n\nWebale okukozesa AfriTokeni!`,
        sw: `Salio lako la ckBTC (Demo)\n\n₿{btc} BTC\n≈ {currency} {amount}\n\nckBTC inatoa uhamisho wa haraka wa Bitcoin na ada ndogo.\n\nAsante kwa kutumia AfriTokeni!`
      },
      usdc_balance: {
        en: `Your USDC Balance (Demo)\n\n${data.usdc} USDC\n≈ {currency} {amount}\n\nCurrent Rate: 1 USDC = {currency} 3,800\n\nThank you for using AfriTokeni!`,
        lg: `Obugagga bwo bwa USDC (Demo)\n\n${data.usdc} USDC\n≈ {currency} {amount}\n\nOmuwendo gwa kati: 1 USDC = {currency} 3,800\n\nWebale okukozesa AfriTokeni!`,
        sw: `Salio lako la USDC (Demo)\n\n${data.usdc} USDC\n≈ {currency} {amount}\n\nBei ya sasa: 1 USDC = {currency} 3,800\n\nAsante kwa kutumia AfriTokeni!`
      }
    };

    let message = messages[type][lang];
    // Replace placeholders
    if (data.currency) message = message.replace(/{currency}/g, data.currency);
    if (data.amount) message = message.replace(/{amount}/g, data.amount);
    if (data.btc) message = message.replace(/{btc}/g, data.btc);
    if (data.phone) message = message.replace(/{phone}/g, data.phone);
    
    return message;
  }
}
