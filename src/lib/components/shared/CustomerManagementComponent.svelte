<script lang="ts">
  import { demoMode } from "$lib/stores/demoMode";
  import { principalId } from "$lib/stores/auth";
  import { toast } from "$lib/stores/toast";
  import { fetchAgentCustomers } from "$lib/services/data/customersData";
  import TransactionHistory from "$lib/components/shared/TransactionHistory.svelte";
  import {
    Ban,
    Calendar,
    CheckCircle,
    History,
    Loader2,
    MapPin,
    Phone,
    PhoneCall,
    Search,
    Shield,
    TrendingUp,
    User,
    X,
  } from "@lucide/svelte";

  interface Customer {
    id: string;
    name: string;
    phone: string;
    location: string;
    joinDate: string;
    totalTransactions: number;
    totalVolume: {
      ugx: number;
      usdc: number;
    };
    lastTransaction: string;
    status: "active" | "inactive" | "blocked";
    kycStatus: "verified" | "pending" | "rejected";
  }

  let customers = $state<Customer[]>([]);
  let searchTerm = $state("");
  let statusFilter = $state<string>("all");
  let isLoading = $state(true);
  let error = $state<string | null>(null);
  let selectedCustomer = $state<Customer | null>(null);
  let showModal = $state(false);
  let showBlockConfirm = $state(false);
  let showHistoryModal = $state(false);

  function openCustomerModal(customer: Customer) {
    selectedCustomer = customer;
    showModal = true;
  }

  function closeModal() {
    showModal = false;
    selectedCustomer = null;
  }

  function handleCallCustomer() {
    if (selectedCustomer) {
      // In real app, this would initiate a call
      window.location.href = `tel:${selectedCustomer.phone}`;
      toast.show("info", `Calling ${selectedCustomer.name}...`);
    }
  }

  function handleViewHistory() {
    if (selectedCustomer) {
      showHistoryModal = true;
    }
  }

  function handleBlockCustomer() {
    if (selectedCustomer) {
      showBlockConfirm = true;
    }
  }

  function confirmBlock() {
    if (selectedCustomer) {
      const customerName = selectedCustomer.name;
      const customerId = selectedCustomer.id;
      // Update customer status
      customers = customers.map((c) =>
        c.id === customerId ? { ...c, status: "blocked" as const } : c,
      );
      toast.show("success", `${customerName} has been blocked`);
      showBlockConfirm = false;
      closeModal();
    }
  }

  // Auto-fetch when stores change
  $effect(() => {
    loadCustomers($demoMode, $principalId);
  });

  async function loadCustomers(
    isDemoMode: boolean,
    agentPrincipal: string | null,
  ) {
    try {
      isLoading = true;
      error = null;

      // Use real data service (handles both demo and real mode)
      customers = await fetchAgentCustomers(agentPrincipal, isDemoMode);
    } catch (err: any) {
      error = err.message || "Failed to load customers";
      customers = [];
    } finally {
      isLoading = false;
    }
  }

  const filteredCustomers = $derived(
    customers.filter((customer) => {
      const matchesSearch =
        customer.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        customer.phone.includes(searchTerm) ||
        customer.location.toLowerCase().includes(searchTerm.toLowerCase());

      const matchesStatus =
        statusFilter === "all" || customer.status === statusFilter;

      return matchesSearch && matchesStatus;
    }),
  );

  function getTimeAgo(dateString: string): string {
    const date = new Date(dateString);
    const now = new Date();
    const diffInMinutes = Math.floor(
      (now.getTime() - date.getTime()) / (1000 * 60),
    );

    if (diffInMinutes < 60) {
      return `${diffInMinutes}m ago`;
    } else if (diffInMinutes < 1440) {
      return `${Math.floor(diffInMinutes / 60)}h ago`;
    } else {
      return `${Math.floor(diffInMinutes / 1440)}d ago`;
    }
  }

  function getStatusColor(status: string): string {
    switch (status) {
      case "active":
        return "bg-green-100 text-green-800 border-green-200";
      case "inactive":
        return "bg-yellow-100 text-yellow-800 border-yellow-200";
      case "blocked":
        return "bg-red-100 text-red-800 border-red-200";
      default:
        return "bg-gray-100 text-gray-800 border-gray-200";
    }
  }

  function getKycStatusColor(status: string): string {
    switch (status) {
      case "verified":
        return "bg-green-100 text-green-800 border-green-200";
      case "pending":
        return "bg-yellow-100 text-yellow-800 border-yellow-200";
      case "rejected":
        return "bg-red-100 text-red-800 border-red-200";
      default:
        return "bg-gray-100 text-gray-800 border-gray-200";
    }
  }
