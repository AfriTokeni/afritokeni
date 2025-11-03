@e2e
Feature: End-to-End Deposit Flow
  As a user
  I want to deposit cash with an agent
  So that I can have digital balance in my account

  Background:
    Given the deposit canister is deployed
    And I am a registered user with phone number "+256700123456"
    And agent "Agent-001" is available with 500000 UGX cash

  Scenario: Complete deposit flow via USSD
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "3" for Deposit
    And I enter my PIN "1234"
    And I enter deposit amount "50000"
    And I select agent "Agent-001"
    Then I should see "Deposit request created"
    And I should see a 6-digit deposit code
    And the deposit request should be stored in the canister
    And the request status should be "pending"

  Scenario: Agent confirms deposit with revenue verification
    Given I have a pending deposit request with code "123456" for 50000 UGX
    When the agent confirms the deposit with code "123456"
    Then the deposit status should be "completed"
    And my balance should increase by exactly 49750 UGX
    And the platform should earn exactly 250 UGX commission
    And the deposit canister should record 250 UGX platform revenue
    And querying platform revenue should show 250 UGX
    And the canister revenue total should match the fee
    And I should receive SMS confirmation
    And the on-chain record should be immutable

  Scenario: Deposit with insufficient agent cash
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "3" for Deposit
    And I enter my PIN "1234"
    And I enter deposit amount "1000000"
    Then I should see "No agents available with sufficient cash"
    And no deposit request should be created

  Scenario: Deposit below minimum amount
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "3" for Deposit
    And I enter my PIN "1234"
    And I enter deposit amount "500"
    Then I should see "Minimum deposit is 1000 UGX"
    And no deposit request should be created

  Scenario: Expired deposit code
    Given I have a pending deposit request with code "123456" created 25 hours ago
    When the agent tries to confirm with code "123456"
    Then the confirmation should fail
    And I should see "Deposit code expired"
    And the deposit should be automatically cancelled
