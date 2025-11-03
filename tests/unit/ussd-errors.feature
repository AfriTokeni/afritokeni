@integration
Feature: USSD Error Handling
  Test error scenarios and edge cases

  Scenario: Insufficient balance for send money
    Given I have 5000 UGX balance
    When I try to send 10000 UGX via USSD for error test
    Then I should see "Insufficient balance" in USSD response
    And the transaction should not be created

  Scenario: Insufficient balance for withdrawal
    Given I have 5000 UGX balance
    When I try to withdraw 10000 UGX via USSD for error test
    Then I should see "Insufficient balance" in USSD response

  Scenario: Invalid phone number format
    Given I am in send money menu
    When I enter invalid phone "12345"
    Then I should see "Invalid phone number" in USSD response

  Scenario: Negative amount rejected
    Given I am in send money menu
    When I enter negative amount "-1000"
    Then I should see "Invalid amount" in USSD response

  Scenario: Zero amount rejected
    Given I am in send money menu
    When I enter zero amount "0"
    Then I should see "Invalid amount" in USSD response

  Scenario: Non-numeric amount rejected
    Given I am in send money menu
    When I enter non-numeric amount "abc"
    Then I should see "Invalid amount" in USSD response

  Scenario: Recipient not found
    Given I am in send money menu
    When I enter non-existent recipient "+256700000000"
    Then I should see "Recipient not found" in USSD response

  Scenario: Agent not available
    Given I am in withdraw menu
    When I select unavailable agent
    Then I should see "Agent not available" in USSD response

  Scenario: Session expired handling
    Given I have an expired USSD session
    When I try to continue the session
    Then I should see "Session expired" in USSD response
    And I should see "Please dial *384*22948# again" in USSD response

  Scenario: Maximum amount exceeded
    Given I am in send money menu
    When I enter amount exceeding limit "10000000"
    Then I should see "Amount exceeds maximum" in USSD response

  Scenario: Minimum amount not met
    Given I am in send money menu
    When I enter amount below minimum "10"
    Then I should see "Minimum amount" in USSD response

  Scenario: Service temporarily unavailable
    Given the balance service is down
    When I try to check balance
    Then I should see "Service temporarily unavailable" in USSD response
    And the session should end gracefully
