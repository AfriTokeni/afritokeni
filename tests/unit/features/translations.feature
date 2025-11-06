Feature: Translations
  As a multilingual system
  I want to support English, Luganda, and Swahili
  So that users can interact in their preferred language

  Scenario Outline: Language code mapping
    Given a language code "<code>"
    When I get the language name
    Then the language should be "<language>"

    Examples:
      | code | language |
      | en   | English  |
      | lg   | Luganda  |
      | sw   | Swahili  |
      | xx   | English  |

  Scenario Outline: Translation keys exist in all languages
    Given a translation key "<key>"
    When I get translations for all languages
    Then English translation should exist
    And Luganda translation should exist
    And Swahili translation should exist

    Examples:
      | key              |
      | welcome          |
      | enter_pin        |
      | invalid_pin      |
      | send_money       |
      | check_balance    |
      | main_menu        |
      | local_currency   |
      | bitcoin          |
      | usdc             |
      | dao_governance   |

  Scenario: Main menu structure in English
    Given language "English"
    When I get the main menu
    Then the menu should contain "1."
    And the menu should contain "2."
    And the menu should contain "3."
    And the menu should contain "4."
    And the menu should contain "5."
    And the menu should contain "6."
    And the menu should contain "0."

  Scenario: Main menu structure in Luganda
    Given language "Luganda"
    When I get the main menu
    Then the menu should contain "1."
    And the menu should contain "2."
    And the menu should contain "3."
    And the menu should contain "4."
    And the menu should contain "5."
    And the menu should contain "6."
    And the menu should contain "0."

  Scenario: Main menu structure in Swahili
    Given language "Swahili"
    When I get the main menu
    Then the menu should contain "1."
    And the menu should contain "2."
    And the menu should contain "3."
    And the menu should contain "4."
    And the menu should contain "5."
    And the menu should contain "6."
    And the menu should contain "0."
