Feature: USSD USDC Menu Navigation and Operations
  As a USSD user
  I want to access USDC features via USSD
  So that I can check balances and rates with proper navigation

  Background:
    Given I have a phone number "+256700123456"
    And I have set my PIN to "1234"

  Scenario: Access USDC menu from main menu
    When I dial "*229#"
    And I select "3" for USDC
    Then I should see "USDC (ckUSDC)" in USSD response
    And I should see "1. Check Balance" in USSD response
    And I should see "2. USDC Rate" in USSD response
    And I should see "3. Buy USDC" in USSD response
    And I should see "4. Sell USDC" in USSD response
    And I should see "5. Send USDC" in USSD response
    And I should see "0. Back to Main Menu" in USSD response

  Scenario: Check USDC balance with PIN
    When I dial "*229#"
    And I select "3" for USDC
    And I select "1" for Check Balance
    Then I should see "Enter your 4-digit PIN" in USSD response
    When I enter PIN "1234"
    Then I should see "Your USDC Balance" in USSD response

  Scenario: Cancel USDC balance check with 0
    When I dial "*229#"
    And I select "3" for USDC
    And I select "1" for Check Balance
    Then I should see "Enter your 4-digit PIN" in USSD response
    When I select "0" to go back
    Then I should see "USDC (ckUSDC)" in USSD response
    And I should see "1. Check Balance" in USSD response

  Scenario: Show USDC menu with 9 during balance check
    When I dial "*229#"
    And I select "3" for USDC
    And I select "1" for Check Balance
    Then I should see "Enter your 4-digit PIN" in USSD response
    When I select "9" to show current menu
    Then I should see "USDC (ckUSDC)" in USSD response
    And I should see "1. Check Balance" in USSD response

  Scenario: Check USDC rate without PIN
    When I dial "*229#"
    And I select "3" for USDC
    And I select "2" for USDC Rate
    Then I should see "Current USDC Exchange Rate" in USSD response
    And I should see "1 USDC" in USSD response

  Scenario: Navigate back to main menu from USDC menu
    When I dial "*229#"
    And I select "3" for USDC
    Then I should see "USDC (ckUSDC)" in USSD response
    When I select "0" to go back
    Then I should see "Welcome to AfriTokeni" in USSD response
    And I should see "3. USDC" in USSD response

  Scenario: Invalid PIN shows error without navigation
    When I dial "*229#"
    And I select "3" for USDC
    And I select "1" for Check Balance
    When I enter PIN "9999"
    Then I should see "Incorrect PIN" in USSD response
    And I should see "Enter your 4-digit PIN" in USSD response

  Scenario: Invalid PIN format shows error without navigation
    When I dial "*229#"
    And I select "3" for USDC
    And I select "1" for Check Balance
    When I enter PIN "123"
    Then I should see "Invalid PIN format" in USSD response
    And I should see "Enter your 4-digit PIN" in USSD response

  Scenario: Chained input for USDC balance check
    When I dial "*229#"
    And I enter chained input "3*1*1234"
    Then I should see "Your USDC Balance" in USSD response

  Scenario: Chained input for USDC rate check
    When I dial "*229#"
    And I enter chained input "3*2"
    Then I should see "Current USDC Exchange Rate" in USSD response

  Scenario: Complete USDC buy flow end-to-end
    When I dial "*229#"
    And I select "3" for USDC
    And I select "3" for Buy USDC
    Then I should see "Enter your 4-digit PIN" in USSD response
    When I enter PIN "1234"
    Then I should see "Buy USDC" in USSD response
    And I should see "Enter amount" in USSD response
    When I enter "10000"
    Then I should see "USDC Purchase Quote" in USSD response
    And I should see "Select an agent" in USSD response
    When I select "1" for first agent
    Then I should see "Purchase Initiated" in USSD response
    And I should see "Transaction ID" in USSD response
    And I should see "Code:" in USSD response

  Scenario: Complete USDC sell flow end-to-end
    When I dial "*229#"
    And I select "3" for USDC
    And I select "4" for Sell USDC
    Then I should see "Enter your 4-digit PIN" in USSD response
    When I enter PIN "1234"
    Then I should see "Sell USDC" in USSD response
    And I should see "Your balance" in USSD response
    And I should see "Enter amount" in USSD response
    When I enter "10"
    Then I should see "USDC Sale Quote" in USSD response
    And I should see "Select an agent" in USSD response
    When I select "1" for first agent
    Then I should see "Sale Initiated" in USSD response
    And I should see "Transaction ID" in USSD response
    And I should see "Code:" in USSD response

  Scenario: Complete USDC send flow end-to-end
    When I dial "*229#"
    And I select "3" for USDC
    And I select "5" for Send USDC
    Then I should see "Send USDC" in USSD response
    And I should see "Enter your 4-digit PIN" in USSD response
    When I enter PIN "1234"
    Then I should see "Send USDC" in USSD response
    And I should see "Enter recipient phone number" in USSD response
    When I enter "+256700123457"
    Then I should see "Recipient" in USSD response
    And I should see "Your balance" in USSD response
    And I should see "Enter amount" in USSD response
    When I enter "5"
    Then I should see "Confirm" in USSD response
    And I should see "5" in USSD response
    And I should see "+256700123457" in USSD response
    When I enter "1"
    Then I should see "Transaction ID" in USSD response
    And I should see "Thank you" in USSD response

  Scenario: USDC buy with insufficient balance fails gracefully
    When I dial "*229#"
    And I select "3" for USDC
    And I select "3" for Buy USDC
    When I enter PIN "1234"
    And I enter "999999999"
    Then I should see "Insufficient balance" in USSD response

  Scenario: USDC sell with insufficient USDC fails gracefully
    When I dial "*229#"
    And I select "3" for USDC
    And I select "4" for Sell USDC
    When I enter PIN "1234"
    And I enter "999999"
    Then I should see "Insufficient balance" in USSD response

  Scenario: USDC send with invalid phone number fails
    When I dial "*229#"
    And I select "3" for USDC
    And I select "5" for Send USDC
    When I enter PIN "1234"
    And I enter "invalid"
    Then I should see "Invalid phone" in USSD response

  # CRITICAL: AfricasTalking cumulative input format tests
  # AfricasTalking sends cumulative text: "3" → "3*4" → "3*4*1234"
  # But handlers receive full string, so "3*4*1234" has 6 digits at end, not 4
  
  Scenario: USDC sell with AfricasTalking cumulative input format
    When I dial "*229#"
    And I enter chained input "3*4*1234"
    Then I should see "Sell USDC" in USSD response
    And I should see "Your balance" in USSD response
    And I should see "Enter amount" in USSD response
    And I should not see "Invalid PIN format" in USSD response
    And I should not see "Enter your 4-digit PIN" in USSD response

  Scenario: USDC buy with AfricasTalking cumulative input format
    When I dial "*229#"
    And I enter chained input "3*3*1234"
    Then I should see "Buy USDC" in USSD response
    And I should see "Enter amount" in USSD response
    And I should not see "Invalid PIN format" in USSD response

  Scenario: USDC send with AfricasTalking cumulative input format
    When I dial "*229#"
    And I enter chained input "3*5*1234"
    Then I should see "Send USDC" in USSD response
    And I should see "Enter recipient phone number" in USSD response
    And I should not see "Invalid PIN format" in USSD response
