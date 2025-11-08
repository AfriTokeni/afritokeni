@integration @ussd
Feature: USSD Language Selection
  As a user
  I want to change my language preference via USSD
  So that I can use the service in my preferred language

  Background:
    Given the local ICP replica is running
    And the USSD canister is deployed
    And I have a registered account with phone "+254700123456"

  Scenario: Change language to English
    When I dial USSD code "*384*22948#"
    And I select option "5" for Language
    And I select option "1" for English
    Then I should see "Language set" in English
    And my language preference should be saved as "en"

  Scenario: Change language to Luganda
    When I dial USSD code "*384*22948#"
    And I select option "5" for Language
    And I select option "2" for Luganda
    Then I should see "Language set" in Luganda
    And my language preference should be saved as "lg"

  Scenario: Change language to Swahili
    When I dial USSD code "*384*22948#"
    And I select option "5" for Language
    And I select option "3" for Swahili
    Then I should see "Language set" in Swahili
    And my language preference should be saved as "sw"

  Scenario: Language persists across sessions
    Given I have set my language to Luganda
    When I dial USSD code "*384*22948#" in a new session
    Then I should see the main menu in Luganda

  Scenario: All menus respect language preference
    Given I have set my language to Swahili
    When I dial USSD code "*384*22948#"
    And I select option "1" for Local Currency
    Then I should see the local currency menu in Swahili
    When I select option "0" to go back
    And I select option "2" for Bitcoin
    Then I should see the Bitcoin menu in Swahili
