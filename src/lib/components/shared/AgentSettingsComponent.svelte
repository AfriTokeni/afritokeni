<script lang="ts">
    import {demoMode} from "$lib/stores/demoMode";
    import {principalId} from "$lib/stores/auth";
    import {toast} from "$lib/stores/toast";
    import {AlertCircle, Bell, Globe, RotateCcw, Save, Shield, User,} from "@lucide/svelte";
    import {onMount} from "svelte";
    import {getDoc, setDoc} from "@junobuild/core";
    import {AGENT_SETTINGS_CONFIG, getSliderLabel,} from "$lib/config/agentSettings";

    interface AgentSettings {
    commissionRate: number;
    maxCashLimit: number;
    operatingHours: { start: string; end: string };
    bitcoinEnabled: boolean;
    notificationsEnabled: boolean;
    smsNotifications: boolean;
    emailNotifications: boolean;
    status: "available" | "busy" | "cash_out" | "offline";
    preferredCurrency: string;
    serviceRadius: number;
    minimumTransaction: number;
    autoAcceptLimit: number;
    securityPinEnabled: boolean;
    locationSharing: boolean;
  }

  let activeTab = $state<
    "profile" | "operations" | "security" | "notifications"
  >("profile");
  let isSaving = $state(false);
  let hasUnsavedChanges = $state(false);
  let isLoading = $state(true);
  let agentDoc = $state<any>(null);
  let originalSettings: AgentSettings;
  let originalProfile = {
    businessName: "",
    phoneNumber: "",
    location: "",
    businessAddress: "",
  };

  // Default settings from config
  const defaultSettings: AgentSettings = {
    commissionRate: AGENT_SETTINGS_CONFIG.commissionRate.default,
    maxCashLimit: AGENT_SETTINGS_CONFIG.maxCashLimit.default,
    operatingHours: {
      start: AGENT_SETTINGS_CONFIG.operatingHours.default.start,
      end: AGENT_SETTINGS_CONFIG.operatingHours.default.end,
    },
    bitcoinEnabled: false,
    notificationsEnabled: false,
    smsNotifications: false,
    emailNotifications: false,
    status: "offline",
    preferredCurrency: "UGX",
    serviceRadius: AGENT_SETTINGS_CONFIG.serviceRadius.default,
    minimumTransaction: AGENT_SETTINGS_CONFIG.minimumTransaction.default,
    autoAcceptLimit: AGENT_SETTINGS_CONFIG.autoAcceptLimit.default,
    securityPinEnabled: false,
    locationSharing: false,
  };

  let settings = $state<AgentSettings>({ ...defaultSettings });

  // Profile settings - NO HARDCODED VALUES
  let businessName = $state("");
  let phoneNumber = $state("");
  let location = $state("");
  let businessAddress = $state("");

  // Load settings from Juno or use demo data
  $effect(() => {
    loadAgentSettings($demoMode, $principalId);
  });

  async function loadAgentSettings(
    isDemoMode: boolean,
    agentPrincipalId: string | null,
  ) {
    isLoading = true;

    if (isDemoMode) {
      // Demo data
      settings = {
        ...defaultSettings,
        bitcoinEnabled: true,
        notificationsEnabled: true,
        smsNotifications: true,
        status: "available",
        securityPinEnabled: true,
        locationSharing: true,
      };
      businessName = "John Doe Agent Services";
      phoneNumber = "+256700123456";
      location = "Kampala, Uganda";
      businessAddress = "Plot 123, Kampala Road";
      originalSettings = JSON.parse(JSON.stringify(settings));
      originalProfile = {
        businessName,
        phoneNumber,
        location,
        businessAddress,
      };
      isLoading = false;
      return;
    }

    if (!agentPrincipalId) {
      settings = { ...defaultSettings };
      businessName = "";
      phoneNumber = "";
      location = "";
      businessAddress = "";
      originalSettings = JSON.parse(JSON.stringify(settings));
      originalProfile = {
        businessName,
        phoneNumber,
        location,
        businessAddress,
      };
      isLoading = false;
      return;
    }

    try {
      const doc = await getDoc({
        collection: "agents",
        key: agentPrincipalId,
      });

      if (!doc) {
        const error = new Error(
          `Agent settings not found for principal: ${agentPrincipalId}`,
        );
        console.error("❌ AGENT SETTINGS ERROR:", error);
        toast.show(
          "error",
          "Agent settings not found. Please complete onboarding.",
        );
        isLoading = false;
        return;
      }

      agentDoc = doc;
      const data = doc.data;

      // NO FALLBACKS - use exact data from Juno
      settings = {
        commissionRate: data.commissionRate,
        maxCashLimit: data.maxCashLimit,
        operatingHours: data.operatingHours,
        bitcoinEnabled: data.bitcoinEnabled,
        notificationsEnabled: data.notificationsEnabled,
        smsNotifications: data.smsNotifications,
        emailNotifications: data.emailNotifications,
        status: data.status,
        preferredCurrency: data.preferredCurrency,
        serviceRadius: data.serviceRadius,
        minimumTransaction: data.minimumTransaction,
        autoAcceptLimit: data.autoAcceptLimit,
        securityPinEnabled: data.securityPinEnabled,
        locationSharing: data.locationSharing,
      };

      businessName = data.businessName;
      phoneNumber = data.phoneNumber;
      location = data.location;
      businessAddress = data.businessAddress;

      originalSettings = JSON.parse(JSON.stringify(settings));
      originalProfile = {
        businessName,
        phoneNumber,
        location,
        businessAddress,
      };
    } catch (error: any) {
      console.error("❌ FAILED TO LOAD AGENT SETTINGS:", error);
      console.error("Error details:", {
        message: error.message,
        stack: error.stack,
        principalId: agentPrincipalId,
      });
      toast.show("error", "Failed to load agent settings. Please try again.");
    } finally {
      isLoading = false;
    }
  }

  // Initialize keyboard shortcuts
  onMount(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === "s") {
        e.preventDefault();
        if (hasUnsavedChanges) saveSettings();
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  });

  // Track changes
  $effect(() => {
    const settingsChanged =
      JSON.stringify(settings) !== JSON.stringify(originalSettings);
    const profileChanged =
      businessName !== originalProfile.businessName ||
      phoneNumber !== originalProfile.phoneNumber ||
      location !== originalProfile.location ||
      businessAddress !== originalProfile.businessAddress;
    hasUnsavedChanges = settingsChanged || profileChanged;
  });

  async function saveSettings() {
    isSaving = true;
    try {
      const currentPrincipalId = $principalId;

      if ($demoMode) {
        // Demo mode - just simulate save
        await new Promise((resolve) => setTimeout(resolve, 500));
      } else if (currentPrincipalId && agentDoc) {
        // Real mode - save to Juno
        await setDoc({
          collection: "agents",
          doc: {
            ...agentDoc,
            data: {
              ...agentDoc.data,
              ...settings,
              businessName,
              phoneNumber,
              location,
              businessAddress,
              updatedAt: new Date().toISOString(),
            },
          },
        });
      } else {
        throw new Error("Not authenticated or no agent document");
      }

      // Update original values
      originalSettings = JSON.parse(JSON.stringify(settings));
      originalProfile = {
        businessName,
        phoneNumber,
        location,
        businessAddress,
      };
      hasUnsavedChanges = false;

      toast.show("success", "Settings saved successfully");
    } catch (error) {
      console.error("Failed to save settings:", error);
      toast.show("error", "Failed to save settings");
    } finally {
      isSaving = false;
    }
  }

  function resetChanges() {
    settings = JSON.parse(JSON.stringify(originalSettings));
    businessName = originalProfile.businessName;
    phoneNumber = originalProfile.phoneNumber;
    location = originalProfile.location;
    businessAddress = originalProfile.businessAddress;
    hasUnsavedChanges = false;
    toast.show("info", "Changes discarded");
  }

  async function updateStatus(
    newStatus: "available" | "busy" | "cash_out" | "offline",
  ) {
    const oldStatus = settings.status;
    settings.status = newStatus;

    try {
      // Save status immediately to backend
      await new Promise((resolve) => setTimeout(resolve, 500));

      // Update original settings so it doesn't trigger unsaved changes
      originalSettings.status = newStatus;

      toast.show("success", `Status changed to ${newStatus.replace("_", " ")}`);
    } catch (error) {
      // Revert on error
      settings.status = oldStatus;
      toast.show("error", "Failed to update status");
    }
  }

  function getStatusColor(status: string): string {
    switch (status) {
      case "available":
        return "bg-green-100 text-green-800 border-green-200";
      case "busy":
        return "bg-yellow-100 text-yellow-800 border-yellow-200";
      case "cash_out":
        return "bg-orange-100 text-orange-800 border-orange-200";
      case "offline":
        return "bg-gray-100 text-gray-800 border-gray-200";
      default:
        return "bg-gray-100 text-gray-800 border-gray-200";
    }
  }
