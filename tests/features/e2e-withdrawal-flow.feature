@e2e
Feature: End-to-End Withdrawal Flow
  As a user
  I want to withdraw my digital balance as cash
  So that I can use physical money

  Background:
    Given the withdrawal canister is deployed
    And I am a registered user with phone number "+256700123456"
    And I have 100000 UGX balance
    And agent "Agent-001" is available

  Scenario: Complete withdrawal flow via USSD
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "4" for Withdraw
    And I enter my PIN "1234"
    And I enter withdrawal amount "50000"
    And I select agent "Agent-001"
    Then I should see "Withdrawal request created"
    And I should see a 6-digit withdrawal code
    And the withdrawal request should be stored in the canister
    And the request status should be "pending"
    And my balance should be reserved

  Scenario: Agent completes withdrawal on canister
    Given I have a pending withdrawal request with code "654321"
    When the agent confirms the withdrawal with code "654321"
    Then the withdrawal status should be "completed"
    And my balance should decrease by 50000 UGX
    And the platform should earn 250 UGX fee
    And the agent should earn 1350 UGX commission
    And I should receive SMS confirmation

  Scenario: Withdrawal exceeds balance
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "4" for Withdraw
    And I enter my PIN "1234"
    And I enter withdrawal amount "200000"
    Then I should see "Insufficient balance"
    And no withdrawal request should be created

  Scenario: Wrong PIN for withdrawal
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "4" for Withdraw
    And I enter my PIN "9999"
    Then I should see "Invalid PIN"
    And no withdrawal request should be created

  Scenario: Agent commission calculation and revenue verification
    Given I request withdrawal of 100000 UGX in urban area
    When the withdrawal is confirmed by the agent
    Then the platform fee should be exactly 500 UGX
    And the agent fee should be exactly 3000 UGX
    And the platform should take exactly 300 UGX from agent fee
    And the agent should keep exactly 2700 UGX
    And the total platform revenue should be exactly 800 UGX
    And the withdrawal canister should record all fees correctly
    And querying agent earnings should show 2700 UGX
    And querying platform revenue should show 800 UGX
    And the on-chain record should be immutable
