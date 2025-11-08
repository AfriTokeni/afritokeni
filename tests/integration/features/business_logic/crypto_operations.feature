Feature: Cryptocurrency Operations via Business Logic Canister
  As a user of AfriTokeni
  I want to buy and send cryptocurrency
  So that I can access Bitcoin and USDC with my local currency

  Background:
    Given the Business Logic Canister is deployed
    And the Data Canister is deployed
    And the USSD Canister is authorized to call Business Logic
    And user "Alice" with phone "+256700000001" has 500000 UGX balance
    And Alice has set up PIN "1234"
    And the exchange rate is 1 BTC = 100000000 UGX
    And the exchange rate is 1 USDC = 3800 UGX

  Scenario: Successfully buy Bitcoin
    When Alice buys Bitcoin with 100000 UGX using PIN "1234"
    Then the purchase should succeed
    And Alice's UGX balance should be 400000 UGX
    And Alice should receive approximately 0.001 BTC
    And an audit entry should be created for "buy_crypto"
    And the audit entry should show success as true

  Scenario: Successfully buy USDC
    When Alice buys USDC with 38000 UGX using PIN "1234"
    Then the purchase should succeed
    And Alice's UGX balance should be 462000 UGX
    And Alice should receive approximately 10 USDC
    And an audit entry should be created for "buy_crypto"
    And the audit entry should show success as true

  Scenario: Buy crypto with insufficient fiat balance
    When Alice buys Bitcoin with 600000 UGX using PIN "1234"
    Then the purchase should fail with "Insufficient fiat balance"
    And Alice's UGX balance should remain 500000 UGX
    And Alice's BTC balance should remain 0
    And an audit entry should be created for "buy_crypto"
    And the audit entry should show success as false

  Scenario: Buy crypto with incorrect PIN
    When Alice buys Bitcoin with 100000 UGX using PIN "9999"
    Then the purchase should fail with "Invalid PIN"
    And Alice's UGX balance should remain 500000 UGX
    And an audit entry should be created for "buy_crypto"
    And the audit entry should show success as false

  Scenario: Send Bitcoin to address
    Given Alice has 0.01 BTC balance
    When Alice sends 0.005 BTC to address "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh" with PIN "1234"
    Then the send should succeed
    And Alice's BTC balance should be 0.005 BTC
    And an audit entry should be created for "send_crypto"
    And the audit entry should show success as true

  Scenario: Send crypto with insufficient balance
    Given Alice has 0.001 BTC balance
    When Alice sends 0.005 BTC to address "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh" with PIN "1234"
    Then the send should fail with "Insufficient crypto balance"
    And Alice's BTC balance should remain 0.001 BTC
    And an audit entry should be created for "send_crypto"
    And the audit entry should show success as false

  Scenario: Exchange rate unavailable
    Given the exchange rate service is down
    When Alice buys Bitcoin with 100000 UGX using PIN "1234"
    Then the purchase should fail with "Unable to get exchange rate"
    And Alice's UGX balance should remain 500000 UGX
