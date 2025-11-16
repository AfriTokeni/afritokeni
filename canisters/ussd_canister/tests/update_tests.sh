#!/bin/bash
# Script to update all test files to use setup_test_user_with_balances

# Find all test files
TEST_FILES=$(find canisters/ussd_canister/tests/integration -name "*_tests.rs" -type f)

for file in $TEST_FILES; do
    echo "Processing $file..."
    
    # Replace pattern: register_user_direct + set_fiat_balance
    # with: setup_test_user_with_balances
    
    # This is a complex replacement that would need careful regex
    # For now, let's just report which files need updating
    
    if grep -q "register_user_direct" "$file"; then
        echo "  - Found register_user_direct calls"
    fi
    
    if grep -q "set_fiat_balance" "$file"; then
        echo "  - Found set_fiat_balance calls"
    fi
    
    if grep -q "set_crypto_balance" "$file"; then
        echo "  - Found set_crypto_balance calls"
    fi
done
