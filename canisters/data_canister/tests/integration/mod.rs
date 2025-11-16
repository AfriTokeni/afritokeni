// Integration tests for data_canister
//
// These tests verify:
// - Stable storage persistence across upgrades (CRITICAL)
// - Agent activity fraud detection storage
// - Access control enforcement (security)
// - KYC workflow compliance

mod stable_storage_tests;
mod agent_activity_tests;
mod access_control_tests;
mod kyc_workflow_tests;