</script>

<!-- Customer Stats -->
{#if !isLoading && !error}
  <div class="mb-6 grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
    <div class="rounded-2xl border border-gray-200 bg-white p-4">
      <div class="flex items-center gap-3">
        <div
          class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-blue-100"
        >
          <User class="h-5 w-5 text-blue-600" />
        </div>
        <div>
          <p class="text-sm text-gray-600">Total Customers</p>
          <p class="text-2xl font-bold text-gray-900">{customers.length}</p>
        </div>
      </div>
    </div>

    <div class="rounded-2xl border border-gray-200 bg-white p-4">
      <div class="flex items-center gap-3">
        <div
          class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-green-100"
        >
          <CheckCircle class="h-5 w-5 text-green-600" />
        </div>
        <div>
          <p class="text-sm text-gray-600">Active Customers</p>
          <p class="text-2xl font-bold text-gray-900">
            {customers.filter((c) => c.status === "active").length}
          </p>
        </div>
      </div>
    </div>

    <div class="rounded-2xl border border-gray-200 bg-white p-4">
      <div class="flex items-center gap-3">
        <div
          class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-purple-100"
        >
          <Shield class="h-5 w-5 text-purple-600" />
        </div>
        <div>
          <p class="text-sm text-gray-600">KYC Verified</p>
          <p class="text-2xl font-bold text-gray-900">
            {customers.filter((c) => c.kycStatus === "verified").length}
          </p>
        </div>
      </div>
    </div>

    <div class="rounded-2xl border border-gray-200 bg-white p-4">
      <div class="flex items-center gap-3">
        <div
          class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-orange-100"
        >
          <TrendingUp class="h-5 w-5 text-orange-600" />
        </div>
        <div>
          <p class="text-sm text-gray-600">Avg. Transactions</p>
          <p class="text-2xl font-bold text-gray-900">
            {customers.length > 0
              ? (
                  customers.reduce((sum, c) => sum + c.totalTransactions, 0) /
                  customers.length
                ).toFixed(1)
              : "0"}
          </p>
        </div>
      </div>
    </div>
  </div>
{/if}

<!-- Search and Filters -->
<div
  class="mb-6 rounded-2xl border border-gray-200 bg-white p-4 shadow-sm sm:p-5 md:p-6"
>
  <div class="flex flex-col gap-3 sm:flex-row sm:gap-4">
    <!-- Search -->
    <div class="relative flex-1">
      <Search
        class="absolute top-1/2 left-3 h-4 w-4 shrink-0 -translate-y-1/2 transform text-gray-400 sm:h-5 sm:w-5"
      />
      <input
        type="text"
        placeholder="Search by name, phone, or location..."
        bind:value={searchTerm}
        class="w-full rounded-lg border border-gray-200 py-2 pr-3 pl-9 text-sm focus:border-transparent focus:ring-2 focus:ring-gray-900 sm:py-2.5 sm:pr-4 sm:pl-10 sm:text-base"
      />
    </div>
  </div>

  <!-- Filter Buttons -->
  <div
    class="scrollbar-hide mt-3 flex gap-1.5 overflow-x-auto pb-2 sm:mt-4 sm:gap-2"
  >
    <button
      onclick={() => (statusFilter = "all")}
      class="shrink-0 rounded-lg px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-colors sm:px-4 sm:py-2 sm:text-sm {statusFilter ===
      'all'
        ? 'bg-black text-white'
        : 'border border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
    >
      All
    </button>
    <button
      onclick={() => (statusFilter = "active")}
      class="shrink-0 rounded-lg px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-colors sm:px-4 sm:py-2 sm:text-sm {statusFilter ===
      'active'
        ? 'bg-black text-white'
        : 'border border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
    >
      Active
    </button>
    <button
      onclick={() => (statusFilter = "inactive")}
      class="shrink-0 rounded-lg px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-colors sm:px-4 sm:py-2 sm:text-sm {statusFilter ===
      'inactive'
        ? 'bg-black text-white'
        : 'border border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
    >
      Inactive
    </button>
    <button
      onclick={() => (statusFilter = "blocked")}
      class="shrink-0 rounded-lg px-3 py-1.5 text-xs font-medium whitespace-nowrap transition-colors sm:px-4 sm:py-2 sm:text-sm {statusFilter ===
      'blocked'
        ? 'bg-black text-white'
        : 'border border-gray-200 bg-white text-gray-700 hover:bg-gray-50'}"
    >
      Blocked
    </button>
  </div>
