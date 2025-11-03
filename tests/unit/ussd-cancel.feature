Feature: USSD Cancel/Go Back Functionality
  As a feature phone user
  I want to cancel any operation by pressing 0
  So that I can go back if I make a mistake

  Background:
    Given I am a registered USSD user with balance

  Scenario: Cancel Send Money at phone number entry
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "1" for Send Money
    And I select "0" to cancel
    Then I should see "Local Currency" in USSD response
    And I should see "1. Send Money" in USSD response

  Scenario: Cancel Send Money at amount entry
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "1" for Send Money
    And I enter phone number "256700123456"
    And I select "0" to cancel
    Then I should see "Local Currency" in USSD response
    And I should see "1. Send Money" in USSD response

  Scenario: Cancel Send Money at PIN entry
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "1" for Send Money
    And I enter phone number "256700123456"
    And I enter amount "1000"
    And I select "0" to cancel
    Then I should see "Transaction cancelled" in USSD response
    And the session should end

  Scenario: Press 0 shows cancel hint on non-numeric prompts
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "1" for Send Money
    Then I should see "0. Back" in USSD response
    When I enter phone number "256700123456"
    Then I should see "amount" in USSD response

  Scenario: Cancel works in Luganda
    Given my language preference is "lg"
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "1" for Send Money
    Then I should see "Ddayo" in USSD response
    When I select "0" to cancel
    Then I should see "Ssente z'omu Uganda" in USSD response

  Scenario: Cancel works in Swahili
    Given my language preference is "sw"
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "1" for Send Money
    Then I should see "Rudi" in USSD response
    When I select "0" to cancel
    Then I should see "Sarafu ya Ndani" in USSD response

  Scenario: Invalid phone shows cancel option
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "1" for Send Money
    And I enter phone number "123"
    Then I should see "Invalid phone" in USSD response
    And I should see "0. Back" in USSD response

  Scenario: Multiple cancel attempts work
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "1" for Send Money
    And I select "0" to cancel
    And I select "1" for Send Money
    And I select "0" to cancel
    Then I should see "Local Currency" in USSD response
