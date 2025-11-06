Feature: USSD Find Agent
  As a USSD user
  I want to find nearby agents
  So that I can perform cash transactions

  Background:
    Given I have a phone number "+256700123456"
    And I have set my PIN to "1234"

  Scenario: Find nearby agents
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "6" for Find Agent
    Then I should see "Available agents" in USSD response
    And I should see agent names
    And I should see agent locations

  Scenario: View agent details
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "6" for Find Agent
    Then I should see "Available agents" in USSD response
    When I select "1" for first agent
    Then I should see agent details
    And I should see "Location:" in USSD response
    And I should see "Services:" in USSD response

  # Scenario: No agents available
  # This scenario requires dynamic mock control which is not yet implemented
  # Given there are no agents in my area
  # When I dial "*229#"
  # And I select "1" for Local Currency
  # And I select "6" for Find Agent
  # Then I should see "No agents available" in USSD response
  # And I should see "Please try again later" in USSD response

  Scenario: Navigate back from agent list
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "6" for Find Agent
    Then I should see "Available agents" in USSD response
    When I select "0" to go back
    Then I should see "Local Currency" in USSD response
    And I should see "Send Money" in USSD response

  Scenario: Search agents by location
    When I dial "*229#"
    And I select "1" for Local Currency
    And I select "6" for Find Agent
    Then I should see "Available agents" in USSD response
    And I should see agents sorted by distance
    And the nearest agent should be listed first
