<script lang="ts">
  import { demoMode } from "$lib/stores/demoMode";
  import { principalId } from "$lib/stores/auth";
  import { toast } from "$lib/stores/toast";
  import { agentOperationsService, userCanisterService } from "$lib/services";
  import type { DepositTransaction } from "$/declarations/agent_canister/agent_canister.did";
  import {
    AlertCircle,
    CheckCircle,
    Clock,
    Info,
    Lock,
    MapPin,
    Phone,
    Search,
    X,
    XCircle,
  } from "@lucide/svelte";

  let showInstructions = $state(false);
  let showPinModal = $state(false);
  let pinForConfirmation = $state("");
  let pendingConfirmRequest = $state<DepositRequest | null>(null);

  interface DepositRequest {
    id: string;
    userId: string;
    userName: string;
    userPhone: string;
    amount: number;
    currency: string;
    code: string;
    status: "pending" | "confirmed" | "completed" | "rejected";
    createdAt: string;
    userLocation?: string;
    userPhoto?: string;
  }

  function getUserInitials(name: string): string {
    return name
      .split(" ")
      .map((n) => n[0])
      .join("")
      .toUpperCase()
      .slice(0, 2);
  }

  let depositRequests = $state<DepositRequest[]>([]);
  let selectedRequest = $state<DepositRequest | null>(null);
  let verificationCodes = $state<Record<string, string>>({});
  let isProcessing = $state(false);
  let loading = $state(true);
  let enrichingUsers = $state(false);
  let error = $state("");
  let filter = $state<
    "all" | "pending" | "confirmed" | "completed" | "rejected"
  >("pending");
  let searchQuery = $state("");

  // Auto-fetch when stores change
  $effect(() => {
    loadDepositRequests($demoMode, $principalId);
  });

  async function loadDepositRequests(
    isDemoMode: boolean,
    agentPrincipal: string | null,
  ) {
    // In demo mode, we don't need a principal
    if (!isDemoMode && !agentPrincipal) {
      depositRequests = [];
      loading = false;
      return;
    }

    try {
      loading = true;
      error = "";

      if (!isDemoMode && agentPrincipal) {
        // Fetch real deposits from agent canister
        const canisterDeposits =
          await agentOperationsService.getAgentDeposits(agentPrincipal);

        // Map to request format first
        const mappedRequests = canisterDeposits.map(
          mapDepositTransactionToRequest,
        );

        // Enrich with user data
        depositRequests = await enrichWithUserData(mappedRequests);
      } else {
        // Demo mode - empty array (demo data handled elsewhere)
        depositRequests = [];
      }
    } catch (err: any) {
      error = err.message || "Failed to load deposit requests";
      depositRequests = [];
    } finally {
      loading = false;
    }
  }

  /**
   * Enrich deposit requests with user data from user_canister
   */
  async function enrichWithUserData(
    requests: DepositRequest[],
  ): Promise<DepositRequest[]> {
    if (requests.length === 0) return requests;

    enrichingUsers = true;
    try {
      const enrichedRequests = await Promise.all(
        requests.map(async (req) => {
          try {
            // Fetch user profile from user_canister using user_id (phone number)
            const userProfile = await userCanisterService.getUserByPhone(
              req.userId,
            );

            // Extract phone_number from Candid optional type [] | [string]
            let phoneNumber: string;
            if (
              Array.isArray(userProfile.phone_number) &&
              userProfile.phone_number.length > 0 &&
              userProfile.phone_number[0] !== undefined
            ) {
              phoneNumber = userProfile.phone_number[0];
            } else if (
              !Array.isArray(userProfile.phone_number) &&
              userProfile.phone_number
            ) {
              phoneNumber = userProfile.phone_number;
            } else {
              phoneNumber = req.userId;
            }

            return {
              ...req,
              userName: `${userProfile.first_name} ${userProfile.last_name}`,
              userPhone: phoneNumber,
            };
          } catch (err) {
            // If user lookup fails, keep original data (user_id as fallback)
            console.warn(`Failed to fetch user data for ${req.userId}:`, err);
            return {
              ...req,
              userName: "Unknown User",
              userPhone: req.userId,
            };
          }
        }),
      );
      return enrichedRequests;
    } catch (err) {
      console.error("Error enriching user data:", err);
      return requests; // Return original requests if enrichment fails completely
    } finally {
      enrichingUsers = false;
    }
  }

  /**
   * Map canister DepositTransaction to component DepositRequest interface
   */
  function mapDepositTransactionToRequest(
    tx: DepositTransaction,
  ): DepositRequest {
    // Map canister status to component status
    let status: DepositRequest["status"] = "pending";
    if ("Confirmed" in tx.status) status = "confirmed";
    else if ("Cancelled" in tx.status) status = "rejected";
    else if ("Expired" in tx.status) status = "rejected";
    else if ("Pending" in tx.status) status = "pending";

    return {
      id: tx.id,
      userId: tx.user_id,
      userName: tx.user_id, // Will be enriched with actual name
      userPhone: tx.user_id, // Will be enriched with actual phone
      amount: Number(tx.amount),
      currency: tx.currency,
      code: tx.deposit_code,
      status,
      createdAt: new Date(Number(tx.timestamp) / 1_000_000).toISOString(),
      userLocation: undefined,
      userPhoto: undefined,
    };
  }

  async function handleVerifyCode(request: DepositRequest) {
    const currentCode = (verificationCodes[request.id] || "")
      .trim()
      .toUpperCase();
    const expectedCode = request.code.trim().toUpperCase();

    if (currentCode === expectedCode) {
      // Store request and show PIN modal for confirmation
      pendingConfirmRequest = request;
      showPinModal = true;
      error = "";
    } else {
      error = `Invalid deposit code. Expected: ${expectedCode}, Got: ${currentCode}`;
    }
  }

  async function handlePinSubmit() {
    if (!pendingConfirmRequest || !pinForConfirmation) {
      error = "PIN is required";
      return;
    }

    try {
      isProcessing = true;
      error = "";

      if ($demoMode) {
        // Demo: mark as confirmed
        depositRequests = depositRequests.map((r) =>
          r.id === pendingConfirmRequest!.id
            ? { ...r, status: "confirmed" as const }
            : r,
        );
        selectedRequest = { ...pendingConfirmRequest, status: "confirmed" };
        closePinModal();
      } else {
        // Real canister call - confirm deposit with agent's PIN
        if (!$principalId) {
          error = "Agent principal ID not found";
          return;
        }

        // Verify agent PIN first
        try {
          const pinValid = await userCanisterService.verifyPin(
            $principalId,
            pinForConfirmation,
          );

          if (!pinValid) {
            error = "Invalid PIN. Please try again.";
            return;
          }
        } catch (err: any) {
          error = err.message || "Failed to verify PIN";
          return;
        }

        // PIN is valid, confirm the deposit
        await agentOperationsService.confirmDeposit({
          depositCode: pendingConfirmRequest.code,
          agentId: $principalId,
          agentPin: pinForConfirmation,
        });

        await loadDepositRequests($demoMode, $principalId);
        selectedRequest = pendingConfirmRequest;
        closePinModal();
      }
    } catch (err: any) {
      error =
        err.message || "Failed to confirm deposit request. Please try again.";
    } finally {
      isProcessing = false;
    }
  }

  function closePinModal() {
    showPinModal = false;
    pinForConfirmation = "";
    pendingConfirmRequest = null;
  }

  async function handleConfirmDeposit(request: DepositRequest) {
    isProcessing = true;
    try {
      if ($demoMode) {
        // Demo: mark as completed (change status, don't remove)
        await new Promise((resolve) => setTimeout(resolve, 1000));
        depositRequests = depositRequests.map((r) =>
          r.id === request.id ? { ...r, status: "completed" as const } : r,
        );
        selectedRequest = null;
        verificationCodes[request.id] = "";
        error = "";
        toast.show(
          "success",
          `Deposit completed! ${formatAmount(request.amount, request.currency)} credited to ${request.userName}'s account.`,
        );
      } else {
        // Real canister call - deposit is already confirmed, just reload to show updated state
        await loadDepositRequests($demoMode, $principalId);
        selectedRequest = null;
        verificationCodes[request.id] = "";
        error = "";
        toast.show(
          "success",
          `Deposit completed! ${formatAmount(request.amount, request.currency)} credited to ${request.userName}'s account.`,
        );
      }
    } catch (err: any) {
      error = "Failed to process deposit. Please try again.";
      toast.show("error", "Failed to process deposit. Please try again.");
    } finally {
      isProcessing = false;
    }
  }

  async function handleRejectDeposit(request: DepositRequest) {
    isProcessing = true;
    try {
      if ($demoMode) {
        await new Promise((resolve) => setTimeout(resolve, 500));
        depositRequests = depositRequests.map((r) =>
          r.id === request.id ? { ...r, status: "rejected" as const } : r,
        );
        selectedRequest = null;
        verificationCodes[request.id] = "";
        error = "";
        toast.show(
          "info",
          `Deposit from ${request.userName} has been rejected.`,
        );
      } else {
        // Note: Agent canister doesn't currently have a "reject" endpoint
        // Deposits are either confirmed or expire automatically
        // This is a no-op for now - expired deposits will be filtered out
        error =
          "Rejection not implemented. Deposits expire automatically if not confirmed.";
        toast.show(
          "info",
          "Deposits expire automatically if not confirmed within the time limit.",
        );
      }
    } catch (err: any) {
      error = "Failed to reject deposit. Please try again.";
      toast.show("error", "Failed to reject deposit. Please try again.");
    } finally {
      isProcessing = false;
    }
  }

  const filteredRequests = $derived(
    depositRequests.filter((request) => {
      const statusMatch = filter === "all" || request.status === filter;
      const searchMatch =
        !searchQuery ||
        request.userName.toLowerCase().includes(searchQuery.toLowerCase()) ||
        request.userPhone.includes(searchQuery);
      return statusMatch && searchMatch;
    }),
  );

  function formatAmount(amount: number, currency: string): string {
    return new Intl.NumberFormat("en-UG", {
      style: "currency",
      currency: currency,
      minimumFractionDigits: 0,
    }).format(amount);
  }

  function getStatusColor(status: string): string {
    switch (status) {
      case "pending":
        return "text-yellow-600 bg-yellow-50 border-yellow-200";
      case "confirmed":
        return "text-blue-600 bg-blue-50 border-blue-200";
      case "completed":
        return "text-green-600 bg-green-50 border-green-200";
      case "rejected":
        return "text-red-600 bg-red-50 border-red-200";
      default:
        return "text-neutral-600 bg-neutral-50 border-neutral-200";
    }
  }
