Feature: USSD Session Reset
  As a user
  I want to be able to dial the USSD code at any time
  So that I can start fresh from the main menu regardless of my current state

  Background:
    Given I am a registered user with phone number "256788123456"
    And my PIN is "1234"

  Scenario: Reset session from main menu
    When I dial "*384*22948#"
    Then I should see "Welcome to AfriTokeni" in USSD response
    And I should see "1. Local Currency" in USSD response
    And I should see "2. Bitcoin (ckBTC)" in USSD response
    And the session should continue

  Scenario: Reset session from local currency menu
    When I dial "*384*22948#"
    And I enter "1"
    Then I should see "Local Currency" in USSD response
    When I dial "*384*22948#"
    Then I should see "Welcome to AfriTokeni" in USSD response
    And I should see "1. Local Currency" in USSD response
    And the session should continue

  Scenario: Reset session from Bitcoin menu
    When I dial "*384*22948#"
    And I enter "2"
    Then I should see "Bitcoin" in USSD response
    When I dial "*384*22948#"
    Then I should see "Welcome to AfriTokeni" in USSD response
    And I should see "1. Local Currency" in USSD response
    And the session should continue

  Scenario: Reset session from USDC menu
    When I dial "*384*22948#"
    And I enter "3"
    Then I should see "USDC" in USSD response
    When I dial "*384*22948#"
    Then I should see "Welcome to AfriTokeni" in USSD response
    And I should see "1. Local Currency" in USSD response
    And the session should continue

  Scenario: Reset session from DAO menu
    When I dial "*384*22948#"
    And I enter "4"
    Then I should see "DAO" in USSD response
    When I dial "*384*22948#"
    Then I should see "Welcome to AfriTokeni" in USSD response
    And I should see "1. Local Currency" in USSD response
    And the session should continue

  Scenario: Multiple resets in succession
    When I dial "*384*22948#"
    Then I should see "Welcome to AfriTokeni" in USSD response
    When I dial "*384*22948#"
    Then I should see "Welcome to AfriTokeni" in USSD response
    When I dial "*384*22948#"
    Then I should see "Welcome to AfriTokeni" in USSD response
    And the session should continue

  Scenario: Reset session does not preserve language (creates fresh session)
    When I dial "*384*22948#"
    And I enter "6"
    And I enter "2"
    Then I should see "Luganda" in USSD response
    When I dial "*384*22948#"
    Then I should see "Welcome to AfriTokeni" in USSD response
    And I should see "1. Local Currency" in USSD response
