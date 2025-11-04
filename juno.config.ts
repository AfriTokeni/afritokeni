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
          read: "public",
          write: "public",
          memory: "stable",
        },
        {
          collection: "transactions",
          read: "public",
          write: "public",
          memory: "stable",
        },
        {
          collection: "balances",
          read: "public",
          write: "public",
          memory: "stable",
        },
        {
          collection: "agents",
          read: "public",
          write: "public",
          memory: "stable",
        },
        {
          collection: "user_roles",
          read: "public",
          write: "public",
          memory: "stable",
        },
        {
          collection: "sms_messages",
          read: "public",
          write: "public",
          memory: "stable",
        },
        {
          collection: "notes",
          read: "public",
          write: "public",
          memory: "stable",
        },
        {
          collection: "email_subscriptions",
          read: "public",
          write: "public",
          memory: "stable",
        },
        {
          collection: "kyc_submissions",
          read: "managed",
          write: "managed",
          memory: "stable",
        },
        {
          collection: "deposit_requests",
          read: "public",
          write: "public",
          memory: "stable",
        },
        {
          collection: "bitcoin_wallets",
          read: "public",
          write: "public",
          memory: "stable",
        },
        {
          collection: "bitcoin_transactions",
          read: "public",
          write: "public",
          memory: "stable",
        },
        {
          collection: "platform_revenue",
          read: "managed",
          write: "managed",
          memory: "stable",
        },
         {
          collection: "ckbtc_deposit_addresses",
          read: "public",
          write: "public",
          memory: "stable",
        },
        {
          collection: "ckbtc_transactions",
          read: "public",
          write: "public",
          memory: "stable",
        },
        {
          collection: "ckusdc_transactions",
          read: "public",
          write: "public",
          memory: "stable",
        },
        {
          collection: "pending_transactions",
          read: "public",
          write: "public",
          memory: "stable",
        },
        {
          collection: "escrow_transactions",
          read: "public",
          write: "public",
          memory: "stable",
        },
        {
          collection: "ussd_sessions",
          read: "public",
          write: "public",
          memory: "stable",
        },
        {
          collection: "demo_balances",
          read: "public",
          write: "public",
          memory: "stable",
        },
        {
          collection: "withdrawal_requests",
          read: "public",
          write: "public",
          memory: "stable",
        },
      ],
      storage: [
        {
          collection: "images",
          read: "managed",
          write: "managed",
          memory: "stable",
        },
        {
          collection: "profile-images",
          read: "public",
          write: "managed",
          memory: "stable",
          maxSize: 5242880n, // 5MB max file size
        },
        {
          collection: "kyc_documents",
          read: "public",
          write: "managed",
          memory: "stable",
        },
      ],
    },
  },
  emulator: {
    runner: {
      type: "docker",
    },
    skylab: {},
  },
});
