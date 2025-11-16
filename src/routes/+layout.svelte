<script lang="ts">
  import "../app.css";
  import DemoModeBanner from "$lib/components/shared/DemoModeBanner.svelte";
  import ToastContainer from "$lib/components/shared/ToastContainer.svelte";
  import { onMount } from "svelte";
  import {
    initSatellite,
    onAuthStateChange,
    type User as JunoUser,
  } from "@junobuild/core";
  import { initJunoAuth, junoInitialized } from "$lib/stores/auth";
  import { browser } from "$app/environment";
  import { goto } from "$app/navigation";

  let { children } = $props();
  let isCheckingRole = $state(false);

  async function checkAndRedirectUser(junoUser: JunoUser) {
    if (isCheckingRole) return;

    isCheckingRole = true;
    try {
      const currentPath = window.location.pathname;

      // Allow access to public pages (homepage, info pages, etc.)
      const publicPaths = [
        "/",
        "/how-it-works",
        "/pricing",
        "/whitepaper",
        "/ussd",
        "/info",
      ];
      const isPublicPage = publicPaths.some(
        (path) => currentPath === path || currentPath.startsWith("/info/"),
      );

      // Get user profile from user_canister by principal ID
      try {
        // Dynamic import to avoid SSR issues with process.env
        const { userCanisterService } = await import(
          "$lib/services/icp/canisters/userCanisterService"
        );
        // Use update call (not query) since it makes inter-canister calls to data_canister
        const userProfile = await userCanisterService.getUserByPrincipalUpdate(
          junoUser.key,
        );

        if (userProfile) {
          // User exists in canister - check their role
          const role = userProfile.user_type.toLowerCase(); // "User", "Agent", or "Admin" → lowercase

          let targetDashboard = "/users/dashboard";
          if (role === "agent") {
            targetDashboard = "/agents/dashboard";
          } else if (role === "admin") {
            targetDashboard = "/admin/dashboard";
          }

          // Don't redirect if already on their dashboard or a route within it
          const dashboardPrefix = targetDashboard.split("/")[1]; // "users", "agents", or "admin"
          if (currentPath.startsWith(`/${dashboardPrefix}`)) {
            console.log(`✅ Already on correct dashboard area (${role})`);
            isCheckingRole = false;
            return;
          }

          // Redirect to dashboard from public pages or role selection
          if (isPublicPage || currentPath === "/auth/role-selection") {
            console.log(`🔄 Redirecting ${role} to ${targetDashboard}`);
            goto(targetDashboard);
          }
        } else {
          // User not found in canister - new user needs role selection
          if (currentPath !== "/auth/role-selection") {
            console.log("🔄 New user - redirecting to role selection");
            goto("/auth/role-selection");
          }
        }
      } catch (canisterError) {
        // User might not exist in canister yet - redirect to role selection
        console.log(
          "⚠️ User not found in canister, redirecting to role selection",
        );
        if (currentPath !== "/auth/role-selection") {
          goto("/auth/role-selection");
        }
      }
    } catch (error) {
      console.error("❌ Error checking user role:", error);
    } finally {
      isCheckingRole = false;
    }
  }

  onMount(() => {
    if (!browser) return;

    let unsubscribe: (() => void) | undefined;
    let authUnsubscribe: (() => void) | undefined;

    (async () => {
      try {
        const satelliteId = import.meta.env.VITE_SATELLITE_ID;
        if (!satelliteId) {
          throw new Error(
            "VITE_SATELLITE_ID not provided. Ensure the Juno Vite plugin is configured.",
          );
        }
        // Enable emulator for quick testing (auto-login with test principal)
        // Set to false to test real Internet Identity flow
        const useContainer = import.meta.env.DEV === true;
        console.log(
          `🚀 Initializing Juno with satellite ${satelliteId} (${useContainer ? "emulator" : "remote"})`,
        );
        await initSatellite({
          container: useContainer,
        });

        // Mark Juno as initialized
        junoInitialized.set(true);

        // Initialize auth subscription
        unsubscribe = initJunoAuth();

        // Subscribe to auth changes and redirect accordingly
        authUnsubscribe = onAuthStateChange((user) => {
          if (user) {
            console.log("👤 User authenticated, checking role...");
            checkAndRedirectUser(user);
          }
        });
      } catch (error) {
        console.error("❌ Failed to initialize Juno:", error);
        console.log(
          "⚠️  Continuing without Juno - you can still test Internet Identity sign-in",
        );
      }
    })();

    // Return cleanup function
    return () => {
      unsubscribe?.();
      authUnsubscribe?.();
    };
  });
</script>

<DemoModeBanner />
{@render children()}
<ToastContainer />
