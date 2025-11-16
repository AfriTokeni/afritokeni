import { defineConfig } from "@junobuild/config";

/**
 * Juno Configuration - File Storage Only
 *
 * NOTE: This configuration is for file storage (images, KYC documents) only.
 * All application data comes from ICP canisters:
 * - Agent data: agent_canister
 * - User data: user_canister
 * - Transactions: wallet_canister
 * - Crypto operations: crypto_canister
 *
 * API credentials are stored in environment variables, not Juno datastore.
 */

export default defineConfig({
  satellite: {
    ids: {
      development: "atbka-rp777-77775-aaaaq-cai",
      preview: "64njw-oiaaa-aaaal-asppa-cai",
      production: "dkk74-oyaaa-aaaal-askxq-cai"
    },
    source: "build",
    predeploy: ["pnpm run build"],
    collections: {
      storage: [
        {
          collection: "profile-images",
          read: "public",
          write: "managed",
          memory: "stable",
          maxSize: 5242880n, // 5MB
        },
        {
          collection: "agent-profile-images",
          read: "public",
          write: "managed",
          memory: "stable",
          maxSize: 5242880n, // 5MB
        },
        {
          collection: "kyc_documents",
          read: "managed",  // Only owner can read their own KYC docs
          write: "managed",
          memory: "stable",
          maxSize: 10485760n, // 10MB
        },
      ],
    },
  },
  emulator: {
    runner: {
      type: "docker"
    },
    skylab: {}
  }
});
