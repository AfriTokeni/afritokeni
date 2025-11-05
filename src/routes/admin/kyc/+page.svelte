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
  } from "lucide-svelte";
  import type { ApexOptions } from "apexcharts";
  import { Chart } from "@flowbite-svelte-plugins/chart";
  import { Button, Dropdown, DropdownItem } from "flowbite-svelte";

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

  // Generate chart data based on date range
  function getChartData() {
    if (chartDateRange === "7") {
      return {
        categories: [
          "Oct 29",
          "Oct 30",
          "Oct 31",
          "Nov 1",
          "Nov 2",
          "Nov 3",
          "Nov 4",
        ],
        submissions: [12, 15, 8, 18, 14, 22, 19],
        approved: [10, 12, 7, 15, 11, 18, 16],
        rejected: [2, 3, 1, 3, 3, 4, 3],
      };
    } else if (chartDateRange === "30") {
      return {
        categories: [
          "Oct 5",
          "Oct 8",
          "Oct 11",
          "Oct 14",
          "Oct 17",
          "Oct 20",
          "Oct 23",
          "Oct 26",
          "Oct 29",
          "Nov 1",
          "Nov 4",
        ],
        submissions: [8, 12, 10, 15, 13, 18, 16, 20, 17, 22, 19],
        approved: [7, 10, 8, 12, 11, 15, 13, 17, 14, 18, 16],
        rejected: [1, 2, 2, 3, 2, 3, 3, 3, 3, 4, 3],
      };
    } else {
      return {
        categories: [
          "Aug 6",
          "Aug 16",
          "Aug 26",
          "Sep 5",
          "Sep 15",
          "Sep 25",
          "Oct 5",
          "Oct 15",
          "Oct 25",
          "Nov 4",
        ],
        submissions: [5, 8, 10, 12, 14, 16, 18, 20, 22, 19],
        approved: [4, 7, 8, 10, 12, 13, 15, 17, 18, 16],
        rejected: [1, 1, 2, 2, 2, 3, 3, 3, 4, 3],
      };
    }
  }

  // KYC submissions trend chart options
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

  // Mock KYC data
  let pendingKYC = $state([
    {
      id: "KYC-001",
      user: {
        name: "John Doe",
        email: "john@example.com",
        phone: "+234 801 234 5678",
        avatar:
          "https://ui-avatars.com/api/?name=John+Doe&background=3b82f6&color=fff",
      },
      type: "user",
      documentType: "National ID",
      documentNumber: "NIN-12345678",
      submittedAt: "Nov 3, 2024 10:30 AM",
      documents: ["id_front.jpg", "id_back.jpg", "selfie.jpg"],
    },
    {
      id: "KYC-002",
      user: {
        name: "Jane Smith",
        email: "jane@example.com",
        phone: "+254 712 345 678",
        avatar:
          "https://ui-avatars.com/api/?name=Jane+Smith&background=8b5cf6&color=fff",
      },
      type: "agent",
      documentType: "Passport",
      documentNumber: "A12345678",
      location: "Nairobi, Kenya",
      businessLicense: "BL-987654",
      submittedAt: "Nov 3, 2024 09:15 AM",
      documents: ["passport.jpg", "business_license.pdf", "location_photo.jpg"],
    },
    {
      id: "KYC-003",
      user: {
        name: "Bob Johnson",
        email: "bob@example.com",
        phone: "+233 20 123 4567",
        avatar:
          "https://ui-avatars.com/api/?name=Bob+Johnson&background=10b981&color=fff",
      },
      type: "user",
      documentType: "Drivers License",
      documentNumber: "DL-ABC123",
      submittedAt: "Nov 2, 2024 08:45 PM",
      documents: ["license_front.jpg", "license_back.jpg"],
    },
  ]);

  let approvedKYC = $state([
    {
      id: "KYC-100",
      user: {
        name: "Alice Brown",
        email: "alice@example.com",
        phone: "+234 802 345 6789",
        avatar:
          "https://ui-avatars.com/api/?name=Alice+Brown&background=3b82f6&color=fff",
      },
      type: "user",
      documentType: "National ID",
      documentNumber: "NIN-87654321",
      submittedAt: "Nov 1, 2024 02:30 PM",
      documents: ["id_front.jpg", "id_back.jpg", "selfie.jpg"],
      approvedAt: "Nov 2, 2024 04:20 PM",
      approvedBy: "Admin User",
    },
  ]);

  let rejectedKYC = $state([
    {
      id: "KYC-200",
      user: {
        name: "Charlie Wilson",
        email: "charlie@example.com",
        phone: "+254 701 234 567",
        avatar:
          "https://ui-avatars.com/api/?name=Charlie+Wilson&background=ef4444&color=fff",
      },
      type: "agent",
      documentType: "Passport",
      documentNumber: "P-9876543",
      location: "Lagos, Nigeria",
      businessLicense: "BL-456789",
      submittedAt: "Oct 31, 2024 03:15 PM",
      documents: ["passport.jpg", "business_license.pdf", "location_photo.jpg"],
      rejectedAt: "Nov 1, 2024 11:30 AM",
      rejectedBy: "Admin User",
      reason:
        "Documents are blurry and unreadable. Please resubmit clear photos.",
    },
  ]);

  function reviewKYC(kyc: any) {
    selectedKYC = kyc;
    showReviewModal = true;
  }

  function approveKYC() {
    if (!selectedKYC) return;

    pendingKYC = pendingKYC.filter((k) => k.id !== selectedKYC.id);
    approvedKYC = [
      ...approvedKYC,
      {
        ...selectedKYC,
        approvedAt: new Date().toLocaleString(),
        approvedBy: "Admin User",
      },
    ];

    showReviewModal = false;
    selectedKYC = null;
  }

  function rejectKYC() {
    if (!selectedKYC || !rejectionReason) return;

    pendingKYC = pendingKYC.filter((k) => k.id !== selectedKYC.id);
    rejectedKYC = [
      ...rejectedKYC,
      {
        ...selectedKYC,
        rejectedAt: new Date().toLocaleString(),
        rejectedBy: "Admin User",
        reason: rejectionReason,
      },
    ];

    showReviewModal = false;
    selectedKYC = null;
    rejectionReason = "";
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
        ? kyc.user.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
          kyc.user.email.toLowerCase().includes(searchQuery.toLowerCase()) ||
          (kyc.user.phone && kyc.user.phone.includes(searchQuery))
        : true,
    ),
  );

  let filteredApprovedKYC = $derived(
    approvedKYC.filter((kyc) =>
      searchQuery
        ? kyc.user.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
          kyc.user.email.toLowerCase().includes(searchQuery.toLowerCase()) ||
          (kyc.user.phone && kyc.user.phone.includes(searchQuery))
        : true,
    ),
  );

  let filteredRejectedKYC = $derived(
    rejectedKYC.filter((kyc) =>
      searchQuery
        ? kyc.user.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
          kyc.user.email.toLowerCase().includes(searchQuery.toLowerCase()) ||
          (kyc.user.phone && kyc.user.phone.includes(searchQuery))
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
    <button
      onclick={() => (activeTab = "pending")}
      class="rounded-xl border border-gray-200 bg-white p-4 text-left transition-all hover:border-yellow-400 hover:shadow-md sm:rounded-2xl sm:p-6"
    >
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-semibold text-gray-500">Pending Review</p>
          <p class="mt-2 font-mono text-3xl font-bold text-yellow-600">
            {pendingKYC.length}
          </p>
        </div>
        <div
          class="flex h-12 w-12 items-center justify-center rounded-xl bg-yellow-50"
        >
          <FileText class="h-6 w-6 text-yellow-600" />
        </div>
      </div>
    </button>

    <!-- Approved Count -->
    <button
      onclick={() => (activeTab = "approved")}
      class="rounded-xl border border-gray-200 bg-white p-4 text-left transition-all hover:border-green-400 hover:shadow-md sm:rounded-2xl sm:p-6"
    >
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-semibold text-gray-500">Approved</p>
          <p class="mt-2 font-mono text-3xl font-bold text-green-600">
            {approvedKYC.length}
          </p>
        </div>
        <div
          class="flex h-12 w-12 items-center justify-center rounded-xl bg-green-50"
        >
          <CheckCircle class="h-6 w-6 text-green-600" />
        </div>
      </div>
    </button>

    <!-- Rejected Count -->
    <button
      onclick={() => (activeTab = "rejected")}
      class="rounded-xl border border-gray-200 bg-white p-4 text-left transition-all hover:border-red-400 hover:shadow-md sm:rounded-2xl sm:p-6"
    >
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm font-semibold text-gray-500">Rejected</p>
          <p class="mt-2 font-mono text-3xl font-bold text-red-600">
            {rejectedKYC.length}
          </p>
        </div>
        <div
          class="flex h-12 w-12 items-center justify-center rounded-xl bg-red-50"
        >
          <XCircle class="h-6 w-6 text-red-600" />
        </div>
      </div>
    </button>
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
                    src={kyc.user.avatar}
                    alt={kyc.user.name}
                    class="h-12 w-12 rounded-lg"
                  />
                  <div class="flex-1">
                    <div class="flex items-center space-x-2">
                      <h4 class="font-semibold text-gray-900">
                        {kyc.user.name}
                      </h4>
                      <span
                        class="rounded-full px-2 py-1 text-xs font-medium {kyc.type ===
                        'agent'
                          ? 'bg-purple-100 text-purple-800'
                          : 'bg-blue-100 text-blue-800'}"
                      >
                        {kyc.type}
                      </span>
                    </div>
                    <p class="mt-1 text-sm text-gray-500">
                      {kyc.user.email} • {kyc.user.phone}
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
                    src={kyc.user.avatar}
                    alt={kyc.user.name}
                    class="h-12 w-12 rounded-lg"
                  />
                  <div class="flex-1">
                    <div class="flex items-center space-x-2">
                      <h4 class="font-semibold text-gray-900">
                        {kyc.user.name}
                      </h4>
                      <span
                        class="rounded-full bg-green-100 px-2 py-1 text-xs font-medium text-green-800"
                      >
                        {kyc.type}
                      </span>
                    </div>
                    <p class="mt-1 text-sm text-gray-600">{kyc.user.email}</p>
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
                    src={kyc.user.avatar}
                    alt={kyc.user.name}
                    class="h-12 w-12 rounded-lg"
                  />
                  <div class="flex-1">
                    <div class="flex items-center space-x-2">
                      <h4 class="font-semibold text-gray-900">
                        {kyc.user.name}
                      </h4>
                      <span
                        class="rounded-full bg-red-100 px-2 py-1 text-xs font-medium text-red-800"
                      >
                        {kyc.type}
                      </span>
                    </div>
                    <p class="mt-1 text-sm text-gray-600">{kyc.user.email}</p>
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
                  src={selectedKYC.user.avatar}
                  alt={selectedKYC.user.name}
                  class="h-24 w-24 rounded-xl shadow-md"
                />
                <div class="flex-1">
                  <div class="flex items-center gap-3">
                    <h4 class="text-2xl font-bold text-gray-900">
                      {selectedKYC.user.name}
                    </h4>
                    <span
                      class="rounded-full px-3 py-1 text-xs font-semibold uppercase {selectedKYC.type ===
                      'agent'
                        ? 'bg-purple-100 text-purple-700'
                        : 'bg-blue-100 text-blue-700'}"
                    >
                      {selectedKYC.type}
                    </span>
                  </div>
                  <div class="mt-4 flex gap-8">
                    <p class="flex items-center text-sm text-gray-600">
                      <span class="font-semibold">Email:</span>
                      <span class="ml-2">{selectedKYC.user.email}</span>
                    </p>
                    <p class="flex items-center text-sm text-gray-600">
                      <span class="font-semibold">Phone:</span>
                      <span class="ml-2">{selectedKYC.user.phone}</span>
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
    <div class="max-h-[90vh] max-w-5xl" onclick={(e) => e.stopPropagation()}>
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
            {selectedKYC.user.name}
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
