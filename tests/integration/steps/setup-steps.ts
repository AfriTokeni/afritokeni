/**
 * Integration test setup step definitions
 * Verifies ICP replica and canisters are ready
 */

import { Given } from '@cucumber/cucumber';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

Given('the local ICP replica is running', async function () {
  try {
    const { stdout } = await execAsync('dfx ping');
    const response = JSON.parse(stdout);
    
    if (response.replica_health_status !== 'healthy') {
      throw new Error(`ICP replica is not healthy: ${response.replica_health_status}`);
    }
    
    console.log('✅ Local ICP replica is running and healthy');
  } catch (error: any) {
    throw new Error(`Failed to ping ICP replica: ${error.message}`);
  }
});

Given('ckBTC ledger is deployed', async function () {
  try {
    const { stdout } = await execAsync('dfx canister id ckbtc_ledger --network local');
    const canisterId = stdout.trim();
    
    if (!canisterId) {
      throw new Error('ckBTC ledger canister ID not found');
    }
    
    console.log(`✅ ckBTC ledger deployed at: ${canisterId}`);
  } catch (error: any) {
    throw new Error(`ckBTC ledger not deployed: ${error.message}`);
  }
});

Given('ckUSDC ledger is deployed', async function () {
  try {
    const { stdout } = await execAsync('dfx canister id ckusdc_ledger --network local');
    const canisterId = stdout.trim();
    
    if (!canisterId) {
      throw new Error('ckUSDC ledger canister ID not found');
    }
    
    console.log(`✅ ckUSDC ledger deployed at: ${canisterId}`);
  } catch (error: any) {
    throw new Error(`ckUSDC ledger not deployed: ${error.message}`);
  }
});
