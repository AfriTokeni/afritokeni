/**
 * Environment Configuration
 *
 * SvelteKit Environment Variable Conventions:
 *
 * PUBLIC VARS (client + server):
 * - Import from: $env/static/public
 * - Must be prefixed with: PUBLIC_
 * - Example: PUBLIC_DEMO_MODE, PUBLIC_SNS_GOVERNANCE_CANISTER
 * - Accessible everywhere (client and server)
 *
 * PRIVATE VARS (server-only):
 * - Import from: $env/static/private
 * - NO prefix (just the name)
 * - Example: AT_USERNAME, AT_API_KEY, RESEND_API_KEY
 * - Only accessible in: +server.ts, +page.server.ts, hooks.server.ts
 * - Will cause build error if imported in client code
 *
 * This file simply re-exports SvelteKit's env modules for convenience.
 */

// Re-export all public environment variables
// These are safe to use anywhere (client or server)
export * as PUBLIC_ENV from "$env/static/public";

// For server-side code, import private vars directly:
// import { AT_USERNAME, AT_API_KEY } from '$env/static/private';
