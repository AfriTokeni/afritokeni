#!/bin/bash
# Comprehensive accessibility fix using sed

echo "🔧 Fixing all accessibility warnings..."

# Fix 1: Add aria-label to toggle buttons in AgentSettingsComponent
sed -i '' '/<button$/,/onclick={() =>/ {
  /<button$/ {
    N
    N
    s/<button\n              type="button"\n              onclick={() =>/<button\n              type="button"\n              aria-label="Toggle setting"\n              onclick={() =>/
  }
}' src/lib/components/shared/AgentSettingsComponent.svelte

# Fix 2: Add for/id to remaining labels and inputs in AgentSettingsComponent
# Auto-Accept Limit
sed -i '' 's/<label class="mb-2 block text-sm font-medium text-gray-700">Auto-Accept Limit (UGX)<\/label>/<label for="autoAcceptLimit" class="mb-2 block text-sm font-medium text-gray-700">Auto-Accept Limit (UGX)<\/label>/' src/lib/components/shared/AgentSettingsComponent.svelte
sed -i '' '/Auto-Accept Limit/,/<input/ {
  /<input$/ {
    N
    s/<input\n              type="number"/<input\n              id="autoAcceptLimit"\n              type="number"/
  }
}' src/lib/components/shared/AgentSettingsComponent.svelte

# Fix 3: Password fields
sed -i '' 's/<label class="mb-2 block text-sm font-medium text-gray-700">Current Password<\/label>/<label for="currentPassword" class="mb-2 block text-sm font-medium text-gray-700">Current Password<\/label>/' src/lib/components/shared/AgentSettingsComponent.svelte
sed -i '' 's/<label class="mb-2 block text-sm font-medium text-gray-700">New Password<\/label>/<label for="newPassword" class="mb-2 block text-sm font-medium text-gray-700">New Password<\/label>/' src/lib/components/shared/AgentSettingsComponent.svelte
sed -i '' 's/<label class="mb-2 block text-sm font-medium text-gray-700">Confirm New Password<\/label>/<label for="confirmPassword" class="mb-2 block text-sm font-medium text-gray-700">Confirm New Password<\/label>/' src/lib/components/shared/AgentSettingsComponent.svelte

# Add IDs to password inputs
sed -i '' '/Current Password/,/<input/ {
  /<input$/ {
    N
    s/<input\n                type="password"/<input\n                id="currentPassword"\n                type="password"/
  }
}' src/lib/components/shared/AgentSettingsComponent.svelte

sed -i '' '/New Password<\/label>/,/<input/ {
  /<input$/ {
    N
    s/<input\n                type="password"/<input\n                id="newPassword"\n                type="password"/
  }
}' src/lib/components/shared/AgentSettingsComponent.svelte

sed -i '' '/Confirm New Password/,/<input/ {
  /<input$/ {
    N
    s/<input\n                type="password"/<input\n                id="confirmPassword"\n                type="password"/
  }
}' src/lib/components/shared/AgentSettingsComponent.svelte

# Fix 4: Other files - add for/id pairs
for file in src/lib/components/shared/CustomerManagementComponent.svelte \
            src/lib/components/dashboard/OnboardingModal.svelte \
            src/lib/components/shared/KYCModal.svelte \
            src/lib/components/landing/SavingsComparisonTable.svelte \
            src/routes/users/profile/AccountSettings.svelte; do
  if [ -f "$file" ]; then
    echo "Processing $file..."
    # Generic fix: add IDs to inputs that follow labels
    sed -i '' 's/<input type="text" bind:value=/<input id="input_'$RANDOM'" type="text" bind:value=/g' "$file"
    sed -i '' 's/<input type="email" bind:value=/<input id="email_'$RANDOM'" type="email" bind:value=/g' "$file"
    sed -i '' 's/<input type="tel" bind:value=/<input id="tel_'$RANDOM'" type="tel" bind:value=/g' "$file"
  fi
done

echo "✅ Done! Run: npm run check"
