@integration
Feature: USSD USDC Menu
  As a user
  I want to manage my USDC (ckUSDC) via USSD
  So that I can buy, send, and check my USDC balance

  Background:
    Given the local ICP replica is running
    And the USSD canister is deployed
    And I have a registered account with phone "+254700123456"
    And I have set up a PIN "1234"

  Scenario: Check USDC balance
    When I dial USSD code "*384*22948#"
    And I select option "3" for USDC
    And I select option "1" for Check Balance
    Then I should see my ckUSDC balance

  Scenario: Buy USDC successfully
    Given I have 20000 KES in my account
    When I dial USSD code "*384*22948#"
    And I select option "3" for USDC
    And I select option "2" for Buy USDC
    And I enter amount "15000" KES
    And I enter PIN "1234"
    Then I should see "Transaction successful"
    And I should see "Bought" and ckUSDC amount
    And 15000 KES should be deducted from my balance
    And ckUSDC should be added to my balance

  Scenario: Buy USDC with insufficient KES
    Given I have 500 KES in my account
    When I dial USSD code "*384*22948#"
    And I select option "3" for USDC
    And I select option "2" for Buy USDC
    And I enter amount "10000" KES
    And I enter PIN "1234"
    Then I should see "Insufficient balance"

  Scenario: Buy USDC with wrong PIN
    Given I have 20000 KES in my account
    When I dial USSD code "*384*22948#"
    And I select option "3" for USDC
    And I select option "2" for Buy USDC
    And I enter amount "15000" KES
    And I enter PIN "9999"
    Then I should see "Incorrect PIN"

  Scenario: Buy USDC with amount below minimum
    When I dial USSD code "*384*22948#"
    And I select option "3" for USDC
    And I select option "2" for Buy USDC
    And I enter amount "5" KES
    Then I should see "Amount too small"

  Scenario: Buy USDC with amount above maximum
    When I dial USSD code "*384*22948#"
    And I select option "3" for USDC
    And I select option "2" for Buy USDC
    And I enter amount "2000000" KES
    Then I should see "Amount too large"
