import { defineConfig } from "@junobuild/config";

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
      datastore: [
        {
          collection: "users",
          read: "managed",
          write: "managed",
          memory: "stable",
        },
        {
          collection: "agents",
          read: "public",   // Agent profiles are public for discovery
          write: "managed", // Agents can only edit their own profile
          memory: "stable",
        },
        {
          collection: "agent_customers",
          read: "managed",  // Agents can only see their own customers
          write: "managed",
          memory: "stable",
        },
        {
          collection: "agent_reviews",
          read: "public",   // Reviews are public
          write: "managed", // Users can write their own reviews
          memory: "stable",
        },
        {
          collection: "user_roles",
          read: "managed",
          write: "managed",
          memory: "stable",
        },
        {
          collection: "user_pins",
          read: "managed",  // Users can only see their own PIN
          write: "managed",
          memory: "stable",
        },
        {
          collection: "balances",
          read: "managed",
          write: "controllers", // Only backend can modify balances
          memory: "stable",
        },
        {
          collection: "transactions",
          read: "managed",
          write: "controllers",
          memory: "stable",
        },
        {
          collection: "deposit_requests",
          read: "managed",
          write: "managed",
          memory: "stable",
        },
        {
          collection: "escrow_transactions",
          read: "managed",
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
      type: "docker",
    },
  },
});
