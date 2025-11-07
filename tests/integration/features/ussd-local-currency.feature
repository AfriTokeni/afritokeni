@integration
Feature: USSD Local Currency Menu
  As a user
  I want to manage my local currency (KES) via USSD
  So that I can send money, check balance, and withdraw

  Background:
    Given the local ICP replica is running
    And the USSD canister is deployed
    And I have a registered account with phone "+254700123456"
    And I have set up a PIN "1234"

  Scenario: Check KES balance
    When I dial USSD code "*384*22948#"
    And I select option "1" for Local Currency
    And I select option "1" for Check Balance
    Then I should see my KES balance
    And I should see my ckBTC balance
    And I should see my ckUSDC balance

  Scenario: Send money successfully
    Given I have 1000 KES in my account
    When I dial USSD code "*384*22948#"
    And I select option "1" for Local Currency
    And I select option "2" for Send Money
    And I enter recipient phone "+254700999888"
    And I enter amount "100"
    And I enter PIN "1234"
    Then I should see "Transaction successful"
    And 100 KES should be deducted from my balance

  Scenario: Send money with invalid phone
    When I dial USSD code "*384*22948#"
    And I select option "1" for Local Currency
    And I select option "2" for Send Money
    And I enter recipient phone "invalid"
    Then I should see "Invalid phone"

  Scenario: Send money with insufficient balance
    Given I have 50 KES in my account
    When I dial USSD code "*384*22948#"
    And I select option "1" for Local Currency
    And I select option "2" for Send Money
    And I enter recipient phone "+254700999888"
    And I enter amount "100"
    And I enter PIN "1234"
    Then I should see "Insufficient balance"

  Scenario: Send money with wrong PIN
    Given I have 1000 KES in my account
    When I dial USSD code "*384*22948#"
    And I select option "1" for Local Currency
    And I select option "2" for Send Money
    And I enter recipient phone "+254700999888"
    And I enter amount "100"
    And I enter PIN "9999"
    Then I should see "Incorrect PIN"

  Scenario: Send money - PIN lockout after 3 failed attempts
    Given I have 1000 KES in my account
    When I dial USSD code "*384*22948#"
    And I select option "1" for Local Currency
    And I select option "2" for Send Money
    And I enter recipient phone "+254700999888"
    And I enter amount "100"
    And I enter wrong PIN 3 times
    Then I should see "Too many failed attempts"
    And I should be locked out for 15 minutes

  Scenario: Withdraw cash successfully
    Given I have 1000 KES in my account
    When I dial USSD code "*384*22948#"
    And I select option "1" for Local Currency
    And I select option "4" for Withdraw
    And I enter amount "500"
    And I enter PIN "1234"
    Then I should see "Transaction successful"
    And I should see "Receive cash"
    And 500 KES should be deducted from my balance

  Scenario: Withdraw with amount below minimum
    When I dial USSD code "*384*22948#"
    And I select option "1" for Local Currency
    And I select option "4" for Withdraw
    And I enter amount "5"
    Then I should see "Amount too small"
    And I should see "Minimum is 10 KES"

  Scenario: Withdraw with amount above maximum
    When I dial USSD code "*384*22948#"
    And I select option "1" for Local Currency
    And I select option "4" for Withdraw
    And I enter amount "2000000"
    Then I should see "Amount too large"
    And I should see "Maximum is 1000000 KES"
