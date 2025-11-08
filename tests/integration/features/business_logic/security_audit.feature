Feature: Security and Audit in Business Logic Canister
  As a system administrator
  I want strict access control and comprehensive audit logging
  So that the system is secure and all operations are traceable

  Background:
    Given the Business Logic Canister is deployed
    And the Data Canister is deployed

  Scenario: Only authorized canisters can call business logic
    Given the USSD Canister is authorized
    And the Web Canister is authorized
    And a random canister is NOT authorized
    When the USSD Canister calls "transfer_money"
    Then the call should succeed
    When the Web Canister calls "transfer_money"
    Then the call should succeed
    When the random canister calls "transfer_money"
    Then the call should be rejected with "Unauthorized caller"

  Scenario: Controller can add authorized canisters
    Given I am the controller
    When I add canister "aaaaa-aa" to authorized list
    Then the canister should be added successfully
    And an audit entry should be created for "add_authorized_canister"
    When I list authorized canisters
    Then "aaaaa-aa" should be in the list

  Scenario: Non-controller cannot add authorized canisters
    Given I am NOT the controller
    When I attempt to add canister "aaaaa-aa" to authorized list
    Then the call should be rejected with "Only controller can add authorized canisters"

  Scenario: Controller can remove authorized canisters
    Given I am the controller
    And canister "aaaaa-aa" is authorized
    When I remove canister "aaaaa-aa" from authorized list
    Then the canister should be removed successfully
    And an audit entry should be created for "remove_authorized_canister"
    When I list authorized canisters
    Then "aaaaa-aa" should NOT be in the list

  Scenario: All operations are audited
    Given user "Alice" with phone "+256700000001" exists
    And Alice has 100000 UGX balance
    And Alice has PIN "1234"
    When Alice transfers 10000 UGX to Bob with PIN "1234"
    Then an audit entry should exist with:
      | field      | value            |
      | action     | transfer_money   |
      | user_id    | +256700000001    |
      | success    | true             |
    And the audit entry should have a timestamp
    And the audit entry should have caller information

  Scenario: Failed operations are also audited
    Given user "Alice" with phone "+256700000001" exists
    And Alice has 100000 UGX balance
    And Alice has PIN "1234"
    When Alice transfers 10000 UGX to Bob with PIN "9999"
    Then an audit entry should exist with:
      | field      | value            |
      | action     | transfer_money   |
      | user_id    | +256700000001    |
      | success    | false            |
    And the audit entry details should contain "Invalid PIN"

  Scenario: Audit log is queryable by controller only
    Given I am the controller
    And there are 50 audit entries
    When I query the audit log with limit 10 and offset 0
    Then I should receive 10 audit entries
    When I query the audit log count
    Then the count should be 50

  Scenario: Non-controller cannot access audit log
    Given I am NOT the controller
    When I attempt to query the audit log
    Then the call should be rejected with "Only controller can access audit log"

  Scenario: Audit log has retention limit
    Given there are 10000 audit entries
    When a new audit entry is created
    Then the oldest entry should be removed
    And the total count should remain 10000