</div>

<!-- Customer List -->
<div class="rounded-2xl border border-gray-200 bg-white shadow-sm">
  <div class="border-b border-gray-200 px-4 py-3 sm:px-5 sm:py-4 md:px-6">
    <h2 class="text-sm font-semibold text-gray-900 sm:text-base md:text-lg">
      Customer List ({filteredCustomers.length})
    </h2>
  </div>

  <div class="p-4 sm:p-5 md:p-6">
    {#if isLoading}
      <div class="py-8 text-center sm:py-10 md:py-12">
        <Loader2
          class="mx-auto mb-3 h-8 w-8 animate-spin text-gray-400 sm:mb-4 sm:h-10 sm:w-10 md:h-12 md:w-12"
        />
        <h3
          class="mb-1.5 text-base font-semibold text-gray-900 sm:mb-2 sm:text-lg"
        >
          Loading customers...
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          Please wait while we fetch your customer data.
        </p>
      </div>
    {:else if error}
      <div class="py-8 text-center sm:py-10 md:py-12">
        <User
          class="mx-auto mb-3 h-8 w-8 text-red-400 sm:mb-4 sm:h-10 sm:w-10 md:h-12 md:w-12"
        />
        <h3
          class="mb-1.5 text-base font-semibold text-red-900 sm:mb-2 sm:text-lg"
        >
          Error Loading Customers
        </h3>
        <p class="mb-3 px-4 text-sm text-red-600 sm:mb-4 sm:text-base">
          {error}
        </p>
        <button
          onclick={() => window.location.reload()}
          class="rounded-lg bg-red-600 px-4 py-2 text-xs text-white transition-colors hover:bg-red-700 sm:text-sm"
        >
          Retry
        </button>
      </div>
    {:else if filteredCustomers.length === 0}
      <div class="py-8 text-center sm:py-10 md:py-12">
        <User
          class="mx-auto mb-3 h-8 w-8 text-gray-400 sm:mb-4 sm:h-10 sm:w-10 md:h-12 md:w-12"
        />
        <h3
          class="mb-1.5 text-base font-semibold text-gray-900 sm:mb-2 sm:text-lg"
        >
          No customers found
        </h3>
        <p class="text-sm text-gray-600 sm:text-base">
          Try adjusting your search or filter criteria.
        </p>
      </div>
    {:else}
      <div class="space-y-3 sm:space-y-4">
        {#each filteredCustomers as customer (customer.id)}
          <div
            class="cursor-pointer rounded-2xl border border-gray-200 bg-white p-3 transition-all duration-200 hover:border-gray-300 hover:shadow-md sm:p-4 md:p-5 lg:p-6"
            onclick={() => openCustomerModal(customer)}
            onkeydown={(e) => e.key === "Enter" && openCustomerModal(customer)}
            role="button"
            tabindex="0"
          >
            <div
              class="flex flex-col gap-3 space-y-3 sm:flex-row sm:items-center sm:justify-between sm:space-y-0"
            >
              <div
                class="flex min-w-0 flex-1 items-center space-x-3 sm:space-x-4"
              >
                <!-- Customer Avatar -->
                <div
                  class="hidden h-10 w-10 shrink-0 items-center justify-center rounded-full bg-black sm:flex md:h-12 md:w-12"
                >
                  <span class="text-sm font-bold text-white">
                    {customer.name
                      .split(" ")
                      .map((n) => n[0])
                      .join("")
                      .toUpperCase()
                      .slice(0, 2)}
                  </span>
                </div>

                <!-- Customer Info -->
                <div class="min-w-0 flex-1">
                  <h3
                    class="truncate text-sm font-semibold text-gray-900 sm:text-base md:text-lg"
                  >
                    {customer.name}
                  </h3>
                  <div
                    class="mt-0.5 flex flex-col space-y-1 text-xs text-gray-600 sm:mt-1 sm:flex-row sm:items-center sm:space-y-0 sm:space-x-3 sm:text-sm md:space-x-4"
                  >
                    <div class="flex items-center space-x-1">
                      <Phone
                        class="h-3 w-3 shrink-0 sm:h-3.5 sm:w-3.5 md:h-4 md:w-4"
                      />
                      <span class="truncate">{customer.phone}</span>
                    </div>
                    <div class="flex items-center space-x-1">
                      <MapPin
                        class="h-3 w-3 shrink-0 sm:h-3.5 sm:w-3.5 md:h-4 md:w-4"
                      />
                      <span class="truncate">{customer.location}</span>
                    </div>
                    <div class="flex items-center space-x-1">
                      <Calendar
                        class="h-3 w-3 shrink-0 sm:h-3.5 sm:w-3.5 md:h-4 md:w-4"
                      />
                      <span class="whitespace-nowrap"
                        >Joined {new Date(
                          customer.joinDate,
                        ).toLocaleDateString()}</span
                      >
                    </div>
                  </div>
                </div>
              </div>

              <!-- Customer Stats and Status -->
              <div class="shrink-0 text-left sm:text-right">
                <!-- Transaction Count -->
                <div
                  class="mb-1.5 text-[10px] text-gray-500 sm:mb-2 sm:text-xs"
                >
                  {customer.totalTransactions} transactions • Last: {getTimeAgo(
                    customer.lastTransaction,
                  )}
                </div>

                <!-- Status Badges -->
                <div class="flex flex-wrap gap-1.5 sm:justify-end sm:gap-2">
                  <span
                    class="inline-flex items-center rounded-full border px-2 py-0.5 text-[10px] font-semibold whitespace-nowrap sm:px-2.5 sm:text-xs {getStatusColor(
                      customer.status,
                    )}"
                  >
                    {customer.status.charAt(0).toUpperCase() +
                      customer.status.slice(1)}
                  </span>
                  <span
                    class="inline-flex items-center rounded-full border px-2 py-0.5 text-[10px] font-semibold whitespace-nowrap sm:px-2.5 sm:text-xs {getKycStatusColor(
                      customer.kycStatus,
                    )}"
                  >
                    KYC: {customer.kycStatus.charAt(0).toUpperCase() +
                      customer.kycStatus.slice(1)}
                  </span>
                </div>
              </div>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<!-- Customer Details Modal -->
{#if showModal && selectedCustomer}
  <div
    class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center bg-black p-4"
    onclick={closeModal}
    onkeydown={(e) => e.key === "Escape" && closeModal()}
    role="button"
    tabindex="-1"
  >
    <div
      class="max-h-[90vh] w-full max-w-2xl overflow-y-auto rounded-2xl bg-white"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      tabindex="-1"
    >
      <!-- Modal Header -->
      <div
        class="sticky top-0 flex items-center justify-between rounded-t-2xl border-b border-gray-200 bg-white px-6 py-4"
      >
        <h2 class="text-xl font-bold text-gray-900">Customer Details</h2>
        <button
          aria-label="Toggle"
          onclick={closeModal}
          class="text-gray-400 transition-colors hover:text-gray-600"
        >
          <X class="h-5 w-5" />
        </button>
      </div>

      <!-- Modal Content -->
      <div class="space-y-6 p-6">
        <!-- Customer Info -->
        <div class="flex items-center gap-4">
          <div
            class="flex h-16 w-16 shrink-0 items-center justify-center rounded-full bg-black"
          >
            <span class="text-xl font-bold text-white">
              {selectedCustomer.name
                .split(" ")
                .map((n) => n[0])
                .join("")
                .toUpperCase()
                .slice(0, 2)}
            </span>
          </div>
          <div>
            <h3 class="text-2xl font-bold text-gray-900">
              {selectedCustomer.name}
            </h3>
            <div class="mt-1 flex items-center gap-2">
              <span
                class="inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold {getStatusColor(
                  selectedCustomer.status,
                )}"
              >
                {selectedCustomer.status.charAt(0).toUpperCase() +
                  selectedCustomer.status.slice(1)}
              </span>
              <span
                class="inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold {getKycStatusColor(
                  selectedCustomer.kycStatus,
                )}"
              >
                KYC: {selectedCustomer.kycStatus.charAt(0).toUpperCase() +
                  selectedCustomer.kycStatus.slice(1)}
              </span>
            </div>
          </div>
        </div>

        <!-- Contact Info -->
        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
          <div class="rounded-lg bg-gray-50 p-4">
            <div class="mb-1 flex items-center gap-2 text-gray-600">
              <Phone class="h-4 w-4" />
              <span class="text-sm font-medium">Phone</span>
            </div>
            <p class="font-semibold text-gray-900">{selectedCustomer.phone}</p>
          </div>
          <div class="rounded-lg bg-gray-50 p-4">
            <div class="mb-1 flex items-center gap-2 text-gray-600">
              <MapPin class="h-4 w-4" />
              <span class="text-sm font-medium">Location</span>
            </div>
            <p class="font-semibold text-gray-900">
              {selectedCustomer.location}
            </p>
          </div>
        </div>

        <!-- Stats -->
        <div class="grid grid-cols-2 gap-4 sm:grid-cols-4">
          <div
            class="rounded-lg border border-gray-200 bg-gray-50 p-4 text-center"
          >
            <p class="text-2xl font-bold text-gray-900">
              {selectedCustomer.totalTransactions}
            </p>
            <p class="mt-1 text-xs text-gray-600">Transactions</p>
          </div>
          <div
            class="rounded-lg border border-gray-200 bg-gray-50 p-4 text-center"
          >
            <p class="text-2xl font-bold text-gray-900">
              {(selectedCustomer.totalVolume.ugx / 1000000).toFixed(1)}M
            </p>
            <p class="mt-1 text-xs text-gray-600">UGX Volume</p>
          </div>
          <div
            class="rounded-lg border border-gray-200 bg-gray-50 p-4 text-center"
          >
            <p class="text-2xl font-bold text-gray-900">
              ${selectedCustomer.totalVolume.usdc.toFixed(0)}
            </p>
            <p class="mt-1 text-xs text-gray-600">USDC Volume</p>
          </div>
          <div
            class="rounded-lg border border-gray-200 bg-gray-50 p-4 text-center"
          >
            <p class="text-sm font-bold text-gray-900">
              {getTimeAgo(selectedCustomer.lastTransaction)}
            </p>
            <p class="mt-1 text-xs text-gray-600">Last Active</p>
          </div>
        </div>

        <!-- Join Date -->
        <div class="rounded-lg bg-gray-50 p-4">
          <div class="mb-1 flex items-center gap-2 text-gray-600">
            <Calendar class="h-4 w-4" />
            <span class="text-sm font-medium">Member Since</span>
          </div>
          <p class="font-semibold text-gray-900">
            {new Date(selectedCustomer.joinDate).toLocaleDateString("en-US", {
              year: "numeric",
              month: "long",
              day: "numeric",
            })}
          </p>
        </div>

        <!-- Actions -->
        <div class="grid grid-cols-1 gap-3 sm:grid-cols-3">
          <button
            onclick={handleCallCustomer}
            class="flex items-center justify-center gap-2 rounded-lg bg-black px-4 py-3 text-white transition-colors hover:bg-gray-800"
          >
            <PhoneCall class="h-4 w-4" />
            <span class="font-medium">Call Customer</span>
          </button>
          <button
            onclick={handleViewHistory}
            class="flex items-center justify-center gap-2 rounded-lg border border-gray-300 px-4 py-3 text-gray-700 transition-colors hover:bg-gray-50"
          >
            <History class="h-4 w-4" />
            <span class="font-medium">View History</span>
          </button>
          <button
            onclick={handleBlockCustomer}
            class="flex items-center justify-center gap-2 rounded-lg border border-red-300 px-4 py-3 text-red-600 transition-colors hover:bg-red-50"
          >
            <Ban class="h-4 w-4" />
            <span class="font-medium">Block Customer</span>
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<!-- Block Confirmation Modal -->
{#if showBlockConfirm && selectedCustomer}
  <div
    class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center bg-black p-4"
    onclick={() => (showBlockConfirm = false)}
    onkeydown={(e) => e.key === "Escape" && (showBlockConfirm = false)}
    role="button"
    tabindex="-1"
  >
    <div
      class="w-full max-w-md rounded-2xl bg-white p-6"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      tabindex="-1"
    >
      <h3 class="mb-4 text-xl font-bold text-gray-900">Block Customer?</h3>
      <p class="mb-6 text-gray-600">
        Are you sure you want to block <strong>{selectedCustomer.name}</strong>?
        They will not be able to make transactions until unblocked.
      </p>
      <div class="flex gap-3">
        <button
          onclick={() => (showBlockConfirm = false)}
          class="flex-1 rounded-lg border border-gray-300 px-4 py-2 text-gray-700 transition-colors hover:bg-gray-50"
        >
          Cancel
        </button>
        <button
          onclick={confirmBlock}
          class="flex-1 rounded-lg bg-red-600 px-4 py-2 text-white transition-colors hover:bg-red-700"
        >
          Block Customer
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Transaction History Modal -->
{#if showHistoryModal && selectedCustomer}
  <div
    class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center bg-black p-4"
    onclick={() => (showHistoryModal = false)}
    onkeydown={(e) => e.key === "Escape" && (showHistoryModal = false)}
    role="button"
    tabindex="-1"
  >
    <div
      class="max-h-[90vh] w-full max-w-3xl overflow-y-auto rounded-2xl bg-white"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="dialog"
      aria-modal="true"
      tabindex="-1"
    >
      <!-- Header -->
      <div
        class="sticky top-0 flex items-center justify-between rounded-t-2xl border-b border-gray-200 bg-white px-6 py-4"
      >
        <div>
          <h2 class="text-xl font-bold text-gray-900">Transaction History</h2>
          <p class="mt-1 text-sm text-gray-600">{selectedCustomer.name}</p>
        </div>
        <button
          onclick={() => (showHistoryModal = false)}
          class="text-gray-400 transition-colors hover:text-gray-600"
        >
          <X class="h-6 w-6" />
        </button>
      </div>

      <!-- Transaction History Component -->
      <div class="p-6">
        <TransactionHistory
          maxTransactions={50}
          showViewAll={false}
          showFilters={true}
        />
      </div>
    </div>
  </div>
{/if}
