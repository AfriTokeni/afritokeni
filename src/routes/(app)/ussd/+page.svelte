<script lang="ts">
  import { Globe, Send, Smartphone } from "@lucide/svelte";
  import { onMount } from "svelte";

  interface Message {
    type: "sent" | "received";
    text: string;
    timestamp: string;
  }

  let phoneNumber = "+256 700 123 456";
  let sessionId = `playground_session_${Date.now()}`;
  let inputCommand = $state("");
  let isInitialized = $state(false);
  let messages = $state<Message[]>([
    {
      type: "received",
      text: "🌍 USSD Demo Mode\n\nWelcome to AfriTokeni!\n\nDial *384*22948# to start\n\n🔐 Demo PIN: 1234",
      timestamp: new Date().toLocaleTimeString("en-US", {
        hour: "2-digit",
        minute: "2-digit",
      }),
    },
  ]);

  let messagesContainer: HTMLDivElement | undefined;

  onMount(() => {
    console.log("🎭 Playground: Using in-memory mock data (no backend calls)");
    isInitialized = true;
  });

  async function processCommand(cmd: string): Promise<string> {
    const trimmedCmd = cmd.trim();

    if (!isInitialized) {
      return "⏳ Initializing... Please wait.";
    }

    // Handle USSD dial code or menu navigation
    const ussdText = trimmedCmd;

    try {
      // Call Juno satellite function (Rust backend)
      console.log(
        `📱 USSD Playground: sessionId="${sessionId}", text="${ussdText}"`,
      );

      const satelliteId = import.meta.env.VITE_SATELLITE_ID || 'dkk74-oyaaa-aaaal-askxq-cai';
      const response = await fetch(`https://${satelliteId}.icp0.io/ussd`, {
        method: "POST",
        headers: {
          "Content-Type": "application/x-www-form-urlencoded",
        },
        body: new URLSearchParams({
          sessionId,
          phoneNumber,
          text: ussdText,
        }),
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const result = await response.text();

      // Clean up response (remove CON/END prefixes)
      return result.replace(/^(CON |END )/, "");
    } catch (error) {
      console.error("Failed to process USSD command:", error);
      return "❌ Error processing command\n\nPlease try again or dial *384*22948# to restart";
    }
  }

  async function handleSendMessage() {
    if (!inputCommand.trim()) return;

    const now = new Date();
    const timestamp = now.toLocaleTimeString("en-US", {
      hour: "2-digit",
      minute: "2-digit",
    });

    // Add sent message
    messages = [
      ...messages,
      {
        type: "sent",
        text: inputCommand,
        timestamp,
      },
    ];

    // Process command and add response
    const response = await processCommand(inputCommand);
    messages = [
      ...messages,
      {
        type: "received",
        text: response,
        timestamp: new Date().toLocaleTimeString("en-US", {
          hour: "2-digit",
          minute: "2-digit",
        }),
      },
    ];

    inputCommand = "";

    // Scroll to bottom
    setTimeout(() => {
      if (messagesContainer) {
        messagesContainer.scrollTop = messagesContainer.scrollHeight;
      }
    }, 100);
  }

  function handleKeyPress(e: KeyboardEvent) {
    if (e.key === "Enter") {
      handleSendMessage();
    }
  }

  function quickDial() {
    inputCommand = "*384*22948#";
    setTimeout(() => handleSendMessage(), 100);
  }
</script>

<svelte:head>
  <title>Try USSD Banking - AfriTokeni</title>
  <meta
    name="description"
    content="Experience how AfriTokeni works on any phone - no internet required. Try the USSD demo!"
  />
</svelte:head>

<div class="min-h-screen bg-gray-50">
  <!-- Hero Section with Gradient -->
  <div
    class="bg-linear-to-br from-green-600 to-teal-600 py-12 text-white sm:py-16 lg:py-20"
  >
    <div class="mx-auto max-w-6xl px-4 text-center sm:px-6">
      <div
        class="mb-4 flex flex-col items-center justify-center gap-2 sm:mb-6 sm:flex-row sm:gap-3"
      >
        <div
          class="inline-flex items-center gap-2 rounded-full bg-white/20 px-3 py-2 text-xs font-semibold text-white backdrop-blur-sm sm:px-4 sm:text-sm"
        >
          <Globe class="h-3 w-3 sm:h-4 sm:w-4" />
          🌍 Demo Mode - Works Worldwide
        </div>
        <div
          class="inline-flex items-center gap-2 rounded-full bg-white/20 px-3 py-2 text-xs font-semibold text-white backdrop-blur-sm sm:px-4 sm:text-sm"
        >
          <Smartphone class="h-3 w-3 sm:h-4 sm:w-4" />
          🇺🇬 Real USSD: Uganda Only
        </div>
      </div>
      <h1
        class="mb-4 text-3xl font-bold sm:mb-6 sm:text-4xl md:text-5xl lg:text-6xl"
      >
        Try USSD Banking
      </h1>
      <p class="mx-auto max-w-2xl text-lg opacity-90 sm:text-xl lg:text-2xl">
        Experience how AfriTokeni works on any phone - no internet required. Try
        the commands below!
      </p>
    </div>
  </div>

  <div class="py-8 sm:py-10 lg:py-12">
    <div class="mx-auto max-w-6xl px-4 sm:px-6">
      <!-- Uganda Box -->
      <div
        class="mx-auto mb-8 max-w-2xl rounded-lg border-2 border-red-300 bg-red-50 p-4"
      >
        <h3 class="mb-2 text-sm font-bold text-red-900 sm:text-base">
          🇺🇬 REAL USSD: UGANDA ONLY
        </h3>
        <p class="mb-2 text-xs text-red-800 sm:text-sm">
          <strong>Uganda:</strong> Dial
          <code class="rounded bg-red-100 px-1.5 py-0.5 font-mono text-xs"
            >*284*78909#</code
          > for real USSD!
        </p>
        <p class="text-xs text-red-700">
          <strong>Other countries:</strong> Coming soon! Use playground to test.
        </p>
      </div>

      <!-- Phone Simulator - Centered -->
      <div class="mx-auto max-w-sm sm:max-w-md">
        <div
          class="overflow-hidden rounded-4xl bg-black p-2 shadow-2xl sm:rounded-[3rem] sm:p-3"
        >
          <!-- Phone Notch -->
          <div
            class="overflow-hidden rounded-3xl bg-gray-900 sm:rounded-[2.5rem]"
          >
            <!-- Status Bar -->
            <div
              class="flex items-center justify-between bg-gray-900 px-4 py-2 text-xs text-white sm:px-6"
            >
              <span>9:21</span>
              <div class="flex items-center gap-1">
                <div
                  class="h-2 w-3 rounded-sm border border-white sm:h-3 sm:w-4"
                ></div>
                <div class="h-2 w-2 rounded-full bg-white sm:h-3 sm:w-3"></div>
                <div class="flex gap-0.5">
                  <div class="h-2 w-0.5 bg-white sm:h-3"></div>
                  <div class="h-2 w-0.5 bg-white sm:h-3"></div>
                  <div class="h-2 w-0.5 bg-white sm:h-3"></div>
                  <div class="h-2 w-0.5 bg-white sm:h-3"></div>
                </div>
              </div>
            </div>
            <!-- USSD Header -->
            <div
              class="flex items-center justify-between border-b border-gray-700 bg-gray-800 px-4 py-2 text-white sm:px-6 sm:py-3"
            >
              <div class="flex items-center gap-2">
                <Smartphone class="h-3 w-3 sm:h-4 sm:w-4" />
                <span class="text-xs font-semibold sm:text-sm">*384*22948#</span
                >
              </div>
              <span class="hidden text-xs text-gray-400 sm:inline"
                >{phoneNumber}</span
              >
            </div>

            <!-- Messages -->
            <div
              bind:this={messagesContainer}
              class="h-[400px] space-y-3 overflow-y-auto scroll-smooth bg-gray-50 p-3 sm:h-[500px] sm:space-y-4 sm:p-4 lg:h-[600px]"
            >
              {#each messages as msg, idx (idx)}
                <div
                  class="flex {msg.type === 'sent'
                    ? 'justify-end'
                    : 'justify-start'}"
                >
                  <div
                    class="max-w-[280px] sm:max-w-xs {msg.type === 'sent'
                      ? 'bg-blue-600 text-white'
                      : 'bg-white text-gray-900'} rounded-2xl px-3 py-2 shadow-sm sm:px-4 sm:py-3"
                  >
                    <p class="text-xs whitespace-pre-line sm:text-sm">
                      {msg.text}
                    </p>
                    <p
                      class="mt-1 text-xs {msg.type === 'sent'
                        ? 'text-blue-200'
                        : 'text-gray-500'}"
                    >
                      {msg.timestamp}
                    </p>
                  </div>
                </div>
              {/each}
            </div>

            <!-- Input -->
            <div class="border-t border-gray-200 bg-white p-3 sm:p-4">
              <!-- Quick Start Button -->
              <button
                onclick={quickDial}
                class="mb-2 flex w-full items-center justify-center gap-2 rounded-xl bg-green-500 py-2.5 text-sm font-bold text-white transition-colors hover:bg-green-600 sm:mb-3 sm:py-3 sm:text-base"
              >
                <Smartphone class="h-4 w-4 sm:h-5 sm:w-5" />
                Dial *384*22948#
              </button>

              <div class="flex gap-2">
                <input
                  type="tel"
                  inputmode="tel"
                  bind:value={inputCommand}
                  onkeypress={handleKeyPress}
                  placeholder="Type USSD command..."
                  class="flex-1 rounded-xl border border-gray-300 px-3 py-2.5 text-sm focus:border-transparent focus:ring-2 focus:ring-blue-500 sm:px-4 sm:py-3 sm:text-base"
                />
                <button
                  onclick={handleSendMessage}
                  class="rounded-xl bg-blue-600 px-4 py-2.5 text-white transition-colors hover:bg-blue-700 sm:px-6 sm:py-3"
                >
                  <Send class="h-4 w-4 sm:h-5 sm:w-5" />
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- CTA Section -->
      <div
        class="mt-8 rounded-xl bg-linear-to-r from-green-600 to-teal-600 p-6 text-center text-white sm:mt-10 sm:p-8 lg:mt-12 lg:p-12"
      >
        <h2 class="mb-3 text-xl font-bold sm:mb-4 sm:text-2xl lg:text-3xl">
          Ready to Start Banking?
        </h2>
        <p class="mb-4 text-base opacity-90 sm:mb-6 sm:text-lg lg:text-xl">
          Experience the future of mobile banking with instant, fee-free
          transfers
        </p>
        <div class="flex flex-col justify-center gap-3 sm:flex-row sm:gap-4">
          <a
            href="/how-it-works"
            class="rounded-lg bg-white px-6 py-2.5 text-sm font-medium text-green-600 transition-colors hover:bg-neutral-100 sm:px-8 sm:py-3 sm:text-base"
          >
            Learn How It Works
          </a>
          <a
            href="/become-agent"
            class="rounded-lg bg-white px-6 py-2.5 text-sm font-medium text-green-600 transition-colors hover:bg-neutral-100 sm:px-8 sm:py-3 sm:text-base"
          >
            Become an Agent
          </a>
        </div>
      </div>
    </div>
  </div>
</div>
