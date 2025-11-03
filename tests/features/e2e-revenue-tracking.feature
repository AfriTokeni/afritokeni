@e2e
Feature: End-to-End Revenue Tracking
  As the platform
  I want to verify all revenue is correctly calculated and tracked
  So that we ensure proper compensation for agents and platform earnings

  Background:
    Given all canisters are deployed
    And the revenue tracking is initialized at zero

  Scenario: Platform earns 0.5% on deposit transaction
    Given a user deposits 100000 UGX with an agent
    When the deposit is confirmed on the canister
    Then the platform should earn exactly 500 UGX
    And the deposit canister should record 500 UGX platform revenue
    And the user should receive exactly 99500 UGX
    And the canister revenue query should show 500 UGX total

  Scenario: Platform + Agent revenue on withdrawal
    Given a user withdraws 100000 UGX in urban area
    When the withdrawal is confirmed on the canister
    Then the platform fee should be exactly 500 UGX
    And the agent fee should be exactly 3000 UGX
    And the platform should take 300 UGX from agent fee
    And the agent should keep exactly 2700 UGX
    And the total platform revenue should be exactly 800 UGX
    And the withdrawal canister should record:
      | platform_fee | agent_commission | platform_cut_from_agent |
      | 500          | 2700            | 300                     |
    And the agent earnings query should show 2700 UGX

  Scenario: Exchange canister earns 0.5% spread
    Given a user exchanges 1000000 satoshis to ckUSD
    When the exchange is executed on the canister
    Then the spread should be exactly 5000 satoshis
    And the user should receive ckUSD minus 5000 satoshis equivalent
    And the exchange canister should record 5000 satoshis revenue
    And the canister revenue query should show the spread earned

  Scenario: Multiple transactions accumulate revenue correctly
    Given the following transactions occur:
      | type       | amount  | location | agent_fee |
      | deposit    | 50000   | urban    | 0         |
      | withdrawal | 100000  | urban    | 3000      |
      | withdrawal | 200000  | rural    | 10000     |
      | deposit    | 75000   | urban    | 0         |
    When I query the total platform revenue
    Then the deposit canister should show 625 UGX revenue
    And the withdrawal canister should show 2800 UGX revenue
    And the total platform earnings should be 3425 UGX
    And the agent total earnings should be 11700 UGX

  Scenario: Agent earnings are tracked per agent
    Given agent "Agent-001" completes 3 withdrawals:
      | amount  | location | agent_fee | platform_cut |
      | 100000  | urban    | 3000      | 300          |
      | 50000   | urban    | 1500      | 150          |
      | 200000  | rural    | 10000     | 1000         |
    When I query agent "Agent-001" earnings
    Then the agent should have earned exactly 13050 UGX
    And the platform should have earned 1450 UGX from this agent
    And the canister should track each transaction separately

  Scenario: Revenue tracking is immutable on-chain
    Given a withdrawal with 500 UGX platform fee is recorded
    When I attempt to modify the revenue record
    Then the modification should fail
    And the canister should reject unauthorized changes
    And the revenue record should remain 500 UGX

  Scenario: Daily revenue aggregation
    Given multiple transactions occur on 2025-11-03:
      | type       | platform_fee | agent_commission |
      | deposit    | 250          | 0                |
      | withdrawal | 500          | 2700             |
      | withdrawal | 1000         | 9000             |
      | deposit    | 375          | 0                |
    When I query daily revenue for 2025-11-03
    Then the total platform revenue should be 2575 UGX
    And the total agent earnings should be 11700 UGX
    And the canister should provide breakdown by transaction type

  Scenario: Monthly agent performance tracking
    Given agent "Agent-001" has monthly activity:
      | metric              | value  |
      | total_withdrawals   | 50     |
      | total_amount        | 5000000|
      | total_earned        | 135000 |
      | platform_cut        | 13500  |
    When I query monthly agent performance
    Then the agent net earnings should be 121500 UGX
    And the platform should have earned 13500 UGX from this agent
    And the performance metrics should be accurate

  Scenario: Zero-fee transactions don't generate revenue
    Given a user transfers 50000 UGX to another user
    And the transfer has 0% platform fee (promotional period)
    When the transfer is completed
    Then the platform revenue should be 0 UGX
    And the canister should record the transaction with zero fee
    And the revenue query should not include this transaction

  Scenario: Revenue calculation with different currencies
    Given transactions in multiple African currencies:
      | currency | amount | platform_fee_percent | expected_fee |
      | UGX      | 100000 | 0.5                 | 500          |
      | KES      | 10000  | 0.5                 | 50           |
      | NGN      | 50000  | 0.5                 | 250          |
    When all transactions are processed
    Then each currency should track revenue separately
    And the total revenue should be calculable in each currency
    And the canister should support multi-currency revenue tracking

  Scenario: Agent commission varies by location (dynamic fees)
    Given withdrawals in different locations:
      | location  | amount  | agent_fee_percent | agent_earns | platform_takes |
      | urban     | 100000  | 3.0              | 2700        | 300            |
      | rural     | 100000  | 5.0              | 4500        | 500            |
      | remote    | 100000  | 10.0             | 9000        | 1000           |
    When all withdrawals are confirmed
    Then each agent should earn according to location multiplier
    And the platform should take exactly 10% of each agent fee
    And the revenue should reflect fair compensation for remote areas

  Scenario: Revenue audit trail is complete
    Given 100 transactions have occurred
    When I query the complete revenue history
    Then every transaction should have:
      | field              | required |
      | transaction_id     | yes      |
      | timestamp          | yes      |
      | amount             | yes      |
      | platform_fee       | yes      |
      | agent_commission   | yes      |
      | platform_cut       | yes      |
    And the sum of all platform_fees should match total revenue
    And no revenue should be missing or duplicated
    And the audit trail should be immutable
