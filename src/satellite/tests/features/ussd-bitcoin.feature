Feature: USSD Bitcoin (ckBTC) Menu Navigation and Operations
  As a USSD user
  I want to access Bitcoin features via USSD
  So that I can check balances, rates, and perform transactions without Principal ID errors

  Background:
    Given I have a phone number "+256700123456"
    And I have set my PIN to "1234"

  Scenario: Access Bitcoin menu from main menu
    When I dial "*229#"
    And I select "2" for Bitcoin
    Then I should see "Bitcoin (ckBTC)" in USSD response
    And I should see "1. Check Balance" in USSD response
    And I should see "2. Bitcoin rate" in USSD response
    And I should see "3. Buy Bitcoin" in USSD response
    And I should see "4. Sell Bitcoin" in USSD response
    And I should see "5. Send Bitcoin" in USSD response
    And I should see "0. Back to Main Menu" in USSD response

  Scenario: Check Bitcoin balance with PIN
    When I dial "*229#"
    And I select "2" for Bitcoin
    And I select "1" for Check Balance
    Then I should see "Enter your 4-digit PIN" in USSD response
    When I enter PIN "1234"
    Then I should see "Your ckBTC Balance" in USSD response
    And I should see "BTC" in USSD response

  Scenario: Sell Bitcoin flow
    When I dial "*229#"
    And I select "2" for Bitcoin
    And I select "4" for Sell Bitcoin
    Then I should see "Sell Bitcoin" in USSD response
    And I should see "Enter your 4-digit PIN" in USSD response

  Scenario: Cancel Bitcoin balance check with 0
    When I dial "*229#"
    And I select "2" for Bitcoin
    And I select "1" for Check Balance
    Then I should see "Enter your 4-digit PIN" in USSD response
    When I select "0" to go back
    Then I should see "Bitcoin (ckBTC)" in USSD response
    And I should see "1. Check Balance" in USSD response

  Scenario: Show Bitcoin menu with 9 during balance check
    When I dial "*229#"
    And I select "2" for Bitcoin
    And I select "1" for Check Balance
    Then I should see "Enter your 4-digit PIN" in USSD response
    When I select "9" to show current menu
    Then I should see "Bitcoin (ckBTC)" in USSD response
    And I should see "1. Check Balance" in USSD response

  Scenario: Check Bitcoin rate without PIN
    When I dial "*229#"
    And I select "2" for Bitcoin
    And I enter chained input "2"
    Then I should see "Bitcoin Exchange Rate" in USSD response
    And I should see "1 BTC" in USSD response

  Scenario: Navigate back to main menu from Bitcoin menu
    When I dial "*229#"
    And I select "2" for Bitcoin
    Then I should see "Bitcoin (ckBTC)" in USSD response
    When I select "0" to go back
    Then I should see "Welcome to AfriTokeni" in USSD response
    And I should see "2. Bitcoin" in USSD response

  Scenario: Invalid PIN shows error without navigation
    When I dial "*229#"
    And I select "2" for Bitcoin
    And I select "1" for Check Balance
    When I enter PIN "9999"
    Then I should see "Incorrect PIN" in USSD response
    And I should see "Enter your 4-digit PIN" in USSD response

  Scenario: Invalid PIN format shows error without navigation
    When I dial "*229#"
    And I select "2" for Bitcoin
    And I select "1" for Check Balance
    When I enter PIN "123"
    Then I should see "Invalid PIN format" in USSD response
    And I should see "Enter your 4-digit PIN" in USSD response

  Scenario: Chained input for Bitcoin balance check
    When I dial "*229#"
    And I enter chained input "2*1*1234"
    Then I should see "Your ckBTC Balance" in USSD response
    And I should see "BTC" in USSD response

  Scenario: Chained input for Bitcoin rate check
    When I dial "*229#"
    And I enter chained input "2*2"
    Then I should see "Bitcoin Exchange Rate" in USSD response

  Scenario: Complete Bitcoin buy flow end-to-end
    When I dial "*229#"
    And I select "2" for Bitcoin
    And I select "3" for Buy Bitcoin
    Then I should see "Enter your 4-digit PIN" in USSD response
    When I enter PIN "1234"
    Then I should see "Buy Bitcoin" in USSD response
    And I should see "Enter amount" in USSD response
    When I enter "50000"
    Then I should see "Bitcoin Purchase Quote" in USSD response
    And I should see "Select an agent" in USSD response
    When I select "1" for first agent
    Then I should see "Purchase Initiated" in USSD response
    And I should see "Transaction ID" in USSD response
    And I should see "Code:" in USSD response

  Scenario: Complete Bitcoin sell flow end-to-end
    When I dial "*229#"
    And I select "2" for Bitcoin
    And I select "4" for Sell Bitcoin
    Then I should see "Enter your 4-digit PIN" in USSD response
    When I enter PIN "1234"
    Then I should see "Sell Bitcoin" in USSD response
    And I should see "Your balance" in USSD response
    And I should see "Enter BTC amount" in USSD response
    When I enter "0.0001"
    Then I should see "Bitcoin Sale Quote" in USSD response
    And I should see "Select an agent" in USSD response
    When I select "1" for first agent
    Then I should see "Sale Initiated" in USSD response
    And I should see "Transaction ID" in USSD response
    And I should see "Code:" in USSD response

  Scenario: Complete Bitcoin send flow end-to-end
    When I dial "*229#"
    And I select "2" for Bitcoin
    And I select "5" for Send Bitcoin
    Then I should see "Send Bitcoin" in USSD response
    And I should see "Enter your 4-digit PIN" in USSD response
    When I enter PIN "1234"
    Then I should see "Send Bitcoin" in USSD response
    And I should see "Your balance" in USSD response
    And I should see "Enter recipient phone number" in USSD response
    When I enter "+256700123457"
    Then I should see "Enter BTC amount" in USSD response
    When I enter "0.0001"
    Then I should see "Send Summary" in USSD response
    And I should see "0.0001" in USSD response
    And I should see "+256700123457" in USSD response
    When I enter "1"
    Then I should see "Transaction ID" in USSD response
    And I should see "Thank you" in USSD response

  Scenario: Bitcoin buy with insufficient balance fails gracefully
    When I dial "*229#"
    And I select "2" for Bitcoin
    And I select "3" for Buy Bitcoin
    When I enter PIN "1234"
    And I enter "999999999"
    Then I should see "Insufficient balance" in USSD response

  Scenario: Bitcoin sell with insufficient BTC fails gracefully
    When I dial "*229#"
    And I select "2" for Bitcoin
    And I select "4" for Sell Bitcoin
    When I enter PIN "1234"
    And I enter "999999999"
    Then I should see "Insufficient balance" in USSD response

  Scenario: Bitcoin send with invalid phone number fails
    When I dial "*229#"
    And I select "2" for Bitcoin
    And I select "5" for Send Bitcoin
    When I enter PIN "1234"
    And I enter "invalid"
    Then I should see "Invalid phone" in USSD response

  # CRITICAL: AfricasTalking cumulative input format tests
  # AfricasTalking sends cumulative text: "2" → "2*4" → "2*4*1234"
  # But handlers receive full string, so "2*4*1234" has 6 digits at end, not 4
  
  Scenario: Bitcoin sell with AfricasTalking cumulative input format
    When I dial "*229#"
    And I enter chained input "2*4*1234"
    Then I should see "Sell Bitcoin" in USSD response
    And I should see "Your balance" in USSD response
    And I should see "Enter BTC amount" in USSD response
    And I should not see "Invalid PIN format" in USSD response
    And I should not see "Enter your 4-digit PIN" in USSD response

  Scenario: Bitcoin buy with AfricasTalking cumulative input format
    When I dial "*229#"
    And I enter chained input "2*3*1234"
    Then I should see "Buy Bitcoin" in USSD response
    And I should see "Enter amount" in USSD response
    And I should not see "Invalid PIN format" in USSD response

  Scenario: Bitcoin send with AfricasTalking cumulative input format
    When I dial "*229#"
    And I enter chained input "2*5*1234"
    Then I should see "Send Bitcoin" in USSD response
    And I should see "Enter recipient phone number" in USSD response
    And I should not see "Invalid PIN format" in USSD response
