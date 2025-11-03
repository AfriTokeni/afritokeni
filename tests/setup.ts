/**
 * Test setup - runs before all tests
 * Mocks Juno to prevent IndexedDB errors in Node.js
 */

// Set global test flag for// Global test flag
(global as Record<string, unknown>).__AFRITOKENI_TEST_MODE__ = true;

// Set test environment - will be overridden by npm scripts
// unit tests: NODE_ENV=unit-test
// integration tests: NODE_ENV=integration
if (!process.env.NODE_ENV) {
  process.env.NODE_ENV = 'unit-test';
}

console.log('🎭 Test setup: __AFRITOKENI_TEST_MODE__ = true');
console.log('🎭 Test setup: NODE_ENV =', process.env.NODE_ENV);

// Mock Juno satellite ID globally
process.env.VITE_JUNO_SATELLITE_ID = 'uxrrr-q7777-77774-qaaaq-cai';

// Create a global satellite state that Juno will use
const mockSatelliteState = {
  satelliteId: 'uxrrr-q7777-77774-qaaaq-cai',
  initialized: true
};

// Mock the @junobuild/core module before it's imported
import Module from 'module';
import { mockJuno } from './mocks/juno.js';

const originalRequire = Module.prototype.require;

Module.prototype.require = function(id: string) {
  if (id === '@junobuild/core') {
    return {
      setDoc: mockJuno.setDoc,
      getDoc: mockJuno.getDoc,
      listDocs: mockJuno.listDocs,
      deleteDoc: async () => {},
      initSatellite: async () => mockSatelliteState,
      authSubscribe: () => () => {},
      listAssets: async () => ({ items: [], items_length: 0n, matches_length: 0n, items_page: 0n, matches_pages: 0n }),
      satelliteId: () => mockSatelliteState.satelliteId
    };
  }
   
  return originalRequire.apply(this, arguments);
};

console.log('✅ Test setup complete - Juno mocked');
