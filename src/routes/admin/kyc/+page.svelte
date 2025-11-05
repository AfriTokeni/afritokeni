<script lang="ts">
  import {
    CheckCircle,
    XCircle,
    Eye,
    FileText,
    User,
    MapPin,
    Info,
    ChevronDown,
    Search,
    ChevronLeft,
    ChevronRight,
    X,
    RefreshCw,
  } from "@lucide/svelte";
  import { onMount } from "svelte";
  import type { ApexOptions } from "apexcharts";
  import { Chart } from "@flowbite-svelte-plugins/chart";
  import StatCard from "$lib/components/admin/StatCard.svelte";
  import { Button, Dropdown, DropdownItem } from "flowbite-svelte";
  import { updateKYCStatus } from "$lib/services/juno/kycService";
  import type { KYCDocument } from "$lib/types/admin";
  import type { PageData } from "./$types";

  let { data }: { data: PageData } = $props();

  // Helper to generate avatar URL
  function getAvatarUrl(name: string): string {
    const colors = ["3b82f6", "8b5cf6", "10b981", "f59e0b", "ef4444"];
    const colorIndex = name.length % colors.length;
    return `https://ui-avatars.com/api/?name=${encodeURIComponent(name)}&background=${colors[colorIndex]}&color=fff`;
  }

  // Last updated timestamp
  let lastUpdated = $state(new Date().toLocaleTimeString());

  // Refresh data
  async function refreshData() {
    lastUpdated = new Date().toLocaleTimeString();
    // TODO: Reload data from Juno
    console.log("Refreshing KYC data...");
  }

  let activeTab = $state<"pending" | "approved" | "rejected">("pending");
  let showReviewModal = $state(false);
  let selectedKYC = $state<any>(null);
  let rejectionReason = $state("");
  let chartDateRange = $state<"7" | "30" | "90">("30");
  let searchQuery = $state("");
  let selectedDocumentIndex = $state<number | null>(null);

  // Pagination state
  let itemsPerPage = 20;
  let displayedPendingCount = $state(itemsPerPage);
  let displayedApprovedCount = $state(itemsPerPage);
  let displayedRejectedCount = $state(itemsPerPage);

  function loadMorePending() {
    displayedPendingCount += itemsPerPage;
  }

  function loadMoreApproved() {
    displayedApprovedCount += itemsPerPage;
  }

  function loadMoreRejected() {
    displayedRejectedCount += itemsPerPage;
  }

  function viewDocument(index: number) {
    selectedDocumentIndex = index;
  }

  function closeDocumentView() {
    selectedDocumentIndex = null;
  }

  function nextDocument() {
    if (selectedDocumentIndex !== null && selectedKYC) {
      selectedDocumentIndex =
        (selectedDocumentIndex + 1) % selectedKYC.documents.length;
    }
  }

  function previousDocument() {
    if (selectedDocumentIndex !== null && selectedKYC) {
      selectedDocumentIndex =
        selectedDocumentIndex === 0
          ? selectedKYC.documents.length - 1
          : selectedDocumentIndex - 1;
    }
  }

  // Generate real chart data from actual KYC documents
  function getChartData() {
    // Group documents by date based on submittedAt
    const days = chartDateRange === "7" ? 7 : chartDateRange === "30" ? 30 : 90;
    const dateMap = new Map<
      string,
      { submissions: number; approved: number; rejected: number }
    >();

    // Initialize all dates with 0
    const today = new Date();
    for (let i = days - 1; i >= 0; i--) {
      const date = new Date(today);
      date.setDate(date.getDate() - i);
      const dateStr = date.toLocaleDateString("en-US", {
        month: "short",
        day: "numeric",
      });
      dateMap.set(dateStr, { submissions: 0, approved: 0, rejected: 0 });
    }

    // Count documents by date
    allDocuments.forEach((doc) => {
      const date = new Date(doc.submittedAt);
      const dateStr = date.toLocaleDateString("en-US", {
        month: "short",
        day: "numeric",
      });
      const stats = dateMap.get(dateStr);
      if (stats) {
        stats.submissions++;
        if (doc.status === "approved") stats.approved++;
        if (doc.status === "rejected") stats.rejected++;
      }
    });

    const categories = Array.from(dateMap.keys());
    const submissions = Array.from(dateMap.values()).map((v) => v.submissions);
    const approved = Array.from(dateMap.values()).map((v) => v.approved);
    const rejected = Array.from(dateMap.values()).map((v) => v.rejected);

    return { categories, submissions, approved, rejected };
  }

  let chartOptions = $derived<ApexOptions>({
    chart: {
      height: "320px",
      type: "area",
      fontFamily: "Inter, sans-serif",
      dropShadow: { enabled: false },
      toolbar: { show: false },
    },
    tooltip: { enabled: true, x: { show: false } },
    fill: {
      type: "gradient",
      gradient: {
        opacityFrom: 0.55,
        opacityTo: 0,
        shade: "#1C64F2",
        gradientToColors: ["#1C64F2"],
      },
    },
    dataLabels: { enabled: false },
    stroke: { width: 2, curve: "smooth" },
    grid: {
      show: true,
      strokeDashArray: 4,
      padding: { left: 2, right: 2, top: 0 },
    },
    series: [
      {
        name: "Submissions",
        data: getChartData().submissions,
        color: "#3b82f6",
      },
      {
        name: "Approved",
        data: getChartData().approved,
        color: "#22c55e",
      },
      {
        name: "Rejected",
        data: getChartData().rejected,
        color: "#ef4444",
      },
    ],
    xaxis: {
      categories: getChartData().categories,
      labels: {
        show: true,
        style: {
          fontFamily: "Inter, sans-serif",
          cssClass: "text-xs font-normal fill-gray-500",
        },
      },
      axisBorder: { show: false },
      axisTicks: { show: false },
    },
    yaxis: { show: true },
    legend: { show: true, position: "top" },
  });

  // Real KYC data from Juno
  let allDocuments = $state<KYCDocument[]>(data.documents);
  let stats = $state(data.stats);

  // Filter documents by status
  let pendingKYC = $derived(
    allDocuments.filter((doc) => doc.status === "pending"),
  );
  let approvedKYC = $derived(
    allDocuments.filter((doc) => doc.status === "approved"),
  );
  let rejectedKYC = $derived(
    allDocuments.filter((doc) => doc.status === "rejected"),
  );

  function reviewKYC(kyc: KYCDocument) {
    selectedKYC = kyc;
    showReviewModal = true;
  }

  async function approveKYC() {
    if (!selectedKYC) return;

    try {
      await updateKYCStatus(selectedKYC.id, "approved");

      // Update local state
      const index = allDocuments.findIndex((d) => d.id === selectedKYC.id);
      if (index !== -1) {
        allDocuments[index] = {
          ...allDocuments[index],
          status: "approved",
          reviewedAt: new Date().toISOString(),
        };
      }

      showReviewModal = false;
      selectedKYC = null;
    } catch (error) {
      console.error("Failed to approve KYC:", error);
    }
  }

  async function rejectKYC() {
    if (!selectedKYC || !rejectionReason) return;

    try {
      await updateKYCStatus(selectedKYC.id, "rejected", rejectionReason);

      // Update local state
      const index = allDocuments.findIndex((d) => d.id === selectedKYC.id);
      if (index !== -1) {
        allDocuments[index] = {
          ...allDocuments[index],
          status: "rejected",
          reviewedAt: new Date().toISOString(),
          adminNotes: rejectionReason,
        };
      }

      showReviewModal = false;
      selectedKYC = null;
      rejectionReason = "";
    } catch (error) {
      console.error("Failed to reject KYC:", error);
    }
  }

  function closeModal() {
    showReviewModal = false;
    selectedKYC = null;
    rejectionReason = "";
  }

  // Filter KYC lists based on search query
  let filteredPendingKYC = $derived(
    pendingKYC.filter((kyc) =>
      searchQuery
        ? kyc.userName.toLowerCase().includes(searchQuery.toLowerCase()) ||
          kyc.userEmail.toLowerCase().includes(searchQuery.toLowerCase()) ||
          kyc.documentNumber.toLowerCase().includes(searchQuery.toLowerCase())
        : true,
    ),
  );

  let filteredApprovedKYC = $derived(
    approvedKYC.filter((kyc) =>
      searchQuery
        ? kyc.userName.toLowerCase().includes(searchQuery.toLowerCase()) ||
          kyc.userEmail.toLowerCase().includes(searchQuery.toLowerCase()) ||
          kyc.documentNumber.toLowerCase().includes(searchQuery.toLowerCase())
        : true,
    ),
  );

  let filteredRejectedKYC = $derived(
    rejectedKYC.filter((kyc) =>
      searchQuery
        ? kyc.userName.toLowerCase().includes(searchQuery.toLowerCase()) ||
          kyc.userEmail.toLowerCase().includes(searchQuery.toLowerCase()) ||
          kyc.documentNumber.toLowerCase().includes(searchQuery.toLowerCase())
        : true,
    ),
  );

  // Paginated lists
  let displayedPendingKYC = $derived(
    filteredPendingKYC.slice(0, displayedPendingCount),
  );
  let displayedApprovedKYC = $derived(
    filteredApprovedKYC.slice(0, displayedApprovedCount),
  );
  let displayedRejectedKYC = $derived(
    filteredRejectedKYC.slice(0, displayedRejectedCount),
  );

  // Check if there are more items to load
  let hasMorePending = $derived(
    displayedPendingCount < filteredPendingKYC.length,
  );
  let hasMoreApproved = $derived(
    displayedApprovedCount < filteredApprovedKYC.length,
  );
  let hasMoreRejected = $derived(
    displayedRejectedCount < filteredRejectedKYC.length,
  );

  function viewKYC(kyc: any) {
    selectedKYC = kyc;
    showReviewModal = true;
  }
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- Stats Grid -->
  <div class="grid grid-cols-1 gap-4 sm:gap-6 md:grid-cols-3">
    <!-- Pending Count -->
    <StatCard
      label="Pending Review"
      value={stats.pending}
      {lastUpdated}
      onRefresh={refreshData}
      valueColor="text-yellow-600"
    />

    <!-- Approved Count -->
    <StatCard
      label="Approved"
      value={stats.approved}
      {lastUpdated}
      onRefresh={refreshData}
      valueColor="text-green-600"
    />

    <!-- Rejected Count -->
    <StatCard
      label="Rejected"
      value={stats.rejected}
      {lastUpdated}
      onRefresh={refreshData}
      valueColor="text-red-600"
    />
  </div>

  <!-- KYC Submissions Trend Chart -->
  <div
    class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
  >
    <div class="mb-4 flex items-center justify-between sm:mb-6">
      <div>
        <h3 class="text-base font-semibold text-gray-900 sm:text-lg">
          KYC Submissions Trend
        </h3>
        <p class="text-xs text-gray-500 sm:text-sm">Submissions over time</p>
      </div>
      <div class="relative">
        <Button size="sm" color="light" class="gap-2">
          {chartDateRange === "7"
            ? "Last 7 days"
            : chartDateRange === "30"
              ? "Last 30 days"
              : "Last 90 days"}
          <ChevronDown class="h-4 w-4" />
        </Button>
        <Dropdown class="z-50 w-44 !shadow-md">
          <DropdownItem onclick={() => (chartDateRange = "7")}
            >Last 7 days</DropdownItem
          >
          <DropdownItem onclick={() => (chartDateRange = "30")}
            >Last 30 days</DropdownItem
          >
          <DropdownItem onclick={() => (chartDateRange = "90")}
            >Last 90 days</DropdownItem
          >
        </Dropdown>
      </div>
    </div>
    <div class="h-64 sm:h-80">
      <Chart options={chartOptions} />
    </div>
  </div>

  <!-- Tabs -->
  <div class="rounded-xl border border-gray-200 bg-white sm:rounded-2xl">
    <div class="border-b border-gray-200 px-4 sm:px-6">
      <div class="flex space-x-8">
        <button
          onclick={() => (activeTab = "pending")}
          class="border-b-2 py-4 text-sm font-medium transition-colors {activeTab ===
          'pending'
            ? 'border-blue-600 text-blue-600'
            : 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700'}"
        >
          Pending ({pendingKYC.length})
        </button>
        <button
          onclick={() => (activeTab = "approved")}
          class="border-b-2 py-4 text-sm font-medium transition-colors {activeTab ===
          'approved'
            ? 'border-blue-600 text-blue-600'
            : 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700'}"
        >
          Approved ({approvedKYC.length})
        </button>
        <button
          onclick={() => (activeTab = "rejected")}
          class="border-b-2 py-4 text-sm font-medium transition-colors {activeTab ===
          'rejected'
            ? 'border-blue-600 text-blue-600'
            : 'border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700'}"
        >
          Rejected ({rejectedKYC.length})
        </button>
      </div>
    </div>

    <!-- Search Bar -->
    <div class="border-b border-gray-200 px-4 py-3 sm:px-6">
      <div class="relative">
        <Search
          class="absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-gray-400"
        />
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Search by name, email, or phone..."
          class="w-full rounded-lg border border-gray-200 py-2 pr-4 pl-10 text-sm focus:border-blue-600 focus:ring-2 focus:ring-blue-600 focus:outline-none"
        />
      </div>
    </div>

    <!-- Tab Content -->
    <div class="p-4 sm:p-6">
      {#if activeTab === "pending"}
        <div class="space-y-4">
          {#each displayedPendingKYC as kyc}
            <button
              onclick={() => viewKYC(kyc)}
              class="w-full rounded-lg border border-gray-200 p-4 text-left transition-all hover:border-blue-400 hover:shadow-md sm:p-6"
            >
              <div class="flex items-start justify-between">
                <div class="flex items-start space-x-4">
                  <img
                    src={getAvatarUrl(kyc.userName)}
                    alt={kyc.userName}
                    class="h-12 w-12 rounded-lg"
                  />
                  <div class="flex-1">
                    <div class="flex items-center space-x-2">
                      <h4 class="font-semibold text-gray-900">
                        {kyc.userName}
                      </h4>
                      {#if kyc.location}
                        <span
                          class="rounded-full bg-purple-100 px-2 py-1 text-xs font-medium text-purple-800"
                        >
                          Agent
                        </span>
                      {:else}
                        <span
                          class="rounded-full bg-blue-100 px-2 py-1 text-xs font-medium text-blue-800"
                        >
                          User
                        </span>
                      {/if}
                    </div>
                    <p class="mt-1 text-sm text-gray-500">
                      {kyc.userEmail}{#if kyc.userPhone}
                        • {kyc.userPhone}{/if}
                    </p>
                    <div class="mt-3 grid grid-cols-2 gap-4">
                      <div>
                        <p class="text-xs text-gray-500">Document Type</p>
                        <p class="mt-1 text-sm font-medium text-gray-900">
                          {kyc.documentType}
                        </p>
                      </div>
                      <div>
                        <p class="text-xs text-gray-500">Document Number</p>
                        <p class="mt-1 text-sm font-medium text-gray-900">
                          {kyc.documentNumber}
                        </p>
                      </div>
                      {#if kyc.location}
                        <div>
                          <p class="text-xs text-gray-500">Location</p>
                          <p class="mt-1 text-sm font-medium text-gray-900">
                            {kyc.location}
                          </p>
                        </div>
                      {/if}
                      {#if kyc.businessLicense}
                        <div>
                          <p class="text-xs text-gray-500">Business License</p>
                          <p class="mt-1 text-sm font-medium text-gray-900">
                            {kyc.businessLicense}
                          </p>
                        </div>
                      {/if}
                    </div>
                    <p class="mt-3 text-xs text-gray-400">
                      Submitted: {kyc.submittedAt}
                    </p>
                  </div>
                </div>
                <div class="flex items-center text-blue-600">
                  <Eye class="h-5 w-5" />
                </div>
              </div>
            </button>
          {/each}
        </div>

        <!-- Load More Button for Pending -->
        {#if hasMorePending}
          <div class="mt-6 flex justify-center">
            <button
              onclick={loadMorePending}
              class="rounded-lg bg-blue-600 px-6 py-3 font-semibold text-white transition-all hover:bg-blue-700"
            >
              Load More ({filteredPendingKYC.length - displayedPendingCount} remaining)
            </button>
          </div>
        {/if}
      {:else if activeTab === "approved"}
        <div class="space-y-4">
          {#each displayedApprovedKYC as kyc}
            <button
              onclick={() => viewKYC(kyc)}
              class="w-full rounded-lg border border-green-200 bg-green-50 p-4 text-left transition-all hover:border-green-400 hover:shadow-md sm:p-6"
            >
              <div class="flex items-start justify-between">
                <div class="flex items-start space-x-4">
                  <img
                    src={getAvatarUrl(kyc.userName)}
                    alt={kyc.userName}
                    class="h-12 w-12 rounded-lg"
                  />
                  <div class="flex-1">
                    <div class="flex items-center space-x-2">
                      <h4 class="font-semibold text-gray-900">
                        {kyc.userName}
                      </h4>
                      <span
                        class="rounded-full bg-green-100 px-2 py-1 text-xs font-medium text-green-800"
                      >
                        {kyc.location ? "Agent" : "User"}
                      </span>
                    </div>
                    <p class="mt-1 text-sm text-gray-600">{kyc.userEmail}</p>
                    <p class="mt-2 text-xs text-gray-500">
                      Approved by {kyc.approvedBy} on {kyc.approvedAt}
                    </p>
                  </div>
                </div>
                <div class="flex items-center text-green-600">
                  <Eye class="h-5 w-5" />
                </div>
              </div>
            </button>
          {/each}
        </div>

        <!-- Load More Button for Approved -->
        {#if hasMoreApproved}
          <div class="mt-6 flex justify-center">
            <button
              onclick={loadMoreApproved}
              class="rounded-lg bg-green-600 px-6 py-3 font-semibold text-white transition-all hover:bg-green-700"
            >
              Load More ({filteredApprovedKYC.length - displayedApprovedCount} remaining)
            </button>
          </div>
        {/if}
      {:else}
        <div class="space-y-4">
          {#each displayedRejectedKYC as kyc}
            <button
              onclick={() => viewKYC(kyc)}
              class="w-full rounded-lg border border-red-200 bg-red-50 p-4 text-left transition-all hover:border-red-400 hover:shadow-md sm:p-6"
            >
              <div class="flex items-start justify-between">
                <div class="flex items-start space-x-4">
                  <img
                    src={getAvatarUrl(kyc.userName)}
                    alt={kyc.userName}
                    class="h-12 w-12 rounded-lg"
                  />
                  <div class="flex-1">
                    <div class="flex items-center space-x-2">
                      <h4 class="font-semibold text-gray-900">
                        {kyc.userName}
                      </h4>
                      <span
                        class="rounded-full bg-red-100 px-2 py-1 text-xs font-medium text-red-800"
                      >
                        {kyc.location ? "Agent" : "User"}
                      </span>
                    </div>
                    <p class="mt-1 text-sm text-gray-600">{kyc.userEmail}</p>
                    <p class="mt-2 text-sm font-medium text-red-900">
                      Reason: {kyc.reason}
                    </p>
                    <p class="mt-1 text-xs text-gray-500">
                      Rejected by {kyc.rejectedBy} on {kyc.rejectedAt}
                    </p>
                  </div>
                </div>
                <div class="flex items-center text-red-600">
                  <Eye class="h-5 w-5" />
                </div>
              </div>
            </button>
          {/each}
        </div>

        <!-- Load More Button for Rejected -->
        {#if hasMoreRejected}
          <div class="mt-6 flex justify-center">
            <button
              onclick={loadMoreRejected}
              class="rounded-lg bg-red-600 px-6 py-3 font-semibold text-white transition-all hover:bg-red-700"
            >
              Load More ({filteredRejectedKYC.length - displayedRejectedCount} remaining)
            </button>
          </div>
        {/if}
      {/if}
    </div>
  </div>
</div>

<!-- Review Modal -->
{#if showReviewModal && selectedKYC}
  <div
    class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center bg-black p-4"
  >
    <div
      class="max-h-[95vh] w-full max-w-6xl overflow-y-auto rounded-2xl bg-white shadow-xl"
    >
      <!-- Header -->
      <div
        class="sticky top-0 z-10 border-b border-gray-100 bg-gradient-to-r from-blue-50 to-white px-8 py-6"
      >
        <div class="flex items-center justify-between">
          <div>
            <h3 class="text-2xl font-bold text-gray-900">KYC Review</h3>
            <p class="mt-1 text-sm text-gray-500">
              Verify identity documents and information
            </p>
          </div>
          <button
            onclick={closeModal}
            class="rounded-lg p-2 text-gray-400 transition-colors hover:bg-gray-100 hover:text-gray-600"
          >
            <XCircle class="h-6 w-6" />
          </button>
        </div>
      </div>

      <div class="p-8">
        <div class="mx-auto max-w-5xl space-y-8">
          <!-- User Info -->
          <div>
            <h4
              class="mb-4 text-sm font-semibold tracking-wide text-gray-500 uppercase"
            >
              Applicant Information
            </h4>
            <div
              class="rounded-xl border border-gray-200 bg-white p-6 shadow-sm"
            >
              <div class="flex items-start space-x-6">
                <img
                  src={getAvatarUrl(selectedKYC.userName)}
                  alt={selectedKYC.userName}
                  class="h-24 w-24 rounded-xl shadow-md"
                />
                <div class="flex-1">
                  <div class="flex items-center gap-3">
                    <h4 class="text-2xl font-bold text-gray-900">
                      {selectedKYC.userName}
                    </h4>
                    <span
                      class="rounded-full px-3 py-1 text-xs font-semibold uppercase {selectedKYC.location
                        ? 'bg-purple-100 text-purple-700'
                        : 'bg-blue-100 text-blue-700'}"
                    >
                      {selectedKYC.location ? "Agent" : "User"}
                    </span>
                  </div>
                  <div class="mt-4 flex gap-8">
                    <p class="flex items-center text-sm text-gray-600">
                      <span class="font-semibold">Email:</span>
                      <span class="ml-2">{selectedKYC.userEmail}</span>
                    </p>
                    <p class="flex items-center text-sm text-gray-600">
                      <span class="font-semibold">Phone:</span>
                      <span class="ml-2">{selectedKYC.userPhone || "N/A"}</span>
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Document Information -->
          <div>
            <h4
              class="mb-4 text-sm font-semibold tracking-wide text-gray-500 uppercase"
            >
              Document Details
            </h4>
            <div
              class="rounded-xl border border-gray-200 bg-white p-6 shadow-sm"
            >
              <div class="grid grid-cols-4 gap-6">
                <div>
                  <p
                    class="text-xs font-medium tracking-wide text-gray-500 uppercase"
                  >
                    Document Type
                  </p>
                  <p class="mt-2 text-base font-semibold text-gray-900">
                    {selectedKYC.documentType}
                  </p>
                </div>
                <div>
                  <p
                    class="text-xs font-medium tracking-wide text-gray-500 uppercase"
                  >
                    Document Number
                  </p>
                  <p
                    class="mt-2 font-mono text-base font-semibold text-gray-900"
                  >
                    {selectedKYC.documentNumber}
                  </p>
                </div>
                {#if selectedKYC.location}
                  <div>
                    <p
                      class="text-xs font-medium tracking-wide text-gray-500 uppercase"
                    >
                      Location
                    </p>
                    <p class="mt-2 text-base font-semibold text-gray-900">
                      {selectedKYC.location}
                    </p>
                  </div>
                {/if}
                {#if selectedKYC.businessLicense}
                  <div>
                    <p
                      class="text-xs font-medium tracking-wide text-gray-500 uppercase"
                    >
                      Business License
                    </p>
                    <p
                      class="mt-2 font-mono text-base font-semibold text-gray-900"
                    >
                      {selectedKYC.businessLicense}
                    </p>
                  </div>
                {/if}
              </div>
            </div>
          </div>

          <!-- Uploaded Documents -->
          <div>
            <h4
              class="mb-4 text-sm font-semibold tracking-wide text-gray-500 uppercase"
            >
              Uploaded Documents
            </h4>
            <div class="grid grid-cols-3 gap-4">
              {#each selectedKYC.documents as doc, index}
                <button
                  onclick={() => viewDocument(index)}
                  class="group overflow-hidden rounded-xl border-2 border-gray-200 bg-white shadow-sm transition-all hover:border-blue-500 hover:shadow-lg"
                >
                  <div class="relative">
                    <!-- Inline SVG placeholder -->
                    <svg
                      class="h-48 w-full"
                      viewBox="0 0 400 300"
                      xmlns="http://www.w3.org/2000/svg"
                    >
                      <rect width="400" height="300" fill="#f3f4f6" />
                      <text
                        x="50%"
                        y="45%"
                        text-anchor="middle"
                        font-family="Arial, sans-serif"
                        font-size="24"
                        fill="#6b7280"
                        font-weight="bold"
                      >
                        {doc.replace("_", " ").replace(".jpg", "")}
                      </text>
                      <text
                        x="50%"
                        y="55%"
                        text-anchor="middle"
                        font-family="Arial, sans-serif"
                        font-size="14"
                        fill="#9ca3af"
                      >
                        Click to view full size
                      </text>
                      <!-- Document icon -->
                      <g transform="translate(170, 100)">
                        <rect
                          x="10"
                          y="0"
                          width="40"
                          height="50"
                          fill="white"
                          stroke="#6b7280"
                          stroke-width="2"
                          rx="2"
                        />
                        <line
                          x1="15"
                          y1="10"
                          x2="45"
                          y2="10"
                          stroke="#6b7280"
                          stroke-width="1.5"
                        />
                        <line
                          x1="15"
                          y1="18"
                          x2="45"
                          y2="18"
                          stroke="#6b7280"
                          stroke-width="1.5"
                        />
                        <line
                          x1="15"
                          y1="26"
                          x2="35"
                          y2="26"
                          stroke="#6b7280"
                          stroke-width="1.5"
                        />
                      </g>
                    </svg>
                    <div
                      class="absolute inset-0 bg-black/0 transition-all group-hover:bg-black/10"
                    ></div>
                    <div class="absolute top-2 right-2">
                      <span
                        class="rounded-md bg-black/60 px-2 py-1 text-xs font-semibold text-white backdrop-blur-sm"
                      >
                        {index + 1}/{selectedKYC.documents.length}
                      </span>
                    </div>
                  </div>
                  <div class="border-t border-gray-200 bg-gray-50 px-3 py-2.5">
                    <p
                      class="flex items-center justify-center text-xs font-semibold text-gray-700"
                    >
                      <FileText class="mr-1.5 h-3.5 w-3.5 text-gray-400" />
                      {doc}
                    </p>
                  </div>
                </button>
              {/each}
            </div>
          </div>

          <!-- Review Status / Checklist -->
          {#if activeTab === "approved"}
            <!-- Approval Information -->
            <div>
              <h4
                class="mb-4 text-sm font-semibold tracking-wide text-gray-500 uppercase"
              >
                Approval Information
              </h4>
              <div
                class="rounded-xl border border-green-200 bg-gradient-to-br from-green-50 to-green-100/50 p-6 shadow-sm"
              >
                <div class="flex items-start space-x-3">
                  <CheckCircle class="mt-0.5 h-6 w-6 shrink-0 text-green-600" />
                  <div class="flex-1">
                    <p class="text-lg font-semibold text-green-900">
                      Application Approved
                    </p>
                    <div class="mt-4 space-y-2 text-sm text-green-800">
                      <p>
                        <span class="font-semibold">Approved by:</span>
                        {selectedKYC.approvedBy}
                      </p>
                      <p>
                        <span class="font-semibold">Approved on:</span>
                        {selectedKYC.approvedAt}
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          {:else if activeTab === "rejected"}
            <!-- Rejection Information -->
            <div>
              <h4
                class="mb-4 text-sm font-semibold tracking-wide text-gray-500 uppercase"
              >
                Rejection Information
              </h4>
              <div
                class="rounded-xl border border-red-200 bg-gradient-to-br from-red-50 to-red-100/50 p-6 shadow-sm"
              >
                <div class="flex items-start space-x-3">
                  <XCircle class="mt-0.5 h-6 w-6 shrink-0 text-red-600" />
                  <div class="flex-1">
                    <p class="text-lg font-semibold text-red-900">
                      Application Rejected
                    </p>
                    <div class="mt-4 space-y-3 text-sm">
                      <div>
                        <p class="font-semibold text-red-800">Reason:</p>
                        <p class="mt-1 text-red-700">{selectedKYC.reason}</p>
                      </div>
                      <div class="flex gap-6 text-red-800">
                        <p>
                          <span class="font-semibold">Rejected by:</span>
                          {selectedKYC.rejectedBy}
                        </p>
                        <p>
                          <span class="font-semibold">Rejected on:</span>
                          {selectedKYC.rejectedAt}
                        </p>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          {:else}
            <!-- Review Checklist for Pending -->
            <div>
              <h4
                class="mb-4 text-sm font-semibold tracking-wide text-gray-500 uppercase"
              >
                Review Checklist
              </h4>
              <div
                class="rounded-xl border border-blue-200 bg-gradient-to-br from-blue-50 to-blue-100/50 p-6 shadow-sm"
              >
                <div class="flex items-start space-x-3">
                  <Info class="mt-0.5 h-5 w-5 shrink-0 text-blue-600" />
                  <div class="flex-1">
                    <p class="font-semibold text-blue-900">
                      Verification Steps
                    </p>
                    <ul
                      class="mt-3 grid grid-cols-2 gap-x-8 gap-y-2 text-sm text-blue-800"
                    >
                      <li class="flex items-start">
                        <span class="mr-2">✓</span>
                        <span>Verify document is clear and readable</span>
                      </li>
                      <li class="flex items-start">
                        <span class="mr-2">✓</span>
                        <span>Check name matches across documents</span>
                      </li>
                      <li class="flex items-start">
                        <span class="mr-2">✓</span>
                        <span>Confirm document is not expired</span>
                      </li>
                      <li class="flex items-start">
                        <span class="mr-2">✓</span>
                        <span>For agents: verify business license</span>
                      </li>
                    </ul>
                  </div>
                </div>
              </div>
            </div>
          {/if}
        </div>
      </div>

      <!-- Actions Section -->
      {#if activeTab === "pending"}
        <div class="border-t border-gray-100 bg-gray-50 px-8 py-6">
          <!-- Rejection Reason -->
          <div class="mb-6">
            <label
              for="rejection-reason"
              class="text-sm font-semibold text-gray-700"
              >Rejection Reason <span class="text-gray-400"
                >(required if rejecting)</span
              ></label
            >
            <textarea
              id="rejection-reason"
              bind:value={rejectionReason}
              placeholder="Provide a detailed reason for rejection that will be sent to the applicant..."
              rows={3}
              class="mt-2 w-full rounded-xl border border-gray-300 bg-white p-4 text-sm shadow-sm transition-all focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:outline-none"
            ></textarea>
          </div>

          <!-- Actions -->
          <div class="flex gap-4">
            <button
              onclick={approveKYC}
              class="flex flex-1 items-center justify-center gap-2 rounded-xl bg-gradient-to-r from-green-600 to-green-700 px-6 py-4 font-semibold text-white shadow-lg transition-all hover:from-green-700 hover:to-green-800 hover:shadow-xl"
            >
              <CheckCircle class="h-5 w-5" />
              Approve Application
            </button>
            <button
              onclick={rejectKYC}
              disabled={!rejectionReason}
              class="flex flex-1 items-center justify-center gap-2 rounded-xl bg-gradient-to-r from-red-600 to-red-700 px-6 py-4 font-semibold text-white shadow-lg transition-all hover:from-red-700 hover:to-red-800 hover:shadow-xl disabled:cursor-not-allowed disabled:from-gray-400 disabled:to-gray-500 disabled:shadow-none"
            >
              <XCircle class="h-5 w-5" />
              Reject Application
            </button>
            <button
              onclick={closeModal}
              class="rounded-xl border-2 border-gray-300 bg-white px-6 py-4 font-semibold text-gray-700 transition-all hover:border-gray-400 hover:bg-gray-50"
            >
              Cancel
            </button>
          </div>
        </div>
      {:else}
        <!-- Close button for approved/rejected -->
        <div class="border-t border-gray-100 bg-gray-50 px-8 py-6">
          <button
            onclick={closeModal}
            class="w-full rounded-xl bg-gradient-to-r from-blue-600 to-blue-700 px-6 py-4 font-semibold text-white shadow-lg transition-all hover:from-blue-700 hover:to-blue-800 hover:shadow-xl"
          >
            Close
          </button>
        </div>
      {/if}
    </div>
  </div>
{/if}

<!-- Document Lightbox -->
{#if selectedDocumentIndex !== null && selectedKYC}
  <div
    class="fixed inset-0 z-[60] flex items-center justify-center bg-black/90 p-4"
    onclick={closeDocumentView}
  >
    <button
      onclick={(e) => {
        e.stopPropagation();
        closeDocumentView();
      }}
      class="absolute top-4 right-4 rounded-lg bg-white/10 p-2 text-white backdrop-blur-sm transition-all hover:bg-white/20"
    >
      <X class="h-6 w-6" />
    </button>

    <!-- Previous Button -->
    {#if selectedKYC.documents.length > 1}
      <button
        onclick={(e) => {
          e.stopPropagation();
          previousDocument();
        }}
        class="absolute top-1/2 left-4 -translate-y-1/2 rounded-lg bg-white/10 p-3 text-white backdrop-blur-sm transition-all hover:bg-white/20"
      >
        <ChevronLeft class="h-8 w-8" />
      </button>
    {/if}

    <!-- Document Display -->
    <div class="max-h-[90vh] max-w-5xl" role="button" tabindex="0" onclick={(e) => e.stopPropagation()}>
      <div class="rounded-2xl bg-white p-4 shadow-2xl">
        <!-- Document Info -->
        <div
          class="mb-4 flex items-center justify-between border-b border-gray-200 pb-4"
        >
          <div>
            <h4 class="text-lg font-bold text-gray-900">
              {selectedKYC.documents[selectedDocumentIndex]}
            </h4>
            <p class="text-sm text-gray-500">
              Document {selectedDocumentIndex + 1} of {selectedKYC.documents
                .length}
            </p>
          </div>
        </div>

        <!-- Full Size Document -->
        <svg
          class="h-[70vh] w-full"
          viewBox="0 0 800 1000"
          xmlns="http://www.w3.org/2000/svg"
          preserveAspectRatio="xMidYMid meet"
        >
          <!-- White background -->
          <rect width="800" height="1000" fill="white" />

          <!-- Header -->
          <rect width="800" height="80" fill="#3b82f6" />
          <text
            x="400"
            y="50"
            text-anchor="middle"
            font-family="Arial, sans-serif"
            font-size="32"
            fill="white"
            font-weight="bold"
          >
            {selectedKYC.documents[selectedDocumentIndex]
              .replace("_", " ")
              .replace(".jpg", "")
              .toUpperCase()}
          </text>

          <!-- Document content area -->
          <rect
            x="50"
            y="120"
            width="700"
            height="800"
            fill="#f9fafb"
            stroke="#e5e7eb"
            stroke-width="2"
            rx="8"
          />

          <!-- Document icon -->
          <g transform="translate(300, 300)">
            <rect
              width="200"
              height="250"
              fill="white"
              stroke="#6b7280"
              stroke-width="3"
              rx="8"
            />
            <line
              x1="30"
              y1="40"
              x2="170"
              y2="40"
              stroke="#6b7280"
              stroke-width="3"
            />
            <line
              x1="30"
              y1="80"
              x2="170"
              y2="80"
              stroke="#6b7280"
              stroke-width="3"
            />
            <line
              x1="30"
              y1="120"
              x2="170"
              y2="120"
              stroke="#6b7280"
              stroke-width="3"
            />
            <line
              x1="30"
              y1="160"
              x2="140"
              y2="160"
              stroke="#6b7280"
              stroke-width="3"
            />
          </g>

          <!-- Document name -->
          <text
            x="400"
            y="600"
            text-anchor="middle"
            font-family="Arial, sans-serif"
            font-size="24"
            fill="#374151"
            font-weight="bold"
          >
            {selectedKYC.documents[selectedDocumentIndex]
              .replace("_", " ")
              .replace(".jpg", "")}
          </text>

          <!-- Document number -->
          <text
            x="400"
            y="650"
            text-anchor="middle"
            font-family="monospace"
            font-size="20"
            fill="#6b7280"
          >
            {selectedKYC.documentNumber}
          </text>

          <!-- User name -->
          <text
            x="400"
            y="700"
            text-anchor="middle"
            font-family="Arial, sans-serif"
            font-size="18"
            fill="#9ca3af"
          >
            {selectedKYC.userName}
          </text>

          <!-- Footer -->
          <rect y="920" width="800" height="80" fill="#f3f4f6" />
          <text
            x="400"
            y="965"
            text-anchor="middle"
            font-family="Arial, sans-serif"
            font-size="16"
            fill="#6b7280"
          >
            This is a placeholder document preview
          </text>
        </svg>
      </div>
    </div>

    <!-- Next Button -->
    {#if selectedKYC.documents.length > 1}
      <button
        onclick={(e) => {
          e.stopPropagation();
          nextDocument();
        }}
        class="absolute top-1/2 right-4 -translate-y-1/2 rounded-lg bg-white/10 p-3 text-white backdrop-blur-sm transition-all hover:bg-white/20"
      >
        <ChevronRight class="h-8 w-8" />
      </button>
    {/if}
  </div>
{/if}
