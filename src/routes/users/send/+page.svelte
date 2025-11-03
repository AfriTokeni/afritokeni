<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import {
    Send,
    Bitcoin,
    DollarSign,
    AlertCircle,
    Search,
  } from "@lucide/svelte";
  import {
    formatCurrencyAmount,
    type AfricanCurrency,
  } from "$lib/types/currency";
  import PrimaryBalanceCard from "$lib/components/dashboard/PrimaryBalanceCard.svelte";
  import CkBTCBalanceCard from "$lib/components/shared/CkBTCBalanceCard.svelte";
  import CkUSDBalanceCard from "$lib/components/shared/CkUSDBalanceCard.svelte";
  import { getUserData, getUserBalance } from "$lib/services/user/userService";

  type SendType = "local" | "ckbtc" | "ckusd";
  type SendStep = "amount" | "recipient" | "confirmation";

  // State
  let currentStep = $state<SendStep>("amount");
  let sendType = $state<SendType>("local");
  let selectedCurrency = $state("");
  let localAmount = $state("");
  let recipientPhone = $state("");
  let recipientName = $state("");
  let error = $state("");
  let transactionCode = $state("");
  let searchQuery = $state("");
  let showContactDropdown = $state(false);
  let walletAddress = $state("");
  let currentUser = $state<any>(null);
  let displayBalance = $state(0);

  const defaultCurrency = $derived(currentUser?.preferredCurrency || "UGX");
  const userCurrency = $derived(selectedCurrency || defaultCurrency);

  onMount(async () => {
    currentUser = await getUserData();
    displayBalance = await getUserBalance();
  });

  // Demo contacts
  const demoContacts = [
    {
      name: "Jane Doe",
      phone: "+256 700 123 456",
      btcWallet: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
      usdcWallet: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    },
    {
      name: "John Smith",
      phone: "+256 701 234 567",
      btcWallet: "",
      usdcWallet: "0x8ba1f109551bD432803012645Ac136ddd64DBA72",
    },
    {
      name: "Janet Mukasa",
      phone: "+256 702 345 678",
      btcWallet: "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq",
      usdcWallet: "",
    },
    {
      name: "James Okello",
      phone: "+256 703 456 789",
      btcWallet: "",
      usdcWallet: "",
    },
    {
      name: "Sarah Nakato",
      phone: "+256 704 567 890",
      btcWallet: "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4",
      usdcWallet: "0xdAC17F958D2ee523a2206206994597C13D831ec7",
    },
  ];

  const filteredContacts = $derived(
    demoContacts.filter(
      (contact) =>
        contact.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        contact.phone.includes(searchQuery),
    ),
  );

  function calculateFee(amount: number): number {
    return Math.round(amount * 0.005);
  }

  function handleAmountChange(value: string) {
    localAmount = value;
    error = "";

    const amount = parseFloat(value);
    if (isNaN(amount) || amount <= 0) return;

    const fee = calculateFee(amount);
    const totalRequired = amount + fee;

    if (totalRequired > displayBalance) {
      error = `Insufficient balance. You need ${formatCurrencyAmount(totalRequired, userCurrency as AfricanCurrency)} (including 0.5% fee)`;
    }
  }

  function handleContinueToRecipient() {
    const amount = parseFloat(localAmount);
    if (!amount || amount <= 0) {
      error = "Please enter a valid amount";
      return;
    }

    const fee = calculateFee(amount);
    const totalRequired = amount + fee;

    if (totalRequired > displayBalance) {
      error = `Insufficient balance. You need ${formatCurrencyAmount(totalRequired, userCurrency as AfricanCurrency)}`;
      return;
    }

    currentStep = "recipient";
  }

  function handleSendMoney() {
    if (sendType === "local" && !recipientPhone.trim()) {
      error = "Please select a recipient or enter phone number";
      return;
    }

    if (
      (sendType === "ckbtc" || sendType === "ckusd") &&
      !walletAddress.trim()
    ) {
      error = "Please enter wallet address";
      return;
    }

    const code = Math.random().toString(36).substring(2, 8).toUpperCase();
    transactionCode = code;
    currentStep = "confirmation";
  }

  function selectContact(contact: (typeof demoContacts)[0]) {
    recipientPhone = contact.phone;
    recipientName = contact.name;
    searchQuery = contact.name;
    showContactDropdown = false;

    if (sendType === "ckbtc" && contact.btcWallet) {
      walletAddress = contact.btcWallet;
    } else if (sendType === "ckusd" && contact.usdcWallet) {
      walletAddress = contact.usdcWallet;
    }
  }
