@integration
Feature: USSD Bitcoin Menu
  As a user
  I want to manage my Bitcoin (ckBTC) via USSD
  So that I can buy, send, and check my Bitcoin balance

  Background:
    Given the local ICP replica is running
    And the USSD canister is deployed
    And I have a registered account with phone "+254700123456"
    And I have set up a PIN "1234"

  Scenario: Check Bitcoin balance
    When I dial USSD code "*384*22948#"
    And I select option "2" for Bitcoin
    And I select option "1" for Check Balance
    Then I should see my ckBTC balance

  Scenario: Buy Bitcoin successfully
    Given I have 100000 KES in my account
    When I dial USSD code "*384*22948#"
    And I select option "2" for Bitcoin
    And I select option "2" for Buy Bitcoin
    And I enter amount "50000" KES
    And I enter PIN "1234"
    Then I should see "Transaction successful"
    And I should see "Bought" and ckBTC amount
    And 50000 KES should be deducted from my balance
    And ckBTC should be added to my balance

  Scenario: Buy Bitcoin with insufficient KES
    Given I have 1000 KES in my account
    When I dial USSD code "*384*22948#"
    And I select option "2" for Bitcoin
    And I select option "2" for Buy Bitcoin
    And I enter amount "50000" KES
    And I enter PIN "1234"
    Then I should see "Insufficient balance"

  Scenario: Buy Bitcoin with wrong PIN
    Given I have 100000 KES in my account
    When I dial USSD code "*384*22948#"
    And I select option "2" for Bitcoin
    And I select option "2" for Buy Bitcoin
    And I enter amount "50000" KES
    And I enter PIN "9999"
    Then I should see "Incorrect PIN"

  Scenario: Send Bitcoin successfully
    Given I have 0.001 ckBTC in my account
    When I dial USSD code "*384*22948#"
    And I select option "2" for Bitcoin
    And I select option "4" for Send Bitcoin
    And I enter BTC address "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
    And I enter amount "0.0005" ckBTC
    And I enter PIN "1234"
    Then I should see "Transaction successful"
    And 0.0005 ckBTC should be deducted from my balance

  Scenario: Send Bitcoin with invalid address
    When I dial USSD code "*384*22948#"
    And I select option "2" for Bitcoin
    And I select option "4" for Send Bitcoin
    And I enter BTC address "invalid_address"
    Then I should see "Invalid BTC address"

  Scenario: Send Bitcoin with insufficient balance
    Given I have 0.0001 ckBTC in my account
    When I dial USSD code "*384*22948#"
    And I select option "2" for Bitcoin
    And I select option "4" for Send Bitcoin
    And I enter BTC address "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
    And I enter amount "0.001" ckBTC
    And I enter PIN "1234"
    Then I should see "Insufficient balance"
