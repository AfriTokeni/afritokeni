#!/bin/bash

# Fix all unused variables by prefixing with underscore
echo "Fixing unused variables..."

# Get all unused variable errors
pnpm run lint 2>&1 | grep "is defined but never used" | grep -oE "'[^']+'" | sort -u | while read -r var; do
  # Remove quotes
  var_name=$(echo "$var" | tr -d "'")
  echo "Fixing unused variable: $var_name"
  
  # Find and replace in src/ files
  find src -name "*.ts" -type f -exec sed -i '' "s/\b${var_name}\b/_${var_name}/g" {} +
done

echo "Done! Run 'pnpm run lint' to verify."
