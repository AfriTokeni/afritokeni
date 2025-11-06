Feature: USSD Menu Navigation
  As a user
  I want to navigate through all USSD menus
  So that I can access all features

  Background:
    Given I have a valid phone number "+256700123456"
    And I have set my PIN to "1234"

  Scenario: Navigate to Local Currency menu and back
    When I dial "*229#"
    Then I should see "AfriTokeni" in USSD response
    When I select "1" for Local Currency
    Then I should see "Local Currency" in USSD response
    And I should see "1. Send Money" in USSD response
    And I should see "0. Back to Main Menu" in USSD response
    When I select "0" to go back
    Then I should see "AfriTokeni" in USSD response

  Scenario: Navigate to Bitcoin menu and back
    When I dial "*229#"
    Then I should see "AfriTokeni" in USSD response
    When I select "2" for Bitcoin
    Then I should see "Bitcoin" in USSD response
    And I should see "1. Check Balance" in USSD response
    And I should see "0. Back to Main Menu" in USSD response
    When I select "0" to go back
    Then I should see "AfriTokeni" in USSD response

  Scenario: Navigate to USDC menu and back
    When I dial "*229#"
    Then I should see "AfriTokeni" in USSD response
    When I select "3" for USDC
    Then I should see "USDC" in USSD response
    And I should see "1. Check Balance" in USSD response
    And I should see "2. USDC Rate" in USSD response
    And I should see "3. Buy USDC" in USSD response
    And I should see "4. Sell USDC" in USSD response
    And I should see "5. Send USDC" in USSD response
    And I should see "0. Back to Main Menu" in USSD response
    When I select "0" to go back
    Then I should see "AfriTokeni" in USSD response
    And I should see "3. USDC" in USSD response

  Scenario: Navigate to DAO menu and back
    When I dial "*229#"
    Then I should see "AfriTokeni" in USSD response
    When I select "4" for DAO Governance
    Then I should see "DAO Governance" in USSD response
    And I should see "1. View Proposals" in USSD response
    And I should see "0. Back to Main Menu" in USSD response
    When I select "0" to go back
    Then I should see "AfriTokeni" in USSD response
    And I should see "4. DAO Governance" in USSD response

  Scenario: Navigate to Help menu and back
    When I dial "*229#"
    Then I should see "AfriTokeni" in USSD response
    When I select "5" for Help
    Then I should see "Help" in USSD response
    When I select "0" to go back
    Then I should see "AfriTokeni" in USSD response
    And I should see "5. Help" in USSD response

  Scenario: Navigate to Language menu and back
    When I dial "*229#"
    Then I should see "AfriTokeni" in USSD response
    When I select "6" for Language Selection
    Then I should see "Select language" in USSD response
    And I should see "1. English" in USSD response
    And I should see "2. Luganda" in USSD response
    And I should see "3. Swahili" in USSD response
    When I select "0" to go back
    Then I should see "AfriTokeni" in USSD response
    And I should see "6. Language Selection" in USSD response

  Scenario: Test invalid menu option
    When I dial "*229#"
    Then I should see "AfriTokeni" in USSD response
    When I select "99" for invalid option
    Then I should see "Invalid option" in USSD response

  Scenario: Navigate through multiple menus in sequence
    When I dial "*229#"
    And I select "2" for Bitcoin
    Then I should see "Bitcoin" in USSD response
    When I select "0" to go back
    And I select "3" for USDC
    Then I should see "USDC" in USSD response
    When I select "0" to go back
    And I select "4" for DAO Governance
    Then I should see "DAO Governance" in USSD response
    When I select "0" to go back
    Then I should see "AfriTokeni" in USSD response

  Scenario: Press 9 to show current menu in Bitcoin submenu
    When I dial "*229#"
    And I select "2" for Bitcoin
    Then I should see "Bitcoin" in USSD response
    When I select "9" to show current menu
    Then I should see "Bitcoin" in USSD response
    And I should see "1. Check Balance" in USSD response

  Scenario: Press 9 to show current menu in USDC submenu
    When I dial "*229#"
    And I select "3" for USDC
    Then I should see "USDC" in USSD response
    When I select "9" to show current menu
    Then I should see "USDC" in USSD response
    And I should see "1. Check Balance" in USSD response
