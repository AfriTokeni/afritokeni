@e2e
Feature: End-to-End Exchange Flow
  As a user
  I want to exchange between ckBTC and ckUSD
  So that I can manage my crypto portfolio

  Background:
    Given the exchange canister is deployed
    And I am a registered user with phone number "+256700123456"
    And I have 0.01 ckBTC balance
    And I have 500 ckUSD balance

  Scenario: Exchange ckBTC to ckUSD
    When I request to exchange 0.005 ckBTC to ckUSD
    Then the exchange canister should calculate the rate
    And the spread should be 0.5%
    And I should receive approximately 300 ckUSD
    And the platform should earn 1.5 ckUSD spread
    And the exchange should be recorded on-chain

  Scenario: Exchange ckUSD to ckBTC
    When I request to exchange 300 ckUSD to ckBTC
    Then the exchange canister should calculate the rate
    And the spread should be 0.5%
    And I should receive approximately 0.00497 ckBTC
    And the platform should earn 0.000015 ckBTC spread
    And the exchange should be recorded on-chain

  Scenario: Exchange with minimum amount
    When I request to exchange 0.0001 ckBTC to ckUSD
    Then the exchange should succeed
    And the spread calculation should be accurate
    And no precision should be lost

  Scenario: Exchange with maximum amount
    When I request to exchange 10 ckBTC to ckUSD
    Then the exchange should succeed
    And the spread should never exceed input amount
    And the calculation should handle large numbers

  Scenario: Same token exchange should fail
    When I request to exchange ckBTC to ckBTC
    Then the exchange should be rejected
    And I should see "Cannot exchange same token"

  Scenario: Zero amount exchange should fail
    When I request to exchange 0 ckBTC to ckUSD
    Then the exchange should be rejected
    And I should see "Amount must be greater than zero"

  Scenario: Exchange revenue tracking and verification
    Given multiple users perform exchanges:
      | from_token | to_token | amount    | spread_percent | platform_earns |
      | ckBTC      | ckUSD    | 0.01      | 0.5           | 0.00005       |
      | ckUSD      | ckBTC    | 600       | 0.5           | 3             |
      | ckBTC      | ckUSD    | 0.005     | 0.5           | 0.000025      |
    When I query the exchange canister revenue
    Then the total spread earned should be exactly 0.000075 ckBTC + 3 ckUSD
    And the revenue should match sum of all exchanges
    And each exchange should be recorded separately
    And the canister should track revenue by token type
    And querying total revenue should show accurate amounts
    And the on-chain record should be immutable
    And no revenue should be missing or duplicated
