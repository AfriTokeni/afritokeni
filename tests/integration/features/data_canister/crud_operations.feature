Feature: Data Canister CRUD Operations
  As a Business Logic Canister
  I want to perform CRUD operations on user data
  So that I can manage user information and balances

  Background:
    Given the Data Canister is deployed
    And the Business Logic Canister is authorized

  Scenario: Create and retrieve user
    When I create a user with:
      | field              | value              |
      | phone_number       | +256700000001      |
      | first_name         | Alice              |
      | last_name          | Nakato             |
      | email              | alice@example.com  |
      | preferred_currency | UGX                |
    Then the user should be created successfully
    When I get user by phone "+256700000001"
    Then the user should exist
    And the user's first name should be "Alice"
    And the user's preferred currency should be "UGX"

  Scenario: Set and get fiat balance
    Given user "Alice" with phone "+256700000001" exists
    When I set Alice's UGX balance to 100000
    Then the balance should be set successfully
    When I get Alice's UGX balance
    Then the balance should be 100000

  Scenario: Update crypto balance
    Given user "Alice" with phone "+256700000001" exists
    And Alice has 0 BTC and 0 USDC
    When I update Alice's crypto balance by +100000000 satoshis BTC
    Then the update should succeed
    When I get Alice's crypto balance
    Then Alice should have 100000000 satoshis BTC
    And Alice should have 0 USDC

  Scenario: Store and retrieve transaction
    Given user "Alice" with phone "+256700000001" exists
    When I store a transaction:
      | field            | value                |
      | id               | tx_123456            |
      | transaction_type | transfer_fiat        |
      | from_user        | +256700000001        |
      | to_user          | +256700000002        |
      | amount           | 10000                |
      | currency         | UGX                  |
      | status           | completed            |
    Then the transaction should be stored successfully
    When I get transactions for user "+256700000001"
    Then the transaction "tx_123456" should be in the list

  Scenario: Verify PIN
    Given user "Alice" with phone "+256700000001" exists
    And Alice has PIN "1234" set up
    When I verify PIN "1234" for Alice
    Then the verification should succeed
    When I verify PIN "9999" for Alice
    Then the verification should fail

  Scenario: Unauthorized canister cannot access data
    Given an unauthorized canister tries to access data
    When the unauthorized canister tries to get user data
    Then the call should be rejected with "Unauthorized"
    When the unauthorized canister tries to set balance
    Then the call should be rejected with "Unauthorized"

  Scenario: Data canister logs all operations
    Given user "Alice" with phone "+256700000001" exists
    When I set Alice's UGX balance to 100000
    Then an audit entry should be created in the data canister
    And the audit entry should contain the operation details