</script>

<div class="space-y-4 sm:space-y-6">
  <!-- Step Indicator -->
  <div class="mb-6 flex items-center justify-center px-2 sm:mb-8">
    <div class="flex items-center space-x-2 overflow-x-auto pb-2 sm:space-x-4">
      <div
        class="flex items-center space-x-1.5 sm:space-x-2 {currentStep ===
        'amount'
          ? 'text-gray-900'
          : 'text-green-600'}"
      >
        <div
          class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full text-xs font-semibold sm:h-8 sm:w-8 sm:text-sm {currentStep ===
          'amount'
            ? 'bg-gray-900 text-white'
            : 'bg-green-600 text-white'}"
        >
          1
        </div>
        <span class="text-xs font-medium whitespace-nowrap sm:text-sm"
          >Enter Amount</span
        >
      </div>

      <div
        class="h-0.5 w-4 shrink-0 sm:w-8 {currentStep === 'recipient' ||
        currentStep === 'confirmation'
          ? 'bg-green-600'
          : 'bg-gray-200'}"
      ></div>

      <div
        class="flex items-center space-x-1.5 sm:space-x-2 {currentStep ===
        'recipient'
          ? 'text-gray-900'
          : currentStep === 'confirmation'
            ? 'text-green-600'
            : 'text-gray-400'}"
      >
        <div
          class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full text-xs font-semibold sm:h-8 sm:w-8 sm:text-sm {currentStep ===
          'recipient'
            ? 'bg-gray-900 text-white'
            : currentStep === 'confirmation'
              ? 'bg-green-600 text-white'
              : 'bg-gray-200'}"
        >
          2
        </div>
        <span class="text-xs font-medium whitespace-nowrap sm:text-sm"
          >Recipient Details</span
        >
      </div>

      <div
        class="h-0.5 w-4 shrink-0 sm:w-8 {currentStep === 'confirmation'
          ? 'bg-green-600'
          : 'bg-gray-200'}"
      ></div>

      <div
        class="flex items-center space-x-1.5 sm:space-x-2 {currentStep ===
        'confirmation'
          ? 'text-gray-900'
          : 'text-gray-400'}"
      >
        <div
          class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full text-xs font-semibold sm:h-8 sm:w-8 sm:text-sm {currentStep ===
          'confirmation'
            ? 'bg-gray-900 text-white'
            : 'bg-gray-200'}"
        >
          3
        </div>
        <span class="text-xs font-medium whitespace-nowrap sm:text-sm"
          >Confirmation</span
        >
      </div>
    </div>
  </div>

  <!-- Amount Step -->
  {#if currentStep === "amount"}
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 sm:rounded-2xl sm:p-6 lg:p-8"
    >
      <h2 class="mb-4 text-xl font-bold text-gray-900 sm:mb-6 sm:text-2xl">
        Enter Send Amount
      </h2>

      <!-- Balance Cards - All in one row -->
      <div class="mb-4 sm:mb-6">
        <div class="grid grid-cols-1 gap-3 sm:gap-4 md:grid-cols-3">
          <PrimaryBalanceCard
            balance={displayBalance}
            currency={userCurrency}
            onCurrencyChange={(currency) => (selectedCurrency = currency)}
          />
          <!-- Encapsulated components - fetch their own data -->
          <CkBTCBalanceCard
            preferredCurrency={userCurrency}
            showActions={false}
          />
          <CkUSDBalanceCard
            preferredCurrency={userCurrency}
            showActions={false}
          />
        </div>
      </div>

      <!-- Send Type Selection -->
      <div class="mb-4 sm:mb-6">
        <div
          class="mb-2 block text-xs font-semibold text-gray-900 sm:mb-3 sm:text-sm"
        >
          What would you like to send?
        </div>
        <div class="grid grid-cols-1 gap-3 sm:gap-4 md:grid-cols-3">
          <button
            type="button"
            onclick={() => (sendType = "local")}
            class="rounded-xl border-2 p-4 text-left transition-all sm:rounded-2xl sm:p-6 {sendType ===
            'local'
              ? 'border-gray-900 bg-gray-50'
              : 'border-gray-200 hover:border-gray-300'}"
          >
            <div class="flex items-start space-x-2 sm:space-x-3">
              <div
                class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg sm:h-12 sm:w-12 sm:rounded-xl {sendType ===
                'local'
                  ? 'bg-gray-100'
                  : 'bg-gray-50'}"
              >
                <Send class="h-5 w-5 text-gray-900 sm:h-6 sm:w-6" />
              </div>
              <div class="min-w-0">
                <h3 class="mb-1 text-sm font-bold text-gray-900 sm:text-base">
                  Send Local Currency
                </h3>
                <p class="text-xs text-gray-600 sm:text-sm">
                  Send {userCurrency} to another user
                </p>
              </div>
            </div>
          </button>

          <button
            type="button"
            onclick={() => (sendType = "ckbtc")}
            class="rounded-xl border-2 p-4 text-left transition-all sm:rounded-2xl sm:p-6 {sendType ===
            'ckbtc'
              ? 'border-gray-900 bg-gray-50'
              : 'border-gray-200 hover:border-gray-300'}"
          >
            <div class="flex items-start space-x-2 sm:space-x-3">
              <div
                class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-orange-50 sm:h-12 sm:w-12 sm:rounded-xl"
              >
                <Bitcoin class="h-5 w-5 text-orange-600 sm:h-6 sm:w-6" />
              </div>
              <div class="min-w-0">
                <h3 class="mb-1 text-sm font-bold text-gray-900 sm:text-base">
                  Send ckBTC
                </h3>
                <p class="text-xs text-gray-600 sm:text-sm">
                  Send Chain Key Bitcoin
                </p>
              </div>
            </div>
          </button>

          <button
            type="button"
            onclick={() => (sendType = "ckusd")}
            class="rounded-xl border-2 p-4 text-left transition-all sm:rounded-2xl sm:p-6 {sendType ===
            'ckusd'
              ? 'border-gray-900 bg-gray-50'
              : 'border-gray-200 hover:border-gray-300'}"
          >
            <div class="flex items-start space-x-2 sm:space-x-3">
              <div
                class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-green-50 sm:h-12 sm:w-12 sm:rounded-xl"
              >
                <DollarSign class="h-5 w-5 text-green-600 sm:h-6 sm:w-6" />
              </div>
              <div class="min-w-0">
                <h3 class="mb-1 text-sm font-bold text-gray-900 sm:text-base">
                  Send ckUSD
                </h3>
                <p class="text-xs text-gray-600 sm:text-sm">
                  Send Chain Key USD
                </p>
              </div>
            </div>
          </button>
        </div>
      </div>

      <!-- Error Message -->
      {#if error}
        <div
          class="mb-3 rounded-lg border border-red-200 bg-red-50 p-3 sm:mb-4 sm:p-4"
        >
          <div class="flex items-start">
            <AlertCircle
              class="mt-0.5 mr-2 h-4 w-4 shrink-0 text-red-500 sm:h-5 sm:w-5"
            />
            <p class="text-xs text-red-700 sm:text-sm">{error}</p>
          </div>
        </div>
      {/if}

      <!-- Amount Input -->
      <div class="mb-4 sm:mb-6">
        <div class="mb-2 flex items-center justify-between sm:mb-3">
          <label
            for="amount"
            class="block text-xs font-medium text-gray-700 sm:text-sm"
          >
            Amount {sendType === "ckbtc" && "(ckBTC)"}
            {sendType === "ckusd" && "(ckUSD)"}
          </label>
        </div>
        <input
          id="amount"
          type="number"
          bind:value={localAmount}
          oninput={(e) => handleAmountChange(e.currentTarget.value)}
          placeholder={sendType === "ckbtc"
            ? "0.00000000"
            : sendType === "ckusd"
              ? "0.00"
              : "0"}
          class="w-full rounded-lg border border-gray-300 px-3 py-3 font-mono text-base focus:border-transparent focus:ring-2 focus:ring-gray-500 sm:px-4 sm:py-4 sm:text-lg"
          step={sendType === "ckbtc" ? "0.00000001" : "0.01"}
        />
      </div>

      <!-- Fee Summary -->
      {#if localAmount && parseFloat(localAmount) > 0}
        <div
          class="mb-4 space-y-2 rounded-lg bg-gray-50 p-3 text-xs sm:mb-6 sm:p-4 sm:text-sm"
        >
          <div class="flex justify-between">
            <span>Amount:</span>
            <span class="font-medium">
              {sendType === "ckbtc"
                ? `₿${parseFloat(localAmount).toFixed(8)} BTC`
                : sendType === "ckusd"
                  ? `$${parseFloat(localAmount).toFixed(2)} USDC`
                  : formatCurrencyAmount(
                      parseFloat(localAmount),
                      userCurrency as AfricanCurrency,
                    )}
            </span>
          </div>
          <div class="flex justify-between">
            <span>Fee (0.5%):</span>
            <span>
              {sendType === "ckbtc"
                ? `₿${(parseFloat(localAmount) * 0.005).toFixed(8)} BTC`
                : sendType === "ckusd"
                  ? `$${(parseFloat(localAmount) * 0.005).toFixed(2)} USDC`
                  : formatCurrencyAmount(
                      calculateFee(parseFloat(localAmount)),
                      userCurrency as AfricanCurrency,
                    )}
            </span>
          </div>
          <div
            class="flex justify-between border-t border-gray-200 pt-2 font-semibold"
          >
            <span>Total:</span>
            <span>
              {sendType === "ckbtc"
                ? `₿${(parseFloat(localAmount) * 1.005).toFixed(8)} BTC`
                : sendType === "ckusd"
                  ? `$${(parseFloat(localAmount) * 1.005).toFixed(2)} USDC`
                  : formatCurrencyAmount(
                      parseFloat(localAmount) +
                        calculateFee(parseFloat(localAmount)),
                      userCurrency as AfricanCurrency,
                    )}
            </span>
          </div>
        </div>
      {/if}

      <button
        onclick={handleContinueToRecipient}
        disabled={!localAmount || parseFloat(localAmount) <= 0 || !!error}
        class="w-full rounded-lg bg-gray-900 py-3 text-sm font-semibold text-white transition-colors hover:bg-gray-800 disabled:cursor-not-allowed disabled:bg-gray-300 sm:py-4 sm:text-base"
      >
        Continue
      </button>
    </div>
  {/if}

  <!-- Recipient Step -->
  {#if currentStep === "recipient"}
    <div
      class="rounded-xl border border-gray-200 bg-white p-4 sm:rounded-2xl sm:p-6 lg:p-8"
    >
      <h2 class="mb-4 text-xl font-bold text-gray-900 sm:mb-6 sm:text-2xl">
        Enter Recipient Details
      </h2>

      <div class="mb-4 space-y-3 sm:mb-6 sm:space-y-4">
        <!-- Contact Search -->
        <div>
          <div class="mb-2 block text-xs font-medium text-gray-700 sm:text-sm">
            Search Recipient (by name or phone) *
          </div>
          <div class="relative">
            <Search
              class="absolute top-1/2 left-3 z-10 h-4 w-4 -translate-y-1/2 transform text-gray-400 sm:h-5 sm:w-5"
            />
            <input
              type="text"
              bind:value={searchQuery}
              oninput={() => {
                showContactDropdown = true;
                walletAddress = "";
              }}
              onfocus={() => (showContactDropdown = true)}
              placeholder="Search by name or enter phone number..."
              class="w-full rounded-lg border border-gray-300 py-2.5 pr-3 pl-9 text-sm focus:border-transparent focus:ring-2 focus:ring-gray-500 sm:py-3 sm:pr-4 sm:pl-10 sm:text-base"
            />

            <!-- Contact Dropdown -->
            {#if showContactDropdown && searchQuery && filteredContacts.length > 0}
              <div
                class="absolute z-20 mt-1 max-h-60 w-full overflow-y-auto rounded-lg border border-gray-200 bg-white shadow-lg"
              >
                {#each filteredContacts as contact}
                  {@const hasWallet =
                    sendType === "ckbtc"
                      ? contact.btcWallet
                      : sendType === "ckusd"
                        ? contact.usdcWallet
                        : true}
                  <button
                    type="button"
                    onclick={() => selectContact(contact)}
                    class="w-full border-b border-gray-100 px-3 py-2.5 text-left last:border-b-0 hover:bg-gray-50 sm:px-4 sm:py-3"
                  >
                    <div class="flex items-center justify-between gap-2">
                      <div class="min-w-0">
                        <div
                          class="truncate text-sm font-medium text-gray-900 sm:text-base"
                        >
                          {contact.name}
                        </div>
                        <div class="truncate text-xs text-gray-500 sm:text-sm">
                          {contact.phone}
                        </div>
                      </div>
                      {#if sendType === "ckbtc" || sendType === "ckusd"}
                        <div class="shrink-0 text-xs">
                          {#if hasWallet}
                            <span class="text-green-600">✓ Has wallet</span>
                          {:else}
                            <span class="text-gray-400">No wallet</span>
                          {/if}
                        </div>
                      {/if}
                    </div>
                  </button>
                {/each}
              </div>
            {/if}
          </div>
          <p class="mt-1 text-xs text-gray-500">
            {sendType === "local"
              ? "Search contacts or enter phone number"
              : "Search registered users or enter wallet address below"}
          </p>
        </div>

        <!-- Wallet Address for crypto -->
        {#if sendType === "ckbtc" || sendType === "ckusd"}
          <div>
            <div
              class="mb-2 block text-xs font-medium text-gray-700 sm:text-sm"
            >
              {sendType === "ckbtc" ? "Bitcoin" : "USDC"} Wallet Address {walletAddress &&
                "✓"}
            </div>
            <div class="relative">
              <input
                type="text"
                bind:value={walletAddress}
                placeholder={sendType === "ckbtc"
                  ? "bc1q... or select contact above"
                  : "0x... or select contact above"}
                class="w-full rounded-lg border border-gray-300 px-3 py-2.5 font-mono text-xs break-all focus:border-transparent focus:ring-2 focus:ring-gray-500 sm:px-4 sm:py-3 sm:text-sm"
              />
            </div>
            <p class="mt-1 text-xs text-gray-500">
              {walletAddress
                ? "Wallet address ready"
                : "Enter manually or select a contact with registered wallet"}
            </p>
          </div>
        {/if}

        <!-- Recipient Name when selected -->
        {#if recipientName}
          <div>
            <div
              class="mb-2 block text-xs font-medium text-gray-700 sm:text-sm"
            >
              Recipient Name ✓
            </div>
            <input
              type="text"
              value={recipientName}
              readonly
              class="w-full rounded-lg border border-gray-300 bg-gray-50 px-3 py-2.5 text-sm text-gray-700 sm:px-4 sm:py-3 sm:text-base"
            />
          </div>
        {/if}
      </div>

      <!-- Transaction Summary -->
      <div class="mb-4 rounded-lg bg-gray-50 p-3 sm:mb-6 sm:p-4">
        <h3 class="mb-2 text-sm font-semibold sm:mb-3 sm:text-base">
          Transaction Summary
        </h3>
        <div class="space-y-2 text-xs sm:text-sm">
          <div class="flex justify-between">
            <span class="text-gray-600">Amount:</span>
            <span class="font-medium">
              {sendType === "ckbtc"
                ? `₿${parseFloat(localAmount).toFixed(8)} BTC`
                : sendType === "ckusd"
                  ? `$${parseFloat(localAmount).toFixed(2)} USDC`
                  : formatCurrencyAmount(
                      parseFloat(localAmount),
                      userCurrency as AfricanCurrency,
                    )}
            </span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">Fee (0.5%):</span>
            <span class="font-medium text-orange-600">
              {sendType === "ckbtc"
                ? `₿${(parseFloat(localAmount) * 0.005).toFixed(8)} BTC`
                : sendType === "ckusd"
                  ? `$${(parseFloat(localAmount) * 0.005).toFixed(2)} USDC`
                  : formatCurrencyAmount(
                      calculateFee(parseFloat(localAmount)),
                      userCurrency as AfricanCurrency,
                    )}
            </span>
          </div>
          <div
            class="flex justify-between border-t border-gray-200 pt-2 text-sm font-semibold sm:text-base"
          >
            <span>Total:</span>
            <span>
              {sendType === "ckbtc"
                ? `₿${(parseFloat(localAmount) * 1.005).toFixed(8)} BTC`
                : sendType === "ckusd"
                  ? `$${(parseFloat(localAmount) * 1.005).toFixed(2)} USDC`
                  : formatCurrencyAmount(
                      parseFloat(localAmount) +
                        calculateFee(parseFloat(localAmount)),
                      userCurrency as AfricanCurrency,
                    )}
            </span>
          </div>
        </div>
      </div>

      {#if error}
        <div
          class="mb-3 rounded-lg border border-red-200 bg-red-50 p-3 sm:mb-4 sm:p-4"
        >
          <p class="text-xs text-red-700 sm:text-sm">{error}</p>
        </div>
      {/if}

      <div class="flex gap-3 sm:gap-4">
        <button
          onclick={() => (currentStep = "amount")}
          class="flex-1 rounded-lg bg-gray-100 py-2.5 text-sm font-semibold text-gray-700 transition-colors hover:bg-gray-200 sm:py-3 sm:text-base"
        >
          Back
        </button>
        <button
          onclick={handleSendMoney}
          disabled={sendType === "local"
            ? !recipientPhone.trim()
            : !walletAddress.trim()}
          class="flex-1 rounded-lg bg-gray-900 py-2.5 text-sm font-semibold text-white transition-colors hover:bg-gray-800 disabled:cursor-not-allowed disabled:bg-gray-300 sm:py-3 sm:text-base"
        >
          Send Money
        </button>
      </div>
    </div>
  {/if}

  <!-- Confirmation Step -->
  {#if currentStep === "confirmation"}
    <div
      class="rounded-xl border border-gray-200 bg-white p-6 text-center sm:rounded-2xl sm:p-8"
    >
      <div
        class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-green-100 sm:mb-6 sm:h-20 sm:w-20"
      >
        <Send class="h-8 w-8 text-green-600 sm:h-10 sm:w-10" />
      </div>
      <h2 class="mb-2 text-xl font-bold text-gray-900 sm:text-2xl">
        Money Sent Successfully!
      </h2>
      <p class="mb-4 px-2 text-sm text-gray-600 sm:mb-6 sm:text-base">
        {formatCurrencyAmount(
          parseFloat(localAmount),
          userCurrency as AfricanCurrency,
        )} has been sent to {recipientPhone}
      </p>

      <div class="mb-4 rounded-lg bg-gray-50 p-4 sm:mb-6 sm:p-6">
        <div class="space-y-2 text-xs sm:space-y-3 sm:text-sm">
          <div class="flex justify-between gap-2">
            <span class="text-gray-600">Transaction Code:</span>
            <span class="font-mono font-bold break-all">{transactionCode}</span>
          </div>
          <div class="flex justify-between gap-2">
            <span class="text-gray-600">Recipient:</span>
            <span class="truncate font-medium"
              >{recipientName || recipientPhone}</span
            >
          </div>
          <div class="flex justify-between gap-2">
            <span class="text-gray-600">Amount:</span>
            <span class="font-medium"
              >{formatCurrencyAmount(
                parseFloat(localAmount),
                userCurrency as AfricanCurrency,
              )}</span
            >
          </div>
          <div class="flex justify-between gap-2">
            <span class="text-gray-600">Fee (0.5%):</span>
            <span class="font-medium"
              >{formatCurrencyAmount(
                calculateFee(parseFloat(localAmount)),
                userCurrency as AfricanCurrency,
              )}</span
            >
          </div>
        </div>
      </div>

      <div class="flex flex-col gap-3 sm:flex-row sm:gap-4">
        <button
          onclick={() => goto("/users/dashboard")}
          class="flex-1 rounded-lg bg-gray-100 py-2.5 text-sm font-semibold text-gray-700 transition-colors hover:bg-gray-200 sm:py-3 sm:text-base"
        >
          Back to Dashboard
        </button>
        <button
          onclick={() => {
            currentStep = "amount";
            localAmount = "";
            recipientPhone = "";
            recipientName = "";
            error = "";
          }}
          class="flex-1 rounded-lg bg-gray-900 py-2.5 text-sm font-semibold text-white transition-colors hover:bg-gray-800 sm:py-3 sm:text-base"
        >
          Send Another
        </button>
      </div>
    </div>
  {/if}
</div>