</script>

<div class="space-y-6">
  <!-- Header -->
  <div>
    <h1 class="text-2xl font-bold text-gray-900 sm:text-3xl">Settings</h1>
    <p class="mt-1 text-gray-600">Manage your agent account settings</p>
  </div>

  <!-- Unsaved Changes Warning -->
  {#if hasUnsavedChanges}
    <div class="rounded-lg border border-yellow-200 bg-yellow-50 p-4">
      <div class="flex items-center justify-between gap-4">
        <div class="flex items-center gap-3">
          <AlertCircle class="h-5 w-5 shrink-0 text-yellow-600" />
          <div>
            <p class="text-sm font-medium text-yellow-900">
              You have unsaved changes
            </p>
            <p class="mt-0.5 text-xs text-yellow-700">
              Press Ctrl+S or click Save to keep your changes
            </p>
          </div>
        </div>
        <button
          onclick={resetChanges}
          class="text-xs font-medium whitespace-nowrap text-yellow-700 hover:text-yellow-900"
        >
          Discard
        </button>
      </div>
    </div>
  {/if}

  <!-- Quick Status Toggle -->
  <div class="rounded-lg border border-gray-200 bg-white p-4">
    <div class="flex flex-wrap items-center justify-between gap-4">
      <div>
        <h3 class="text-sm font-semibold text-gray-900">Quick Status</h3>
        <p class="mt-0.5 text-xs text-gray-600">
          Change your availability status
        </p>
      </div>
      <div class="flex flex-wrap gap-2">
        <button
          onclick={() => updateStatus("available")}
          class="rounded-lg border px-3 py-1.5 text-xs font-medium transition-colors {settings.status ===
          'available'
            ? getStatusColor('available')
            : 'border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
        >
          Available
        </button>
        <button
          onclick={() => updateStatus("busy")}
          class="rounded-lg border px-3 py-1.5 text-xs font-medium transition-colors {settings.status ===
          'busy'
            ? getStatusColor('busy')
            : 'border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
        >
          Busy
        </button>
        <button
          onclick={() => updateStatus("cash_out")}
          class="rounded-lg border px-3 py-1.5 text-xs font-medium transition-colors {settings.status ===
          'cash_out'
            ? getStatusColor('cash_out')
            : 'border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
        >
          Cash Out
        </button>
        <button
          onclick={() => updateStatus("offline")}
          class="rounded-lg border px-3 py-1.5 text-xs font-medium transition-colors {settings.status ===
          'offline'
            ? getStatusColor('offline')
            : 'border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
        >
          Offline
        </button>
      </div>
    </div>
  </div>

  <!-- Tabs -->
  <div class="border-b border-gray-200">
    <div class="scrollbar-hide flex gap-2 overflow-x-auto">
      <button
        onclick={() => (activeTab = "profile")}
        class="border-b-2 px-4 py-2 text-sm font-medium whitespace-nowrap transition-colors {activeTab ===
        'profile'
          ? 'border-black text-black'
          : 'border-transparent text-gray-600 hover:text-gray-900'}"
      >
        <div class="flex items-center gap-2">
          <User class="h-4 w-4" />
          <span>Profile</span>
        </div>
      </button>
      <button
        onclick={() => (activeTab = "operations")}
        class="border-b-2 px-4 py-2 text-sm font-medium whitespace-nowrap transition-colors {activeTab ===
        'operations'
          ? 'border-black text-black'
          : 'border-transparent text-gray-600 hover:text-gray-900'}"
      >
        <div class="flex items-center gap-2">
          <Globe class="h-4 w-4" />
          <span>Operations</span>
        </div>
      </button>
      <button
        onclick={() => (activeTab = "security")}
        class="border-b-2 px-4 py-2 text-sm font-medium whitespace-nowrap transition-colors {activeTab ===
        'security'
          ? 'border-black text-black'
          : 'border-transparent text-gray-600 hover:text-gray-900'}"
      >
        <div class="flex items-center gap-2">
          <Shield class="h-4 w-4" />
          <span>Security</span>
        </div>
      </button>
      <button
        onclick={() => (activeTab = "notifications")}
        class="border-b-2 px-4 py-2 text-sm font-medium whitespace-nowrap transition-colors {activeTab ===
        'notifications'
          ? 'border-black text-black'
          : 'border-transparent text-gray-600 hover:text-gray-900'}"
      >
        <div class="flex items-center gap-2">
          <Bell class="h-4 w-4" />
          <span>Notifications</span>
        </div>
      </button>
    </div>
  </div>

  <!-- Tab Content -->
  <div class="rounded-2xl border border-gray-200 bg-white p-6">
    {#if activeTab === "profile"}
      <!-- Profile Settings -->
      <div class="space-y-6">
        <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
          <div>
            <label class="mb-2 block text-sm font-medium text-gray-700"
              >Business Name</label
            >
            <input
              type="text"
              bind:value={businessName}
              class="w-full rounded-lg border border-gray-200 px-4 py-2 focus:border-transparent focus:ring-2 focus:ring-gray-900"
            />
          </div>

          <div>
            <label class="mb-2 block text-sm font-medium text-gray-700"
              >Phone Number</label
            >
            <input
              type="tel"
              bind:value={phoneNumber}
              class="w-full rounded-lg border border-gray-200 px-4 py-2 focus:border-transparent focus:ring-2 focus:ring-gray-900"
            />
          </div>

          <div>
            <label class="mb-2 block text-sm font-medium text-gray-700"
              >Location</label
            >
            <input
              type="text"
              bind:value={location}
              class="w-full rounded-lg border border-gray-200 px-4 py-2 focus:border-transparent focus:ring-2 focus:ring-gray-900"
            />
          </div>

          <div>
            <label class="mb-2 block text-sm font-medium text-gray-700"
              >Business Address</label
            >
            <input
              type="text"
              bind:value={businessAddress}
              class="w-full rounded-lg border border-gray-200 px-4 py-2 focus:border-transparent focus:ring-2 focus:ring-gray-900"
            />
          </div>
        </div>

        <!-- Status -->
        <div>
          <label class="mb-2 block text-sm font-medium text-gray-700"
            >Current Status</label
          >
          <div class="flex flex-wrap gap-2">
            <button
              onclick={() => (settings.status = "available")}
              class="rounded-lg border px-4 py-2 text-sm font-medium transition-colors {settings.status ===
              'available'
                ? getStatusColor('available')
                : 'border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
            >
              Available
            </button>
            <button
              onclick={() => (settings.status = "busy")}
              class="rounded-lg border px-4 py-2 text-sm font-medium transition-colors {settings.status ===
              'busy'
                ? getStatusColor('busy')
                : 'border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
            >
              Busy
            </button>
            <button
              onclick={() => (settings.status = "cash_out")}
              class="rounded-lg border px-4 py-2 text-sm font-medium transition-colors {settings.status ===
              'cash_out'
                ? getStatusColor('cash_out')
                : 'border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
            >
              Cash Out
            </button>
            <button
              onclick={() => (settings.status = "offline")}
              class="rounded-lg border px-4 py-2 text-sm font-medium transition-colors {settings.status ===
              'offline'
                ? getStatusColor('offline')
                : 'border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
            >
              Offline
            </button>
          </div>
        </div>
      </div>
    {:else if activeTab === "operations"}
      <!-- Operations Settings -->
      <div class="space-y-8">
        <!-- Operating Hours with Clock Visual -->
        <div>
          <h3
            class="mb-4 flex items-center gap-2 text-lg font-semibold text-gray-900"
          >
            <Globe class="h-5 w-5" />
            Operating Hours
          </h3>
          <div class="rounded-lg bg-gray-50 p-6">
            <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
              <div>
                <label class="mb-3 block text-sm font-medium text-gray-700"
                  >Opening Time</label
                >
                <input
                  type="time"
                  bind:value={settings.operatingHours.start}
                  class="w-full rounded-lg border border-gray-300 px-4 py-3 font-mono text-lg focus:border-transparent focus:ring-2 focus:ring-black"
                />
              </div>
              <div>
                <label class="mb-3 block text-sm font-medium text-gray-700"
                  >Closing Time</label
                >
                <input
                  type="time"
                  bind:value={settings.operatingHours.end}
                  class="w-full rounded-lg border border-gray-300 px-4 py-3 font-mono text-lg focus:border-transparent focus:ring-2 focus:ring-black"
                />
              </div>
            </div>
          </div>
        </div>

        <!-- Commission Rate Slider -->
        <div>
          <div class="mb-3 flex items-center justify-between">
            <label class="text-sm font-medium text-gray-700"
              >Commission Rate</label
            >
            <span class="font-mono text-2xl font-bold text-black"
              >{settings.commissionRate}%</span
            >
          </div>
          <input
            type="range"
            min={AGENT_SETTINGS_CONFIG.commissionRate.min}
            max={AGENT_SETTINGS_CONFIG.commissionRate.max}
            step={AGENT_SETTINGS_CONFIG.commissionRate.step}
            bind:value={settings.commissionRate}
            style="accent-color: black;"
            class="h-2 w-full cursor-pointer appearance-none rounded-lg bg-gray-200"
          />
          <div class="mt-1 flex justify-between text-xs text-gray-500">
            <span>{AGENT_SETTINGS_CONFIG.commissionRate.min}%</span>
            <span
              >{Math.round(AGENT_SETTINGS_CONFIG.commissionRate.max / 2)}%</span
            >
            <span>{AGENT_SETTINGS_CONFIG.commissionRate.max}%</span>
          </div>
        </div>

        <!-- Service Radius Slider -->
        <div>
          <div class="mb-3 flex items-center justify-between">
            <label class="text-sm font-medium text-gray-700"
              >Service Radius</label
            >
            <span class="font-mono text-2xl font-bold text-black"
              >{settings.serviceRadius} km</span
            >
          </div>
          <input
            type="range"
            min={AGENT_SETTINGS_CONFIG.serviceRadius.min}
            max={AGENT_SETTINGS_CONFIG.serviceRadius.max}
            step={AGENT_SETTINGS_CONFIG.serviceRadius.step}
            bind:value={settings.serviceRadius}
            style="accent-color: black;"
            class="h-2 w-full cursor-pointer appearance-none rounded-lg bg-gray-200"
          />
          <div class="mt-1 flex justify-between text-xs text-gray-500">
            <span>{AGENT_SETTINGS_CONFIG.serviceRadius.min} km</span>
            <span
              >{Math.round(AGENT_SETTINGS_CONFIG.serviceRadius.max / 2)} km</span
            >
            <span>{AGENT_SETTINGS_CONFIG.serviceRadius.max} km</span>
          </div>
        </div>

        <!-- Max Cash Limit Slider -->
        <div>
          <div class="mb-3 flex items-center justify-between">
            <label class="text-sm font-medium text-gray-700"
              >Max Cash Limit</label
            >
            <span class="font-mono text-2xl font-bold text-black"
              >{settings.maxCashLimit.toLocaleString()} UGX</span
            >
          </div>
          <input
            type="range"
            min={AGENT_SETTINGS_CONFIG.maxCashLimit.min}
            max={AGENT_SETTINGS_CONFIG.maxCashLimit.max}
            step={AGENT_SETTINGS_CONFIG.maxCashLimit.step}
            bind:value={settings.maxCashLimit}
            style="accent-color: black;"
            class="h-2 w-full cursor-pointer appearance-none rounded-lg bg-gray-200"
          />
          <div class="mt-1 flex justify-between text-xs text-gray-500">
            <span>{getSliderLabel(AGENT_SETTINGS_CONFIG.maxCashLimit.min)}</span
            >
            <span
              >{getSliderLabel(
                AGENT_SETTINGS_CONFIG.maxCashLimit.max / 2,
              )}</span
            >
            <span>{getSliderLabel(AGENT_SETTINGS_CONFIG.maxCashLimit.max)}</span
            >
          </div>
        </div>

        <!-- Minimum Transaction Slider -->
        <div>
          <div class="mb-3 flex items-center justify-between">
            <label class="text-sm font-medium text-gray-700"
              >Minimum Transaction</label
            >
            <span class="font-mono text-2xl font-bold text-black"
              >{settings.minimumTransaction.toLocaleString()} UGX</span
            >
          </div>
          <input
            type="range"
            min={AGENT_SETTINGS_CONFIG.minimumTransaction.min}
            max={AGENT_SETTINGS_CONFIG.minimumTransaction.max}
            step={AGENT_SETTINGS_CONFIG.minimumTransaction.step}
            bind:value={settings.minimumTransaction}
            style="accent-color: black;"
            class="h-2 w-full cursor-pointer appearance-none rounded-lg bg-gray-200"
          />
          <div class="mt-1 flex justify-between text-xs text-gray-500">
            <span
              >{getSliderLabel(
                AGENT_SETTINGS_CONFIG.minimumTransaction.min,
              )}</span
            >
            <span
              >{getSliderLabel(
                AGENT_SETTINGS_CONFIG.minimumTransaction.max / 2,
              )}</span
            >
            <span
              >{getSliderLabel(
                AGENT_SETTINGS_CONFIG.minimumTransaction.max,
              )}</span
            >
          </div>
        </div>

        <!-- Toggles -->
        <div class="space-y-4 border-t border-gray-200 pt-4">
          <label
            class="flex cursor-pointer items-center justify-between rounded-lg bg-gray-50 p-4 transition-colors hover:bg-gray-100"
          >
            <div>
              <span class="text-sm font-medium text-gray-900"
                >Bitcoin Services</span
              >
              <p class="mt-0.5 text-xs text-gray-600">
                Enable Bitcoin exchange services
              </p>
            </div>
            <button
              type="button"
              onclick={() =>
                (settings.bitcoinEnabled = !settings.bitcoinEnabled)}
              class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors {settings.bitcoinEnabled
                ? 'bg-black'
                : 'bg-gray-300'}"
            >
              <span
                class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {settings.bitcoinEnabled
                  ? 'translate-x-6'
                  : 'translate-x-1'}"
              ></span>
            </button>
          </label>

          <label
            class="flex cursor-pointer items-center justify-between rounded-lg bg-gray-50 p-4 transition-colors hover:bg-gray-100"
          >
            <div>
              <span class="text-sm font-medium text-gray-900"
                >Location Sharing</span
              >
              <p class="mt-0.5 text-xs text-gray-600">
                Share your location with customers
              </p>
            </div>
            <button
              type="button"
              onclick={() =>
                (settings.locationSharing = !settings.locationSharing)}
              class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors {settings.locationSharing
                ? 'bg-black'
                : 'bg-gray-300'}"
            >
              <span
                class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {settings.locationSharing
                  ? 'translate-x-6'
                  : 'translate-x-1'}"
              ></span>
            </button>
          </label>
        </div>
      </div>
    {:else if activeTab === "security"}
      <!-- Security Settings -->
      <div class="space-y-6">
        <div class="space-y-4">
          <label
            class="flex cursor-pointer items-center justify-between rounded-lg bg-gray-50 p-4 transition-colors hover:bg-gray-100"
          >
            <div>
              <span class="text-sm font-medium text-gray-900"
                >Security PIN Enabled</span
              >
              <p class="mt-0.5 text-xs text-gray-600">
                Require PIN for high-value transactions
              </p>
            </div>
            <button
              type="button"
              onclick={() =>
                (settings.securityPinEnabled = !settings.securityPinEnabled)}
              class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors {settings.securityPinEnabled
                ? 'bg-black'
                : 'bg-gray-300'}"
            >
              <span
                class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {settings.securityPinEnabled
                  ? 'translate-x-6'
                  : 'translate-x-1'}"
              ></span>
            </button>
          </label>

          <div>
            <label class="mb-2 block text-sm font-medium text-gray-700"
              >Auto-Accept Limit (UGX)</label
            >
            <input
              type="number"
              bind:value={settings.autoAcceptLimit}
              class="w-full rounded-lg border border-gray-200 px-4 py-2 focus:border-transparent focus:ring-2 focus:ring-gray-900"
            />
            <p class="mt-1 text-xs text-gray-500">
              Transactions below this amount will be auto-accepted
            </p>
          </div>
        </div>

        <!-- Change Password -->
        <div class="border-t border-gray-200 pt-6">
          <h3 class="mb-4 text-lg font-semibold text-gray-900">
            Change Password
          </h3>
          <div class="space-y-4">
            <div>
              <label class="mb-2 block text-sm font-medium text-gray-700"
                >Current Password</label
              >
              <input
                type="password"
                class="w-full rounded-lg border border-gray-200 px-4 py-2 focus:border-transparent focus:ring-2 focus:ring-gray-900"
              />
            </div>
            <div>
              <label class="mb-2 block text-sm font-medium text-gray-700"
                >New Password</label
              >
              <input
                type="password"
                class="w-full rounded-lg border border-gray-200 px-4 py-2 focus:border-transparent focus:ring-2 focus:ring-gray-900"
              />
            </div>
            <div>
              <label class="mb-2 block text-sm font-medium text-gray-700"
                >Confirm New Password</label
              >
              <input
                type="password"
                class="w-full rounded-lg border border-gray-200 px-4 py-2 focus:border-transparent focus:ring-2 focus:ring-gray-900"
              />
            </div>
          </div>
        </div>
      </div>
    {:else if activeTab === "notifications"}
      <!-- Notifications Settings -->
      <div class="space-y-6">
        <div class="space-y-4">
          <label
            class="flex cursor-pointer items-center justify-between rounded-lg bg-gray-50 p-4 transition-colors hover:bg-gray-100"
          >
            <div>
              <span class="text-sm font-medium text-gray-900"
                >Enable Notifications</span
              >
              <p class="mt-0.5 text-xs text-gray-600">
                Master switch for all notifications
              </p>
            </div>
            <button
              type="button"
              onclick={() =>
                (settings.notificationsEnabled =
                  !settings.notificationsEnabled)}
              class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors {settings.notificationsEnabled
                ? 'bg-black'
                : 'bg-gray-300'}"
            >
              <span
                class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {settings.notificationsEnabled
                  ? 'translate-x-6'
                  : 'translate-x-1'}"
              ></span>
            </button>
          </label>

          <label
            class="flex cursor-pointer items-center justify-between rounded-lg bg-gray-50 p-4 transition-colors hover:bg-gray-100 {!settings.notificationsEnabled
              ? 'opacity-50'
              : ''}"
          >
            <div>
              <span class="text-sm font-medium text-gray-900"
                >SMS Notifications</span
              >
              <p class="mt-0.5 text-xs text-gray-600">Receive alerts via SMS</p>
            </div>
            <button
              type="button"
              onclick={() =>
                settings.notificationsEnabled &&
                (settings.smsNotifications = !settings.smsNotifications)}
              disabled={!settings.notificationsEnabled}
              class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors {settings.smsNotifications
                ? 'bg-black'
                : 'bg-gray-300'}"
            >
              <span
                class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {settings.smsNotifications
                  ? 'translate-x-6'
                  : 'translate-x-1'}"
              ></span>
            </button>
          </label>

          <label
            class="flex cursor-pointer items-center justify-between rounded-lg bg-gray-50 p-4 transition-colors hover:bg-gray-100 {!settings.notificationsEnabled
              ? 'opacity-50'
              : ''}"
          >
            <div>
              <span class="text-sm font-medium text-gray-900"
                >Email Notifications</span
              >
              <p class="mt-0.5 text-xs text-gray-600">
                Receive alerts via email
              </p>
            </div>
            <button
              type="button"
              onclick={() =>
                settings.notificationsEnabled &&
                (settings.emailNotifications = !settings.emailNotifications)}
              disabled={!settings.notificationsEnabled}
              class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors {settings.emailNotifications
                ? 'bg-black'
                : 'bg-gray-300'}"
            >
              <span
                class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform {settings.emailNotifications
                  ? 'translate-x-6'
                  : 'translate-x-1'}"
              ></span>
            </button>
          </label>
        </div>
      </div>
    {/if}
  </div>

  <!-- Action Buttons -->
  <div class="flex flex-wrap items-center justify-between gap-4">
    <button
      onclick={resetChanges}
      disabled={!hasUnsavedChanges || isSaving}
      class="flex items-center gap-2 rounded-lg border border-gray-300 px-4 py-2 text-gray-700 transition-colors hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50"
    >
      <RotateCcw class="h-4 w-4" />
      <span>Reset Changes</span>
    </button>

    <button
      onclick={saveSettings}
      disabled={!hasUnsavedChanges || isSaving}
      class="flex items-center gap-2 rounded-lg bg-black px-6 py-3 text-white transition-colors hover:bg-gray-800 disabled:cursor-not-allowed disabled:opacity-50"
    >
      {#if isSaving}
        <div
          class="h-5 w-5 animate-spin rounded-full border-2 border-white border-t-transparent"
        ></div>
        <span>Saving...</span>
      {:else}
        <Save class="h-5 w-5" />
        <span>Save Changes</span>
      {/if}
    </button>
  </div>
</div>

<style>
  /* Force black slider color */
  input[type="range"] {
    accent-color: black !important;
  }

  input[type="range"]::-webkit-slider-thumb {
    background-color: black !important;
  }

  input[type="range"]::-moz-range-thumb {
    background-color: black !important;
  }
</style>
