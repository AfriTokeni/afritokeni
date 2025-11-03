@integration
Feature: USSD Handler Logic
  Test individual USSD handler implementations

  Background:
    Given I am a registered USSD user with balance

  # Main Menu Handler Tests
  Scenario: Main menu shows all options
    When I access the main menu
    Then I should see "Local Currency" in USSD response
    And I should see "Bitcoin" in USSD response
    And I should see option "1" for local currency
    And I should see option "2" for Bitcoin

  Scenario: Main menu routes to local currency
    When I access the main menu
    And I select option "1"
    Then the session menu should be "local_currency"
    And I should see "Send Money" in USSD response

  Scenario: Main menu routes to Bitcoin
    When I access the main menu
    And I select option "2"
    Then the session menu should be "bitcoin"
    And I should see "Bitcoin" in USSD response

  # Local Currency Handler Tests
  Scenario: Local currency menu shows all options
    Given I am in local currency menu
    Then I should see "Send Money" in USSD response
    And I should see "Check Balance" in USSD response
    And I should see "Deposit" in USSD response
    And I should see "Withdraw" in USSD response

  Scenario: Local currency routes to send money
    Given I am in local currency menu
    When I select option "1"
    Then the session menu should be "send_money"
    And I should see "Enter recipient phone number" in USSD response

  Scenario: Local currency routes to check balance
    Given I am in local currency menu
    When I select option "2"
    Then I should see my balance in response

  Scenario: Local currency routes to deposit
    Given I am in local currency menu
    When I select option "3"
    Then the session menu should be "deposit"
    And I should see "Enter amount" in USSD response

  Scenario: Withdraw menu shows amount prompt
    Given I am in local currency menu
    When I select option "4"
    Then the session menu should be "withdraw"
    And I should see "Enter amount" in USSD response

  # Bitcoin Handler Tests
  Scenario: Bitcoin menu shows all options
    Given I am in bitcoin menu
    Then I should see "Check Balance" in USSD response
    And I should see "Buy Bitcoin" in USSD response
    And I should see "Sell Bitcoin" in USSD response

  Scenario: Check Bitcoin balance
    Given I am in bitcoin menu
    When I select option "1"
    And I enter PIN "1234"
    Then I should see "ckBTC Balance" in USSD response

  Scenario: Bitcoin routes to rate check
    Given I am in bitcoin menu
    When I select option "2"
    Then I should see "Bitcoin Exchange Rate" in USSD response

  # Agent Handler Tests
  Scenario: Find agent shows available agents
    Given I am in find agent menu
    Then I should see "Available agents" in USSD response
    And I should see agent options

  # Send Money Handler Tests
  Scenario: Send money requests phone number
    Given I am in send money menu
    Then I should see "Enter recipient" in USSD response

  Scenario: Send money validates phone number
    Given I am in send money menu
    When I enter phone number "+256700123456"
    Then I should see "Enter amount" in USSD response

  # Withdraw Handler Tests
  Scenario: Withdraw requests amount
    Given I am in withdraw menu
    Then I should see "Enter amount" in USSD response

  Scenario: Withdraw validates amount
    Given I am in withdraw menu with 100000 balance
    When I enter withdraw amount "50000"
    Then I should see "Select an agent" in USSD response

  Scenario: Withdraw rejects amount exceeding balance
    Given I am in withdraw menu with 10000 balance
    When I enter withdraw amount "50000"
    Then I should see "Insufficient balance" in USSD response

  # Deposit Handler Tests
  Scenario: Deposit requests amount
    Given I am in deposit menu
    Then I should see "Enter amount" in USSD response

  Scenario: Deposit generates code
    Given I am in deposit menu
    When I enter deposit amount "50000"
    Then I should see deposit code
    And the code should start with "DEP-" in handler test
