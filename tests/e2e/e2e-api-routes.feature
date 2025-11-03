@e2e
Feature: End-to-End API Routes
  As a system
  I want to handle SMS/USSD webhooks via SvelteKit API routes
  So that users can interact without a separate backend

  Background:
    Given the SvelteKit server is running
    And Africa's Talking is configured

  Scenario: USSD webhook receives request
    When Africa's Talking sends POST to "/api/ussd" with:
      | sessionId   | phoneNumber    | text |
      | ATUid_12345 | +256700123456  | 1    |
    Then the response should be "CON"
    And the response should contain "Local Currency"
    And the USSD session should be created
    And the session should be stored in memory

  Scenario: SMS webhook receives message
    When Africa's Talking sends POST to "/api/sms" with:
      | from          | text        |
      | +256700123456 | BAL         |
    Then the response status should be 200
    And the SMS should be logged
    And the response should acknowledge receipt

  Scenario: Send SMS verification code
    When the system sends POST to "/api/send-sms" with:
      | phoneNumber    | verificationCode |
      | +256700123456  | 123456          |
    Then the SMS should be sent via Africa's Talking
    And the verification code should be stored
    And the response should include messageId

  Scenario: Verify SMS code
    Given a verification code "123456" was sent to "+256700123456"
    When the system sends POST to "/api/verify-code" with:
      | phoneNumber    | code   |
      | +256700123456  | 123456 |
    Then the verification should succeed
    And the code should be deleted
    And the response should include userId

  Scenario: Health check endpoint
    When the system sends GET to "/api/health"
    Then the response status should be 200
    And the response should include service name
    And the response should list all endpoints

  Scenario: USSD session persistence across requests
    Given a USSD session "ATUid_12345" for "+256700123456"
    When Africa's Talking sends multiple requests:
      | sessionId   | text |
      | ATUid_12345 | 1    |
      | ATUid_12345 | 1    |
      | ATUid_12345 | 1234 |
    Then the session state should be maintained
    And each response should reflect the current menu
    And the final response should show balance

  Scenario: Demo mode SMS sending
    Given AT_USERNAME is "sandbox"
    When the system sends POST to "/api/send-sms"
    Then the SMS should NOT be sent to Africa's Talking
    And the response should indicate demo mode
    And the message should be logged only
