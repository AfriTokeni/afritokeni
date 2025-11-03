Feature: USSD Language Selection
  As a feature phone user
  I want to select my preferred language
  So that I can use AfriTokeni in my native language

  Background:
    Given I am a registered USSD user

  Scenario: Access language selection from main menu
    When I dial "*229#"
    Then I should see "Welcome to AfriTokeni" in USSD response
    And I should see "6. Language Selection" in USSD response

  Scenario: View language options
    When I dial "*229#"
    And I select "6" for Language Selection
    Then I should see "Select language:" in USSD response
    And I should see "1. English" in USSD response
    And I should see "2. Luganda" in USSD response
    And I should see "3. Swahili" in USSD response

  Scenario: Select English language
    When I dial "*229#"
    And I select "6" for Language Selection
    And I select "1" for English
    Then I should see "Language set to English" in USSD response
    And I should see "0. Back" in USSD response
    And the session language should be "en"

  Scenario: Select Luganda language
    When I dial "*229#"
    And I select "6" for Language Selection
    And I select "2" for Luganda
    Then I should see "Olulimi lutegekeddwa ku Luganda" in USSD response
    And I should see "0. Ddayo" in USSD response
    And the session language should be "lg"

  Scenario: Select Swahili language
    When I dial "*229#"
    And I select "6" for Language Selection
    And I select "3" for Swahili
    Then I should see "Lugha imewekwa kwa Kiswahili" in USSD response
    And I should see "0. Rudi" in USSD response
    And the session language should be "sw"

  Scenario: Return to main menu after language selection shows translated menu
    When I dial "*229#"
    And I select "6" for Language Selection
    And I select "2" for Luganda
    And I select "0" to go back
    Then I should see "Tukusanyukidde ku AfriTokeni" in USSD response
    And I should see "Obuyambi" in USSD response
    And the session language should be "lg"

  Scenario: Main menu shows translated text in Luganda
    Given I am a registered USSD user
    And I select "6" for Language Selection
    And I select "2" for Luganda
    And I select "0" to go back
    Then I should see "Tukusanyukidde ku AfriTokeni" in USSD response
    And I should see "Obuyambi" in USSD response

  Scenario: Main menu shows translated text in Swahili
    Given I am a registered USSD user
    And I select "6" for Language Selection
    And I select "3" for Swahili
    And I select "0" to go back
    Then I should see "Karibu AfriTokeni" in USSD response
    And I should see "Msaada" in USSD response

  Scenario: Help menu shows translated text in Luganda
    Given I am a registered USSD user
    And I select "6" for Language Selection
    And I select "2" for Luganda
    And I select "0" to go back
    And I select "5" for Help
    Then I should see "Obuyambi" in USSD response
    And I should see "Wereza Ssente" in USSD response
    And I should see "Ggya Ssente" in USSD response

  Scenario: Help menu shows translated text in Swahili
    Given I am a registered USSD user
    And I select "6" for Language Selection
    And I select "3" for Swahili
    And I select "0" to go back
    And I select "5" for Help
    Then I should see "Msaada" in USSD response
    And I should see "Tuma Pesa" in USSD response
    And I should see "Ondoa Pesa" in USSD response

  Scenario: Go back from language selection menu
    When I dial "*229#"
    And I select "6" for Language Selection
    And I select "0" to go back
    Then I should see "Welcome to AfriTokeni" in USSD response
    And I should see "1. Local Currency" in USSD response

  Scenario: Invalid language selection
    When I dial "*229#"
    And I select "6" for Language Selection
    And I select "9" for invalid option
    Then I should see "Select language:" in USSD response
    And I should see "1. English" in USSD response

  Scenario: Language persists across session
    When I dial "*229#"
    And I select "6" for Language Selection
    And I select "2" for Luganda
    And I select "0" to go back
    Then the session language should be "lg"
    And I should see "Tukusanyukidde ku AfriTokeni" in USSD response

  Scenario: Chained input for language selection
    When I dial "*229#"
    And I enter chained input "6*2"
    Then I should see "Olulimi lutegekeddwa ku Luganda" in USSD response
    And the session language should be "lg"

  Scenario: Error messages show in selected language
    Given my language preference is "lg"
    When I dial "*229#"
    And I select "99" for invalid option
    Then I should see "Ekiragiro si kituufu" in USSD response
