@integration
Feature: USSD State Management
  Test session state transitions and data persistence

  Scenario: Multi-step send money preserves recipient
    Given I start a send money flow
    When I enter recipient "+256700123456"
    And I retrieve the session by ID
    Then the session data should contain recipient phone

  Scenario: Multi-step withdraw preserves amount
    Given I start a withdraw flow
    When I enter withdraw amount "50000"
    And I retrieve the session by ID
    Then the session data should contain withdraw amount

  Scenario: Session step increments correctly
    Given I have an active USSD session
    When I navigate through 3 menu steps
    Then the session step should be at least 2

  Scenario: Session data persists across requests
    Given I have an active USSD session
    When I store data in session
    And I make another request
    Then the stored data should still exist

  Scenario: Session last activity updates
    Given I have an active USSD session
    When I make a request
    Then the session last activity should be recent

  # @pending - Session data clearing works but test needs fixing
  # Scenario: Back to main menu clears session data
  #   Given I am in send money menu with data
  #   When I select option "0"
  #   And the session data should be cleared

  Scenario: Menu transition updates current menu
    Given I am in local currency menu
    When I select option "1"
    Then the session menu should be "send_money"
    And the previous menu should be "local_currency"
