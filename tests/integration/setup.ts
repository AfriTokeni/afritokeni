/**
 * Test setup - runs before all tests
 * Mocks Juno to prevent IndexedDB errors in Node.js
 */

// Set global test flag for mock detection
(global as any).__AFRITOKENI_TEST_MODE__ = true;

// Set test environment - will be overridden by npm scripts
// unit tests: NODE_ENV=unit-test
// integration tests: NODE_ENV=integration
if (!process.env.NODE_ENV) {
  process.env.NODE_ENV = 'unit-test';
}

console.log('🎭 Test setup: __AFRITOKENI_TEST_MODE__ = true');
console.log('🎭 Test setup: NODE_ENV =', process.env.NODE_ENV);

// Load canister IDs from dfx for local testing
try {
  const fs = require('fs');
  const path = require('path');
  const canisterIdsPath = path.join(process.cwd(), '.dfx/local/canister_ids.json');
  
  if (fs.existsSync(canisterIdsPath)) {
    const canisterIds = JSON.parse(fs.readFileSync(canisterIdsPath, 'utf8'));
    
    // Set environment variables for canister IDs
    if (canisterIds.ckbtc_ledger?.local) {
      process.env.CANISTER_ID_CKBTC_LEDGER = canisterIds.ckbtc_ledger.local;
      console.log(`✅ Set CANISTER_ID_CKBTC_LEDGER = ${canisterIds.ckbtc_ledger.local}`);
    }
    
    if (canisterIds.ckusdc_ledger?.local) {
      process.env.CANISTER_ID_CKUSDC_LEDGER = canisterIds.ckusdc_ledger.local;
      console.log(`✅ Set CANISTER_ID_CKUSDC_LEDGER = ${canisterIds.ckusdc_ledger.local}`);
    }
    
    if (canisterIds.deposit_canister?.local) {
      process.env.CANISTER_ID_DEPOSIT_CANISTER = canisterIds.deposit_canister.local;
      console.log(`✅ Set CANISTER_ID_DEPOSIT_CANISTER = ${canisterIds.deposit_canister.local}`);
    }
    
    if (canisterIds.exchange_canister?.local) {
      process.env.CANISTER_ID_EXCHANGE_CANISTER = canisterIds.exchange_canister.local;
      console.log(`✅ Set CANISTER_ID_EXCHANGE_CANISTER = ${canisterIds.exchange_canister.local}`);
    }
    
    if (canisterIds.withdrawal_canister?.local) {
      process.env.CANISTER_ID_WITHDRAWAL_CANISTER = canisterIds.withdrawal_canister.local;
      console.log(`✅ Set CANISTER_ID_WITHDRAWAL_CANISTER = ${canisterIds.withdrawal_canister.local}`);
    }
  }
} catch (error) {
  console.warn('⚠️ Could not load canister IDs from .dfx/local/canister_ids.json:', error);
}

// Mock Juno satellite ID globally
process.env.VITE_JUNO_SATELLITE_ID = 'uxrrr-q7777-77774-qaaaq-cai';

// Create a global satellite state that Juno will use
const mockSatelliteStateIntegration = {
  satelliteId: 'uxrrr-q7777-77774-qaaaq-cai',
  initialized: true
};

// Mock the @junobuild/core module before it's imported
const ModuleIntegration = require('module');
const originalRequireIntegration = ModuleIntegration.prototype.require;

ModuleIntegration.prototype.require = function(id: string) {
  if (id === '@junobuild/core') {
    const mockJuno = require('$tests/mocks/juno');
    return {
      setDoc: mockJuno.mockJuno.setDoc,
      getDoc: mockJuno.mockJuno.getDoc,
      listDocs: mockJuno.mockJuno.listDocs,
      deleteDoc: async () => {},
      initSatellite: async () => mockSatelliteStateIntegration,
      authSubscribe: () => () => {},
      listAssets: async () => ({ items: [], items_length: 0n, matches_length: 0n, items_page: 0n, matches_pages: 0n }),
      satelliteId: () => mockSatelliteStateIntegration.satelliteId
    };
  }
  return originalRequireIntegration.apply(this, arguments as any);
};

console.log('✅ Test setup complete - Juno mocked');
