Feature: Money Transfer via Business Logic Canister
  As a user of AfriTokeni
  I want to transfer money to other users
  So that I can send money securely with proper validation and audit

  Background:
    Given the Business Logic Canister is deployed
    And the Data Canister is deployed
    And the USSD Canister is authorized to call Business Logic
    And user "Alice" with phone "+256700000001" has 100000 UGX balance
    And user "Bob" with phone "+256700000002" has 50000 UGX balance
    And Alice has set up PIN "1234"

  Scenario: Successful money transfer
    When Alice transfers 10000 UGX to Bob with PIN "1234"
    Then the transfer should succeed
    And Alice's balance should be 90000 UGX
    And Bob's balance should be 60000 UGX
    And an audit entry should be created for "transfer_money"
    And the audit entry should show success as true

  Scenario: Transfer with insufficient balance
    When Alice transfers 150000 UGX to Bob with PIN "1234"
    Then the transfer should fail with "Insufficient balance"
    And Alice's balance should remain 100000 UGX
    And Bob's balance should remain 50000 UGX
    And an audit entry should be created for "transfer_money"
    And the audit entry should show success as false

  Scenario: Transfer with incorrect PIN
    When Alice transfers 10000 UGX to Bob with PIN "9999"
    Then the transfer should fail with "Invalid PIN"
    And Alice's balance should remain 100000 UGX
    And Bob's balance should remain 50000 UGX
    And an audit entry should be created for "transfer_money"
    And the audit entry should show success as false

  Scenario: Transfer exceeding fraud limit
    When Alice transfers 15000000 UGX to Bob with PIN "1234"
    Then the transfer should fail with "Transaction blocked"
    And Alice's balance should remain 100000 UGX
    And Bob's balance should remain 50000 UGX
    And an audit entry should be created for "transfer_money"
    And the audit entry should show success as false

  Scenario: Transfer to non-existent user
    When Alice transfers 10000 UGX to phone "+256700999999" with PIN "1234"
    Then the transfer should fail with "User not found"
    And Alice's balance should remain 100000 UGX
    And an audit entry should be created for "transfer_money"
    And the audit entry should show success as false

  Scenario: Unauthorized canister attempting transfer
    Given an unauthorized canister tries to call Business Logic
    When the unauthorized canister attempts a transfer
    Then the call should be rejected with "Unauthorized caller"
    And no audit entry should be created
