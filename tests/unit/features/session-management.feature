Feature: Session Management
  As a USSD system
  I want to manage user sessions properly
  So that conversations are stateful and secure

  Scenario: Create new session
    Given a phone number "+256700123456"
    When I create a new session
    Then the session should have a unique ID
    And the session language should be "en"
    And the session step should be 0
    And the session data should be empty

  Scenario: Session expiry after 5 minutes
    Given an active session
    When 6 minutes pass
    Then the session should be expired

  Scenario: Session not expired within 5 minutes
    Given an active session
    When 4 minutes pass
    Then the session should not be expired

  Scenario: Store data in session
    Given an active session
    When I store "recipient" as "+256700999999"
    And I store "amount" as "10000"
    Then I should be able to retrieve "recipient"
    And I should be able to retrieve "amount"

  Scenario: Retrieve non-existent data
    Given an active session
    When I try to retrieve "nonexistent"
    Then the result should be None

  Scenario: Track current menu
    Given an active session
    When I navigate to "send_money" menu
    Then the current menu should be "send_money"

  Scenario: Track step in flow
    Given an active session in "send_money" menu
    When I advance to step 2
    Then the step should be 2

  Scenario: Change language
    Given an active session with language "en"
    When I change language to "lg"
    Then the session language should be "lg"

  Scenario: Update activity timestamp
    Given an active session
    When I update activity
    Then the last activity timestamp should be updated

  Scenario: PIN verification state
    Given an active session
    Then PIN should not be verified
    When I mark PIN as verified
    Then PIN should be verified