</script>

<!-- PIN Confirmation Modal -->
{#if showPinModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
    role="dialog"
    aria-labelledby="pin-modal-title"
    aria-modal="true"
  >
    <div
      class="w-full max-w-md rounded-2xl border border-gray-200 bg-white p-6 shadow-2xl"
    >
      <div class="mb-4 flex items-center justify-between">
        <h3 id="pin-modal-title" class="text-lg font-semibold text-gray-900">
          Confirm with PIN
        </h3>
        <button
          onclick={() => closePinModal()}
          class="text-gray-400 hover:text-gray-600"
          aria-label="Close modal"
        >
          <X class="h-5 w-5" />
        </button>
      </div>

      <p class="mb-4 text-sm text-gray-600">
        Enter your 4-digit PIN to confirm this deposit.
      </p>

      <div class="mb-4">
        <label
          for="agent-pin"
          class="mb-2 block text-sm font-medium text-gray-700"
        >
          Agent PIN
        </label>
        <div class="relative">
          <Lock
            class="absolute top-1/2 left-3 h-5 w-5 -translate-y-1/2 text-gray-400"
          />
          <input
            id="agent-pin"
            type="password"
            inputmode="numeric"
            maxlength="4"
            pattern="[0-9]*"
            bind:value={pinForConfirmation}
            placeholder="Enter 4-digit PIN"
            class="w-full rounded-lg border border-gray-300 py-3 pr-4 pl-10 text-center font-mono text-lg tracking-widest focus:border-transparent focus:ring-2 focus:ring-blue-500"
            autocomplete="off"
          />
        </div>
      </div>

      {#if error}
        <div class="mb-4 rounded-lg bg-red-50 p-3 text-sm text-red-700">
          {error}
        </div>
      {/if}

      <div class="flex gap-3">
        <button
          onclick={() => closePinModal()}
          disabled={isProcessing}
          class="flex-1 rounded-lg border border-gray-300 bg-white px-4 py-2.5 text-sm font-medium text-gray-700 transition-colors hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50"
        >
          Cancel
        </button>
        <button
          onclick={handlePinSubmit}
          disabled={isProcessing || pinForConfirmation.length !== 4}
          class="flex flex-1 items-center justify-center gap-2 rounded-lg bg-blue-600 px-4 py-2.5 text-sm font-medium text-white transition-colors hover:bg-blue-700 disabled:cursor-not-allowed disabled:bg-gray-300"
        >
          {#if isProcessing}
            <div
              class="h-4 w-4 animate-spin rounded-full border-2 border-white border-t-transparent"
            ></div>
            <span>Verifying...</span>
          {:else}
            <Lock class="h-4 w-4" />
            <span>Confirm</span>
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Error Message -->
{#if error && !showPinModal}
  <div
    class="mb-6 rounded-2xl border-2 border-red-500 bg-red-50 p-3 shadow-lg sm:p-4"
  >
    <div class="flex items-start">
      <AlertCircle
        class="mt-0.5 mr-2 h-5 w-5 shrink-0 text-red-600 sm:mr-3 sm:h-6 sm:w-6"
      />
      <div class="min-w-0 flex-1">
        <h3 class="mb-1 text-sm font-semibold text-red-900 sm:text-base">
          Verification Failed
        </h3>
        <p class="text-xs text-red-700 sm:text-sm">{error}</p>
      </div>
      <button
        onclick={() => (error = "")}
        class="shrink-0 text-red-400 hover:text-red-600"
      >
        <X class="h-4 w-4 sm:h-5 sm:w-5" />
      </button>
    </div>
  </div>
{/if}

<!-- Collapsible Instructions -->
{#if showInstructions}
  <div
    class="mb-6 rounded-2xl border border-blue-200 bg-blue-50 p-4 sm:p-5 md:p-6"
  >
    <div class="mb-2 flex items-start justify-between sm:mb-3">
      <h3 class="text-sm font-semibold text-blue-900 sm:text-base">
        Deposit Process:
      </h3>
      <button
        onclick={() => (showInstructions = false)}
        class="shrink-0 text-blue-600 hover:text-blue-800"
      >
        <X class="h-4 w-4" />
      </button>
    </div>
    <ol
      class="list-inside list-decimal space-y-1.5 text-xs text-blue-800 sm:space-y-2 sm:text-sm"
    >
      <li>Customer shows you their deposit code</li>
      <li>Verify the code matches the request</li>
      <li>Collect the cash amount from customer</li>
      <li>Enter your PIN to confirm the deposit</li>
      <li>Customer receives confirmation notification</li>
    </ol>
    <div class="mt-3 rounded-lg bg-blue-100 p-2.5 sm:mt-4 sm:p-3">
      <p class="text-xs font-medium text-blue-900 sm:text-sm">
        ⚠️ Always verify the deposit code before accepting cash. You will need
        to enter your PIN to confirm each deposit.
      </p>
    </div>
    {#if $demoMode}
      <div
        class="mt-3 rounded-lg border border-purple-200 bg-purple-100 p-2.5 sm:mt-4 sm:p-3"
      >
        <p class="text-xs font-medium text-purple-900 sm:text-sm">
          🎭 Demo: John (DEP8BL), Sarah (DEP6CP), David (DEPKY3)
        </p>
      </div>
    {/if}
  </div>
{/if}

<!-- Search and Filter -->
<div class="rounded-2xl border border-gray-200 bg-white p-4 sm:p-5 md:p-6">
  <!-- Header with Info Button -->
  <div class="mb-4 flex items-center justify-between">
    <h2 class="text-lg font-semibold text-gray-900 sm:text-xl">
      Deposit Requests
    </h2>
    <button
      onclick={() => (showInstructions = !showInstructions)}
      class="flex items-center gap-2 rounded-lg bg-blue-600 px-3 py-2 text-sm font-medium text-white shadow-sm transition-colors hover:bg-blue-700 sm:px-4"
    >
      <Info class="h-4 w-4 sm:h-5 sm:w-5" />
      <span>How it works</span>
    </button>
  </div>

  <!-- Search Bar -->
  <div class="mb-4 sm:mb-5 md:mb-6">
    <div class="relative">
      <Search
        class="absolute top-1/2 left-3 h-4 w-4 shrink-0 -translate-y-1/2 transform text-gray-400 sm:h-5 sm:w-5"
      />
      <input
        type="text"
        placeholder="Search by name or phone number..."
        bind:value={searchQuery}
        class="w-full rounded-lg border border-gray-300 py-2.5 pr-4 pl-9 text-sm focus:border-transparent focus:ring-2 focus:ring-gray-500 sm:py-3 sm:pl-10 sm:text-base"
      />
    </div>
  </div>

  <!-- Filter Tabs -->
  <div
    class="scrollbar-hide mb-4 flex space-x-1 overflow-x-auto rounded-lg bg-gray-100 p-1 sm:mb-5 md:mb-6"
  >
    {#each ["all", "pending", "confirmed", "completed", "rejected"] as tab}
      <button
        onclick={() => (filter = tab as any)}
        class="shrink-0 rounded-md px-2 py-1.5 text-xs font-medium transition-colors sm:px-4 sm:py-2 sm:text-sm {filter ===
        tab
          ? 'bg-white text-neutral-900 shadow-sm'
          : 'text-neutral-600 hover:text-neutral-900'}"
      >
        <span class="block whitespace-nowrap sm:inline"
          >{tab.charAt(0).toUpperCase() + tab.slice(1)}</span
        >
        <span
          class="ml-1 rounded-full bg-neutral-200 px-1.5 py-0.5 text-xs text-neutral-600 sm:ml-2 sm:px-2"
        >
          {tab === "all"
            ? depositRequests.length
            : depositRequests.filter((r) => r.status === tab).length}
        </span>
      </button>
    {/each}
  </div>

  <!-- Deposit Requests List -->
  <div class="space-y-3 sm:space-y-4">
    {#if loading || enrichingUsers}
      <div class="py-8 text-center sm:py-10 md:py-12">
        <div
          class="mx-auto mb-3 h-7 w-7 animate-spin rounded-full border-2 border-blue-600 border-t-transparent sm:mb-4 sm:h-8 sm:w-8"
        ></div>
        <p class="text-sm text-neutral-600 sm:text-base">
          {enrichingUsers
            ? "Loading user details..."
            : "Loading deposit requests..."}
        </p>
      </div>
    {:else if filteredRequests.length === 0}
      <div class="py-8 text-center sm:py-10 md:py-12">
        <Clock
          class="mx-auto mb-3 h-10 w-10 shrink-0 text-neutral-400 sm:mb-4 sm:h-12 sm:w-12"
        />
        <h3 class="mb-2 text-base font-semibold text-neutral-900 sm:text-lg">
          No deposit requests
        </h3>
        <p class="text-sm text-neutral-600 sm:text-base">
          {filter === "pending"
            ? "No pending deposits at the moment."
            : `No ${filter} deposits found.`}
        </p>
      </div>
    {:else}
      {#each filteredRequests as request (request.id)}
        <div
          class="space-y-3 rounded-2xl border border-gray-200 p-3 transition-colors hover:border-gray-300 sm:space-y-4 sm:p-4"
        >
          <!-- Header: User Info + Status -->
          <div class="flex items-start justify-between gap-2 sm:gap-3">
            <div class="flex min-w-0 flex-1 items-start space-x-2 sm:space-x-3">
              <!-- User Photo/Avatar -->
              <div
                class="h-10 w-10 shrink-0 overflow-hidden rounded-full border-2 border-gray-200 sm:h-12 sm:w-12"
              >
                {#if request.userPhoto}
                  <img
                    src={request.userPhoto}
                    alt={request.userName}
                    class="h-full w-full object-cover"
                  />
                {:else}
                  <div
                    class="flex h-full w-full items-center justify-center bg-black"
                  >
                    <span class="text-sm font-bold text-white sm:text-base">
                      {getUserInitials(request.userName)}
                    </span>
                  </div>
                {/if}
              </div>
              <div class="min-w-0 flex-1">
                <h3
                  class="truncate text-sm font-semibold text-gray-900 sm:text-base"
                >
                  {request.userName}
                </h3>
                <div
                  class="mt-0.5 flex flex-col text-xs text-gray-600 sm:text-sm"
                >
                  <div class="flex items-center space-x-1">
                    <Phone class="h-3 w-3 shrink-0 sm:h-3.5 sm:w-3.5" />
                    <span class="truncate">{request.userPhone}</span>
                  </div>
                  {#if request.userLocation}
                    <div class="mt-0.5 flex items-center space-x-1">
                      <MapPin class="h-3 w-3 shrink-0 sm:h-3.5 sm:w-3.5" />
                      <span class="truncate">{request.userLocation}</span>
                    </div>
                  {/if}
                </div>
              </div>
            </div>
            <div
              class="inline-flex shrink-0 items-center space-x-1 rounded-md border px-1.5 py-0.5 text-xs font-medium sm:px-2 sm:py-1 {getStatusColor(
                request.status,
              )}"
            >
              {#if request.status === "pending"}
                <Clock class="h-4 w-4 sm:h-5 sm:w-5" />
              {:else if request.status === "confirmed"}
                <AlertCircle class="h-4 w-4 sm:h-5 sm:w-5" />
              {:else if request.status === "completed"}
                <CheckCircle class="h-4 w-4 sm:h-5 sm:w-5" />
              {:else}
                <XCircle class="h-4 w-4 sm:h-5 sm:w-5" />
              {/if}
              <span class="hidden capitalize sm:inline">{request.status}</span>
            </div>
          </div>

          <!-- Amount Section -->
          <div class="rounded-lg bg-gray-50 p-2.5 sm:p-3">
            <div class="mb-0.5 text-xs text-gray-500 sm:mb-1">
              Deposit Amount
            </div>
            <div class="font-mono text-xl font-bold text-gray-900 sm:text-2xl">
              {formatAmount(request.amount, request.currency)}
            </div>
          </div>

          <div
            class="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between sm:gap-3"
          >
            <div class="text-xs text-gray-600 sm:text-sm">
              {#if !$demoMode}
                <span>Code: </span>
                <span class="font-mono font-semibold text-gray-900"
                  >{request.code}</span
                >
                <span class="ml-2 sm:ml-4"></span>
              {/if}
              <span>{new Date(request.createdAt).toLocaleString()}</span>
            </div>

            <div class="flex flex-col gap-2 sm:flex-row">
              {#if request.status === "pending"}
                <div class="flex items-center space-x-2">
                  <input
                    type="text"
                    placeholder="Enter code"
                    bind:value={verificationCodes[request.id]}
                    class="flex-1 rounded border border-neutral-300 px-2 py-1 font-mono text-xs uppercase sm:w-32 sm:px-3 sm:text-sm"
                  />
                  <button
                    onclick={() => handleVerifyCode(request)}
                    disabled={!verificationCodes[request.id]}
                    class="rounded bg-blue-600 px-2 py-1 text-xs whitespace-nowrap text-white hover:bg-blue-700 disabled:bg-neutral-300 sm:px-3 sm:text-sm"
                  >
                    Verify
                  </button>
                </div>
              {/if}

              {#if request.status === "confirmed" && selectedRequest?.id === request.id}
                <div class="flex flex-col gap-2 sm:flex-row">
                  <button
                    onclick={() => handleConfirmDeposit(request)}
                    disabled={isProcessing}
                    class="flex w-full items-center justify-center space-x-1 rounded bg-green-600 px-3 py-1.5 text-xs text-white hover:bg-green-700 disabled:bg-neutral-300 sm:w-auto sm:px-4 sm:py-2 sm:text-sm"
                  >
                    {#if isProcessing}
                      <div
                        class="h-3.5 w-3.5 animate-spin rounded-full border-b-2 border-white sm:h-4 sm:w-4"
                      ></div>
                      <span>Processing...</span>
                    {:else}
                      <CheckCircle class="h-3.5 w-3.5 shrink-0 sm:h-4 sm:w-4" />
                      <span>Complete Deposit</span>
                    {/if}
                  </button>
                  <button
                    onclick={() => handleRejectDeposit(request)}
                    disabled={isProcessing}
                    class="flex w-full items-center justify-center space-x-1 rounded bg-red-600 px-3 py-1.5 text-xs text-white hover:bg-red-700 disabled:bg-neutral-300 sm:w-auto sm:px-4 sm:py-2 sm:text-sm"
                  >
                    <XCircle class="h-3.5 w-3.5 shrink-0 sm:h-4 sm:w-4" />
                    <span>Reject</span>
                  </button>
                </div>
              {/if}
            </div>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>
