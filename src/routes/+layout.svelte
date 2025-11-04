<script lang="ts">
  import "../app.css";
  import DemoModeBanner from "$lib/components/shared/DemoModeBanner.svelte";
  import { onMount } from "svelte";
  import {
    getDoc,
    initSatellite,
    onAuthStateChange,
    type User as JunoUser,
  } from "@junobuild/core";
  import { initJunoAuth } from "$lib/stores/auth";
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

      if (isPublicPage) {
        console.log("✅ Public page - no redirect needed");
        isCheckingRole = false;
        return;
      }

      // Check if user has a role
      const roleDoc = await getDoc({
        collection: "user_roles",
        key: junoUser.key,
      });

      if (roleDoc?.data) {
        // User has existing role, redirect to dashboard ONLY if on auth pages
        const role = (roleDoc.data as any).role;

        // Only redirect if on auth/role-selection page
        if (currentPath === "/auth/role-selection") {
          if (role === "agent") {
            console.log("🔄 Redirecting agent to dashboard");
            goto("/agents/dashboard");
          } else if (role === "user") {
            console.log("🔄 Redirecting user to dashboard");
            goto("/users/dashboard");
          }
        }
      } else {
        // New user - redirect to role selection ONLY if trying to access protected routes
        const protectedPaths = ["/users", "/agents"];
        const isProtectedRoute = protectedPaths.some((path) =>
          currentPath.startsWith(path),
        );

        if (isProtectedRoute && currentPath !== "/auth/role-selection") {
          console.log(
            "🔄 New user accessing protected route - redirecting to role selection",
          );
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
        const useContainer = import.meta.env.DEV === true;
        console.log(
          `🚀 Initializing Juno with satellite ${satelliteId} (${useContainer ? "emulator" : "remote"})`,
        );
        await initSatellite({
          container: useContainer,
        });

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
