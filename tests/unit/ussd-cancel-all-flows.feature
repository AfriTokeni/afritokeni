Feature: Cancel Functionality Across All USSD Flows
  As a feature phone user
  I want to cancel any multi-step operation by pressing 0
  So that I can go back if I make a mistake or change my mind

  Background:
    Given I am a registered USSD user with balance

  # ==================== SEND MONEY ====================
  Scenario: Cancel Send Money at phone entry (English)
    Given I am a registered USSD user with balance
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "1" for Send Money
    Then I should see "0. Back | 9. Menu" in USSD response
    When I select "0" to cancel
    Then I should see "Local Currency" in USSD response

  Scenario: Cancel Send Money at amount entry (Luganda)
    Given my language preference is "lg"
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "1" for Send Money
    And I enter phone number "256700123456"
    Then I should see "omuwendo" in USSD response
    When I select "0" to cancel
    Then I should see "Ssente z'omu Uganda" in USSD response

  Scenario: Cancel Send Money at PIN entry (Swahili)
    Given my language preference is "sw"
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "1" for Send Money
    And I enter phone number "256700123456"
    And I enter amount "1000"
    Then I should see "PIN" in USSD response
    When I select "0" to cancel
    Then I should see "Muamala umeshindwa" in USSD response

  # ==================== WITHDRAW ====================
  Scenario: Cancel Withdraw at amount entry (English)
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "4" for Withdraw
    Then I should see "amount" in USSD response
    When I select "0" to cancel
    Then I should see "Local Currency" in USSD response

  Scenario: Cancel Withdraw at agent selection (Luganda)
    Given my language preference is "lg"
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "4" for Withdraw
    And I enter amount "5000"
    Then I should see "agent" in USSD response
    When I select "0" to cancel
    Then I should see "Ssente z'omu Uganda" in USSD response

  Scenario: Cancel Withdraw at PIN entry (Swahili)
    Given my language preference is "sw"
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "4" for Withdraw
    And I enter amount "5000"
    And I select "1" for first agent
    Then I should see "PIN" in USSD response
    When I select "0" to cancel
    Then I should see "Muamala umeshindwa" in USSD response

  # ==================== DEPOSIT ====================
  Scenario: Cancel Deposit at amount entry (English)
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "3" for Deposit
    Then I should see "amount" in USSD response
    When I select "0" to cancel
    Then I should see "Local Currency" in USSD response

  Scenario: Cancel Deposit at agent selection (Luganda)
    Given my language preference is "lg"
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "3" for Deposit
    And I enter amount "10000"
    Then I should see "agent" in USSD response
    When I select "0" to cancel
    Then I should see "Ssente z'omu Uganda" in USSD response

  # ==================== BITCOIN BUY ====================
  Scenario: Cancel Bitcoin Buy at amount entry (English)
    When I dial "*229#"
    And I select "2" for Bitcoin
    And I select "3" for Buy Bitcoin
    Then I should see "PIN" in USSD response
    When I enter PIN "1234"
    Then I should see "amount" in USSD response
    When I select "0" to cancel
    Then I should see "Bitcoin" in USSD response

  # ==================== BITCOIN SELL ====================
  Scenario: Cancel Bitcoin Sell at amount entry (Luganda)
    Given my language preference is "lg"
    When I dial "*229#"
    And I select "2" for Bitcoin
    And I select "4" for Sell Bitcoin
    Then I should see "PIN" in USSD response
    When I enter PIN "1234"
    Then I should see "omuwendo" in USSD response
    When I select "0" to cancel
    Then I should see "Bitcoin" in USSD response

  # ==================== BITCOIN SEND ====================
  Scenario: Cancel Bitcoin Send at PIN entry (English)
    When I dial "*229#"
    And I select "2" for Bitcoin
    And I select "5" for Send Bitcoin
    Then I should see "PIN" in USSD response
    When I select "0" to cancel
    Then I should see "Bitcoin" in USSD response

  # ==================== MULTIPLE CANCELS ====================
  Scenario: Multiple consecutive cancels work correctly
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "1" for Send Money
    And I select "0" to cancel
    And I select "4" for Withdraw
    And I select "0" to cancel
    And I select "3" for Deposit
    And I select "0" to cancel
    Then I should see "Local Currency" in USSD response

  # ==================== CANCEL FROM ERRORS ====================
  Scenario: Cancel works after validation error
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "1" for Send Money
    And I enter phone number "123"
    Then I should see "Invalid phone" in USSD response
    And I should see "0. Back" in USSD response
    When I select "0" to cancel
    Then I should see "Local Currency" in USSD response
