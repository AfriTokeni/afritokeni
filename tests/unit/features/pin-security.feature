Feature: PIN Security
  As a security-conscious system
  I want to ensure PIN validation and hashing is secure
  So that user accounts are protected

  Scenario: Valid 4-digit PIN
    Given a PIN "1234"
    When I validate the PIN
    Then the PIN should be valid

  Scenario: Valid 6-digit PIN
    Given a PIN "123456"
    When I validate the PIN
    Then the PIN should be valid

  Scenario: Invalid short PIN
    Given a PIN "123"
    When I validate the PIN
    Then the PIN should be invalid

  Scenario: Invalid long PIN
    Given a PIN "1234567"
    When I validate the PIN
    Then the PIN should be invalid

  Scenario: Invalid non-numeric PIN
    Given a PIN "12a4"
    When I validate the PIN
    Then the PIN should be invalid

  Scenario: PIN hashing produces Argon2 hash
    Given a PIN "1234"
    And a phone number "+256700123456"
    When I hash the PIN
    Then the hash should start with "$argon2"

  Scenario: Correct PIN verification
    Given a PIN "1234"
    And a phone number "+256700123456"
    When I hash the PIN
    And I verify the PIN with the hash
    Then the verification should succeed

  Scenario: Wrong PIN verification fails
    Given a PIN "1234"
    And a phone number "+256700123456"
    When I hash the PIN
    And I verify PIN "5678" with the hash
    Then the verification should fail

  Scenario: Different users produce different hashes
    Given a PIN "1234"
    When I hash the PIN for phone "+256700111111"
    And I hash the PIN for phone "+256700222222"
    Then the hashes should be different

  Scenario: Same user produces deterministic hash
    Given a PIN "1234"
    And a phone number "+256700123456"
    When I hash the PIN twice
    Then both hashes should be identical
