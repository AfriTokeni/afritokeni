@integration
Feature: USSD Service Core Logic
  Test the USSDService routing and session management

  Scenario: Create new USSD session on first dial
    When I dial "*229#" as a new user
    Then a new USSD session should be created
    And the session should have menu "main"
    And I should see the main menu with options

  Scenario: Main menu displays correctly
    When I dial "*229#" as a new user
    Then I should see "AfriTokeni" in USSD response
    And I should see option "1" for local currency
    And I should see option "2" for Bitcoin

  Scenario: Session retrieval works
    Given I have an active USSD session
    When I retrieve the session by ID
    Then the session should exist
    And the session should have my phone number

  Scenario: Session update works
    Given I have an active USSD session
    When I update the session menu to "bitcoin"
    And I retrieve the session by ID
    Then the session menu should be "bitcoin"

  Scenario: Invalid menu selection shows error
    Given I have an active USSD session
    When I select option "99"
    Then I should see "Invalid" in USSD response
    And the session should continue

  Scenario: Empty input shows main menu
    Given I have an active USSD session
    When I send empty input
    Then I should see the main menu with options

  Scenario: Rate limiting prevents spam
    Given I have made 15 USSD requests in 1 minute
    When I dial "*229#" again
    Then I should see "Too many requests" in USSD response
    And the session should end
