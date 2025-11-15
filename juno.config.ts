import { defineConfig } from "@junobuild/config";

/**
 * REQUIRED CONFIGURATION FOR PRODUCTION:
 * 
 * Store Africa's Talking credentials in the "config" collection:
 * 
 * 1. Create a document in the "config" collection with key "afritalking"
 * 2. Set the data to:
 *    {
 *      "at_username": "your_africastalking_username",
 *      "at_api_key": "your_africastalking_api_key",
 *      "playground_mode": false
 *    }
 * 
 * For testing/playground mode (uses sandbox credentials):
 *    Set "playground_mode": true in the config
 * 
 * If no config is found, the system defaults to sandbox mode (safe fallback).
 */

export default defineConfig({
  satellite: {
    ids: {
      development: "atbka-rp777-77775-aaaaq-cai",
      test: "jx5yt-yyaaa-aaaal-abzbq-cai", // Docker emulator satellite
      preview: "64njw-oiaaa-aaaal-asppa-cai",
      production: "dkk74-oyaaa-aaaal-askxq-cai"
    },
    source: "build",
    predeploy: ["pnpm run build"],
    collections: {
      datastore: [
        {
          collection: "agents",
          read: "public",   // Discovery directory for agents
          write: "managed", // Agents edit their own profile
          memory: "stable",
        },
        {
          collection: "config",
          read: "controllers",  // Holds AfricasTalking + third-party creds
          write: "controllers",
          memory: "stable",
        },
      ],
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
    satellite: {}
  }
});
