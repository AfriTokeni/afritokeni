<script lang="ts">
  interface Props {
    balance: number;
    totalSupply: number;
    breakdown?: {
      transactions: number;
      agentActivity: number;
      referrals: number;
      staking: number;
    };
  }

  let { balance, totalSupply, breakdown }: Props = $props();
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- Token Breakdown -->
  <div
    class="rounded-lg border border-gray-200 bg-white p-4 sm:rounded-xl sm:p-6"
  >
    <h3 class="mb-3 text-base font-bold text-gray-900 sm:mb-4 sm:text-lg">
      Token Breakdown
    </h3>
    <div class="space-y-2.5 sm:space-y-3">
      <div class="flex items-center justify-between gap-2">
        <span class="text-xs text-gray-600 sm:text-sm">From Transactions</span>
        <span
          class="text-xs font-semibold whitespace-nowrap text-gray-900 sm:text-sm"
          >{breakdown?.transactions || 0} AFRI</span
        >
      </div>
      <div class="flex items-center justify-between gap-2">
        <span class="text-xs text-gray-600 sm:text-sm">From Agent Activity</span
        >
        <span
          class="text-xs font-semibold whitespace-nowrap text-gray-900 sm:text-sm"
          >{breakdown?.agentActivity || 0} AFRI</span
        >
      </div>
      <div class="flex items-center justify-between gap-2">
        <span class="text-xs text-gray-600 sm:text-sm">From Referrals</span>
        <span
          class="text-xs font-semibold whitespace-nowrap text-gray-900 sm:text-sm"
          >{breakdown?.referrals || 0} AFRI</span
        >
      </div>
      <div class="flex items-center justify-between gap-2">
        <span class="text-xs text-gray-600 sm:text-sm">From Staking</span>
        <span
          class="text-xs font-semibold whitespace-nowrap text-gray-900 sm:text-sm"
          >{breakdown?.staking || 0} AFRI</span
        >
      </div>
      <div
        class="flex items-center justify-between gap-2 border-t border-gray-200 pt-2.5 sm:pt-3"
      >
        <span class="text-sm font-bold text-gray-900 sm:text-base"
          >Total Balance</span
        >
        <span
          class="text-base font-bold whitespace-nowrap text-gray-900 sm:text-lg lg:text-xl"
          >{balance.toLocaleString()} AFRI</span
        >
      </div>
    </div>
  </div>

  <!-- Earn AFRI via SMS -->
  <div
    class="rounded-lg border border-blue-200 bg-blue-50 p-4 sm:rounded-xl sm:p-6"
  >
    <h3 class="mb-2 text-sm font-semibold text-gray-900 sm:mb-3 sm:text-base">
      📱 Earn AFRI via SMS
    </h3>
    <div class="space-y-1.5 font-mono text-xs sm:space-y-2 sm:text-sm">
      <div class="wrap-break-word text-gray-700">
        <span class="text-blue-600">AFRI</span> - Check your AFRI balance
      </div>
      <div class="wrap-break-word text-gray-700">
        <span class="text-blue-600">VOTE YES PROP-001</span> - Vote on proposals
      </div>
    </div>
  </div>
</div>
