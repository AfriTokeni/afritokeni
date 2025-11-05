<script lang="ts">
  import {
    CheckCircle,
    XCircle,
    Eye,
    FileText,
    User,
    MapPin,
    Info,
  } from "lucide-svelte";
  import type { ApexOptions } from 'apexcharts';
  import { Chart } from '@flowbite-svelte-plugins/chart';

  let activeTab = $state<"pending" | "approved" | "rejected">("pending");
  let showReviewModal = $state(false);
  let selectedKYC = $state<any>(null);
  let rejectionReason = $state("");
  
  // KYC submissions trend chart options
  let chartOptions: ApexOptions = {
    chart: {
      height: '320px',
      type: 'area',
      fontFamily: 'Inter, sans-serif',
      dropShadow: { enabled: false },
      toolbar: { show: false },
    },
    tooltip: { enabled: true, x: { show: false } },
    fill: {
      type: 'gradient',
      gradient: {
        opacityFrom: 0.55,
        opacityTo: 0,
        shade: '#1C64F2',
        gradientToColors: ['#1C64F2'],
      },
    },
    dataLabels: { enabled: false },
    stroke: { width: 2, curve: 'smooth' },
    grid: {
      show: true,
      strokeDashArray: 4,
      padding: { left: 2, right: 2, top: 0 },
    },
    series: [
      {
        name: 'Submissions',
        data: [12, 15, 8, 18, 14, 22, 19],
        color: '#3b82f6',
      },
      {
        name: 'Approved',
        data: [10, 12, 7, 15, 11, 18, 16],
        color: '#22c55e',
      },
      {
        name: 'Rejected',
        data: [2, 3, 1, 3, 3, 4, 3],
        color: '#ef4444',
      },
    ],
    xaxis: {
      categories: ['Oct 29', 'Oct 30', 'Oct 31', 'Nov 1', 'Nov 2', 'Nov 3', 'Nov 4'],
      labels: {
        show: true,
        style: {
          fontFamily: 'Inter, sans-serif',
          cssClass: 'text-xs font-normal fill-gray-500',
        },
      },
      axisBorder: { show: false },
      axisTicks: { show: false },
    },
    yaxis: { show: true },
    legend: { show: true, position: 'top' },
  };

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
        avatar:
          "https://ui-avatars.com/api/?name=Alice+Brown&background=3b82f6&color=fff",
      },
      type: "user",
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
        avatar:
          "https://ui-avatars.com/api/?name=Charlie+Wilson&background=ef4444&color=fff",
      },
      type: "agent",
      rejectedAt: "Nov 2, 2024 02:10 PM",
      rejectedBy: "Admin User",
      reason: "Document image quality too low",
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
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- KYC Submissions Trend Chart -->
  <div class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6">
    <div class="mb-4 sm:mb-6">
      <h3 class="text-base font-semibold text-gray-900 sm:text-lg">KYC Submissions Trend</h3>
      <p class="text-xs text-gray-500 sm:text-sm">Last 7 days performance</p>
    </div>
    <div class="h-64 sm:h-80">
      <Chart options={chartOptions} />
    </div>
  </div>
  
  <!-- Header with Stats -->
  <div class="grid grid-cols-1 gap-4 sm:gap-6 md:grid-cols-3">
    <!-- Pending Count -->
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
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
    </div>

    <!-- Approved Count -->
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
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
    </div>

    <!-- Rejected Count -->
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 sm:rounded-2xl sm:p-6"
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

    <!-- Tab Content -->
    <div class="p-4 sm:p-6">
      {#if activeTab === "pending"}
        <div class="space-y-4">
          {#each pendingKYC as kyc}
            <div
              class="rounded-lg border border-gray-200 p-4 transition-all hover:border-gray-300 sm:p-6"
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
                <button
                  onclick={() => reviewKYC(kyc)}
                  class="rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-700"
                >
                  <Eye class="mr-2 inline h-4 w-4" />
                  Review
                </button>
              </div>
            </div>
          {/each}
        </div>
      {:else if activeTab === "approved"}
        <div class="space-y-4">
          {#each approvedKYC as kyc}
            <div
              class="rounded-lg border border-green-200 bg-green-50 p-4 sm:p-6"
            >
              <div class="flex items-start space-x-4">
                <img
                  src={kyc.user.avatar}
                  alt={kyc.user.name}
                  class="h-12 w-12 rounded-lg"
                />
                <div class="flex-1">
                  <div class="flex items-center space-x-2">
                    <h4 class="font-semibold text-gray-900">{kyc.user.name}</h4>
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
                <CheckCircle class="h-6 w-6 text-green-600" />
              </div>
            </div>
          {/each}
        </div>
      {:else}
        <div class="space-y-4">
          {#each rejectedKYC as kyc}
            <div class="rounded-lg border border-red-200 bg-red-50 p-4 sm:p-6">
              <div class="flex items-start space-x-4">
                <img
                  src={kyc.user.avatar}
                  alt={kyc.user.name}
                  class="h-12 w-12 rounded-lg"
                />
                <div class="flex-1">
                  <div class="flex items-center space-x-2">
                    <h4 class="font-semibold text-gray-900">{kyc.user.name}</h4>
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
                <XCircle class="h-6 w-6 text-red-600" />
              </div>
            </div>
          {/each}
        </div>
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
      class="max-h-[90vh] w-full max-w-2xl overflow-y-auto rounded-2xl bg-white p-6 shadow-xl"
    >
      <h3 class="text-xl font-bold text-gray-900">Review KYC Submission</h3>

      <!-- User Info -->
      <div
        class="mt-6 flex items-center space-x-4 rounded-lg border border-gray-200 p-4"
      >
        <img
          src={selectedKYC.user.avatar}
          alt={selectedKYC.user.name}
          class="h-16 w-16 rounded-lg"
        />
        <div>
          <h4 class="font-semibold text-gray-900">{selectedKYC.user.name}</h4>
          <p class="text-sm text-gray-500">{selectedKYC.user.email}</p>
          <p class="text-sm text-gray-500">{selectedKYC.user.phone}</p>
          <span
            class="mt-2 inline-block rounded-full px-2 py-1 text-xs font-medium {selectedKYC.type ===
            'agent'
              ? 'bg-purple-100 text-purple-800'
              : 'bg-blue-100 text-blue-800'}"
          >
            {selectedKYC.type}
          </span>
        </div>
      </div>

      <!-- Document Details -->
      <div class="mt-6 space-y-4">
        <h4 class="font-semibold text-gray-900">Document Information</h4>
        <div class="grid grid-cols-2 gap-4">
          <div class="rounded-lg border border-gray-200 p-3">
            <p class="text-xs text-gray-500">Document Type</p>
            <p class="mt-1 font-medium text-gray-900">
              {selectedKYC.documentType}
            </p>
          </div>
          <div class="rounded-lg border border-gray-200 p-3">
            <p class="text-xs text-gray-500">Document Number</p>
            <p class="mt-1 font-medium text-gray-900">
              {selectedKYC.documentNumber}
            </p>
          </div>
          {#if selectedKYC.location}
            <div class="rounded-lg border border-gray-200 p-3">
              <p class="text-xs text-gray-500">Location</p>
              <p class="mt-1 font-medium text-gray-900">
                {selectedKYC.location}
              </p>
            </div>
          {/if}
          {#if selectedKYC.businessLicense}
            <div class="rounded-lg border border-gray-200 p-3">
              <p class="text-xs text-gray-500">Business License</p>
              <p class="mt-1 font-medium text-gray-900">
                {selectedKYC.businessLicense}
              </p>
            </div>
          {/if}
        </div>
      </div>

      <!-- Documents -->
      <div class="mt-6 space-y-3">
        <h4 class="font-semibold text-gray-900">Uploaded Documents</h4>
        <div class="grid grid-cols-3 gap-4">
          {#each selectedKYC.documents as doc}
            <div class="rounded-lg border border-gray-200 p-4 text-center">
              <FileText class="mx-auto h-12 w-12 text-gray-400" />
              <p class="mt-2 text-xs text-gray-600">{doc}</p>
            </div>
          {/each}
        </div>
      </div>

      <!-- Info Box -->
      <div
        class="mt-6 flex items-start space-x-2 rounded-lg border border-blue-200 bg-blue-50 p-3"
      >
        <Info class="mt-0.5 h-4 w-4 shrink-0 text-blue-600" />
        <div class="text-xs text-blue-900">
          <p class="font-semibold">Review Checklist:</p>
          <ul class="mt-1 space-y-0.5 text-blue-800">
            <li>• Verify document is clear and readable</li>
            <li>• Check name matches across documents</li>
            <li>• Confirm document is not expired</li>
            <li>• For agents: verify business license</li>
          </ul>
        </div>
      </div>

      <!-- Rejection Reason -->
      <div class="mt-6">
        <label for="rejection-reason" class="text-sm font-medium text-gray-900"
          >Rejection Reason (if rejecting)</label
        >
        <textarea
          id="rejection-reason"
          bind:value={rejectionReason}
          placeholder="Provide a clear reason for rejection..."
          rows={3}
          class="mt-2 w-full rounded-lg border border-gray-200 p-3 text-sm focus:border-blue-600 focus:ring-2 focus:ring-blue-600 focus:outline-none"
        ></textarea>
      </div>

      <!-- Actions -->
      <div class="mt-6 flex gap-3">
        <button
          onclick={approveKYC}
          class="flex-1 rounded-lg bg-green-600 px-4 py-3 font-medium text-white transition-colors hover:bg-green-700"
        >
          <CheckCircle class="mr-2 inline h-5 w-5" />
          Approve KYC
        </button>
        <button
          onclick={rejectKYC}
          disabled={!rejectionReason}
          class="flex-1 rounded-lg bg-red-600 px-4 py-3 font-medium text-white transition-colors hover:bg-red-700 disabled:cursor-not-allowed disabled:opacity-50"
        >
          <XCircle class="mr-2 inline h-5 w-5" />
          Reject KYC
        </button>
        <button
          onclick={closeModal}
          class="rounded-lg border border-gray-200 px-4 py-3 font-medium text-gray-700 transition-colors hover:bg-gray-50"
        >
          Cancel
        </button>
      </div>
    </div>
  </div>
{/if}
