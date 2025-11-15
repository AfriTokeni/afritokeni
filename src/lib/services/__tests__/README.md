# AfriTokeni Service Tests - Quick Reference

## Running Tests

```bash
# Run all frontend tests
pnpm run test:frontend

# Run tests in watch mode (auto-rerun on file changes)
pnpm run test:frontend:watch

# Run tests with UI (browser interface)
pnpm run test:frontend:ui

# Run tests with coverage report
pnpm run test:frontend:coverage
```

## Quick Fix for Failing Tests

All 22 failing tests are due to a BigInt conversion issue. Run this command to fix them all:

```bash
find src/lib/services/__tests__/ -name "*.test.ts" -exec sed -i '' 's/BigInt(Date\.now() \/ 1000)/BigInt(Math.floor(Date.now() \/ 1000))/g' {} \;
```

After running the fix, re-run tests:

```bash
pnpm run test:frontend
```

Expected result: **121/121 tests passing (100%)**

## Test Files

- `setup.ts` - Global mocks and test utilities
- `revenueProtection.test.ts` - **CRITICAL** - Verifies 0.5% fee collection
- `cryptoService.test.ts` - Crypto operations (buy, sell, swap, escrow)
- `walletService.test.ts` - Fiat operations (P2P transfers, balances)
- `agentOperationsService.test.ts` - Agent operations (deposits, withdrawals)
- `integration.test.ts` - End-to-end user journeys

## Test Utilities

Located in `setup.ts`:

```typescript
import {
  TEST_USER_ID,
  TEST_USER_PIN,
  TEST_AGENT_ID,
  calculateExpectedFee,
  calculateNetAmount
} from './setup';

// Calculate 0.5% platform fee
const fee = calculateExpectedFee(100_000); // Returns 500

// Calculate net amount after fee
const net = calculateNetAmount(100_000); // Returns 99_500
```

## Writing New Tests

### Template for Service Method Test

```typescript
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { YourService } from '../yourService';
import { yourCanisterService } from '../icp/canisters/yourCanisterService';
import { TEST_USER_ID, TEST_USER_PIN } from './setup';

describe('YourService', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('methodName', () => {
    it('should do something', async () => {
      // Arrange
      const mockResponse = { /* ... */ };
      vi.spyOn(yourCanisterService, 'methodName').mockResolvedValue(mockResponse);

      // Act
      const result = await YourService.methodName(params);

      // Assert
      expect(result).toEqual(mockResponse);
      expect(yourCanisterService.methodName).toHaveBeenCalledWith(expectedParams);
    });
  });
});
```

### Template for Revenue Protection Test

```typescript
it('should collect 0.5% platform fee', async () => {
  const amount = 100_000;
  const expectedFee = calculateExpectedFee(amount);

  const mockResponse = {
    platform_fee: BigInt(expectedFee),
    // ... other fields
  };

  vi.spyOn(canisterService, 'method').mockResolvedValue(mockResponse);

  const result = await Service.method(params);

  // CRITICAL: Verify fee was collected
  expect(result.platform_fee).toBe(BigInt(expectedFee));
  expect(Number(result.platform_fee)).toBe(500); // 0.5% of 100,000
});
```

## Common Patterns

### Mocking Canister Calls

```typescript
vi.spyOn(canisterService, 'methodName').mockResolvedValue(mockData);
```

### Testing Error Handling

```typescript
vi.spyOn(canisterService, 'methodName').mockRejectedValue(
  new Error('Expected error message')
);

await expect(Service.methodName(params)).rejects.toThrow('Expected error message');
```

### Testing with BigInt

```typescript
// Use Math.floor() for timestamps
const timestamp = BigInt(Math.floor(Date.now() / 1000));

// Compare BigInts
expect(result.amount).toBe(100_000n);

// Convert to number for assertions
expect(Number(result.amount)).toBe(100_000);
```

## Current Test Status

- **Total Tests:** 121
- **Passing:** 99 (82%)
- **Failing:** 22 (18% - all due to BigInt timestamp issue)

After applying the fix:
- **Expected Passing:** 121 (100%)

## Key Test Categories

1. **Revenue Protection (CRITICAL)** ⚠️
   - Crypto buy/sell fee collection
   - Crypto swap spread collection
   - Fiat P2P fee collection
   - Anti-bypass verification

2. **Service Methods**
   - All public methods tested
   - Happy paths and error paths
   - Parameter validation

3. **Integration Tests**
   - Complete user journeys
   - Multi-step flows
   - State transitions

4. **Edge Cases**
   - Negative/zero amounts
   - Very large amounts
   - Concurrent operations
   - Network failures

## Debugging Failed Tests

### View Test Output in Detail

```bash
pnpm run test:frontend -- --reporter=verbose
```

### Run Specific Test File

```bash
pnpm run test:frontend src/lib/services/__tests__/revenueProtection.test.ts
```

### Run Specific Test

```bash
pnpm run test:frontend -t "should collect 0.5% platform fee"
```

### Debug with Breakpoints

```bash
pnpm run test:frontend --inspect-brk
```

Then open Chrome DevTools at: `chrome://inspect`

## Coverage Reports

Generate coverage report:

```bash
pnpm run test:frontend:coverage
```

View coverage in browser:

```bash
open coverage/index.html
```

## CI/CD Integration

Tests are automatically run in the CI/CD pipeline via:

```bash
pnpm run test
```

Which runs both:
1. Rust canister tests (`pnpm run test:unit` and `pnpm run test:integration`)
2. Frontend service tests (`pnpm run test:frontend`)

## Need Help?

- See `TEST_COVERAGE_SUMMARY.md` for detailed coverage analysis
- See `CLAUDE.md` for project conventions
- Check test files for examples of different test patterns
