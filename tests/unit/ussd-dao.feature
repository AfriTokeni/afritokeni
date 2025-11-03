Feature: USSD DAO Governance
  As a feature phone user
  I want to participate in DAO governance via USSD
  So that I can vote on proposals without internet access

  Background:
    Given I am a registered USSD user with 5000 AFRI tokens
    And there are active DAO proposals

  Scenario: Access DAO menu from main menu
    When I dial "*229#"
    And I select "4" for DAO Governance
    Then I should see "DAO Governance" in USSD response
    And I should see "1. View Proposals" in USSD response
    And I should see "2. My Voting Power" in USSD response
    And I should see "3. Active Votes" in USSD response

  Scenario: View my voting power
    When I dial "*229#"
    And I select "4" for DAO Governance
    And I select "2" for My Voting Power
    Then I should see "Your Voting Power" in USSD response
    And I should see "5000 AFRI" in USSD response

  Scenario: View active proposals
    When I dial "*229#"
    And I select "4" for DAO Governance
    And I select "1" for View Proposals
    Then I should see "Active Proposals" in USSD response
    And I should see proposal titles in USSD response
    And I should see "0. Back" in USSD response


  Scenario: Vote YES on a proposal
    When I dial "*229#"
    And I select "4" for DAO Governance
    And I select "1" for View Proposals
    And I select "1" for first proposal
    And I select "1" for Vote YES
    And I enter voting amount "1000"
    And I enter my PIN "1234"
    Then I should see "Vote Successful" in USSD response
    And I should see "Voted YES" in USSD response
    And I should see "1000 AFRI" in USSD response
    And my voting power should be reduced by 1000

  Scenario: Vote NO on a proposal
    When I dial "*229#"
    And I select "4" for DAO Governance
    And I select "1" for View Proposals
    And I select "1" for first proposal
    And I select "2" for Vote NO
    And I enter voting amount "500"
    And I enter my PIN "1234"
    Then I should see "Vote Successful" in USSD response
    And I should see "Voted NO" in USSD response
    And I should see "500 AFRI" in USSD response

  Scenario: Vote ABSTAIN on a proposal
    When I dial "*229#"
    And I select "4" for DAO Governance
    And I select "1" for View Proposals
    And I select "1" for first proposal
    And I select "3" for Vote ABSTAIN
    And I enter voting amount "250"
    And I enter my PIN "1234"
    Then I should see "Vote Successful" in USSD response
    And I should see "Voted ABSTAIN" in USSD response

  Scenario: Cannot vote with insufficient tokens
    When I dial "*229#"
    And I select "4" for DAO Governance
    And I select "1" for View Proposals
    And I select "1" for first proposal
    And I select "1" for Vote YES
    And I enter voting amount "10000"
    Then I should see "Insufficient" in USSD response

  Scenario: Cannot vote with invalid amount
    When I dial "*229#"
    And I select "4" for DAO Governance
    And I select "1" for View Proposals
    And I select "1" for first proposal
    And I select "1" for Vote YES
    And I enter voting amount "0"
    Then I should see "Invalid" in USSD response

  Scenario: View my active votes
    Given I have voted on 2 proposals
    When I dial "*229#"
    And I select "4" for DAO Governance
    And I select "3" for Active Votes
    Then I should see "Your Active Votes" in USSD response

  Scenario: Cannot vote twice on same proposal
    Given I have already voted on proposal 1
    When I dial "*229#"
    And I select "4" for DAO Governance
    And I select "1" for View Proposals
    And I select "1" for first proposal
    Then I should see "Already voted" in USSD response

  Scenario: Back to main menu from DAO
    When I dial "*229#"
    And I select "4" for DAO Governance
    And I select "0" for Back
    Then the session menu should be "main"
    And I should see "AfriTokeni" in USSD response
