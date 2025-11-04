@integration
Feature: ckBTC Integration
  As a user of AfriTokeni
  I want to interact with real ckBTC ledger on ICP
  So that I can verify the blockchain integration works correctly

  Background:
    Given the local ICP replica is running
    And ckBTC ledger is deployed

  Scenario: Query ckBTC ledger metadata
    When I query the ckBTC ledger for token metadata
    Then the token symbol should be "ckBTC"
    And the token name should be "ckBTC"
    And the decimals should be 8

  Scenario: Query ckBTC balance
    Given I have a test principal
    When I query my ckBTC balance on the ledger
    Then I should receive a valid balance response
    And the balance should be a non-negative number

  Scenario: Complete user deposit flow
    Given I have a test principal
    And I have 1000000 satoshis of ckBTC
    When I transfer 50000 satoshis to another user
    Then the transfer should succeed
    And my balance should decrease by 50000 satoshis
    And the recipient balance should increase by 50000 satoshis

  Scenario: Exchange ckBTC for local currency via escrow
    Given I have a test principal
    And I have 100000 satoshis of ckBTC
    When I create an escrow to exchange 50000 satoshis for UGX
    Then an escrow transaction should be created
    And I should receive a 6-digit exchange code
    When the agent verifies the exchange code
    Then the ckBTC should be released to the agent
    And the escrow status should be "completed"

  Scenario: Check ckBTC balance
    Given I have 0.005 ckBTC
    When I check my ckBTC balance
    Then I see 0.005 ckBTC

  Scenario: Send ckBTC to another user
    Given I have 0.005 ckBTC
    When I send 0.003 ckBTC to another user
    Then my balance is 0.002 ckBTC

  Scenario: Sell ckBTC for cash via agent
    Given I have 0.005 ckBTC
    When I sell 0.005 ckBTC for UGX via agent
    Then I receive an escrow code

  Scenario: Complete agent exchange
    Given I have an active escrow with code "BTC-123456"
    When the agent confirms the exchange
    Then the ckBTC is released to the agent
