<script lang="ts">
	import { Smartphone, Send, Globe } from '@lucide/svelte';

	interface Message {
		type: 'sent' | 'received';
		text: string;
		timestamp: string;
	}

	let phoneNumber = '+256 700 123 456';
	let inputCommand = $state('');
	let messages = $state<Message[]>([
		{
			type: 'received',
			text: '🌍 USSD Demo Mode\n\nWelcome to AfriTokeni!\n\nDial *384*22948# to start\n\n🔐 Demo PIN: 1234',
			timestamp: new Date().toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' })
		}
	]);

	let messagesContainer: HTMLDivElement | undefined;

	// Demo responses for common commands
	const demoResponses: Record<string, string> = {
		'*384*22948#': 'Welcome to AfriTokeni\n\n1. Local Currency\n2. Bitcoin\n3. ckUSD\n4. DAO Governance\n5. Profile\n6. Help',
		'1': 'Local Currency Menu\n\n1. Send Money\n2. Check Balance\n3. Deposit\n4. Withdraw\n5. Transactions\n6. Find Agent\n\n0. Back',
		'2': 'Bitcoin Menu\n\n1. Check Balance\n2. Bitcoin Rate\n3. Buy Bitcoin\n4. Sell Bitcoin\n5. Send Bitcoin\n\n0. Back',
		'3': 'ckUSD Menu\n\n1. Check Balance\n2. ckUSD Rate\n3. Buy ckUSD\n4. Sell ckUSD\n5. Send ckUSD\n\n0. Back',
		'4': 'DAO Governance\n\n1. View Proposals\n2. Vote\n3. Check AFRI Balance\n4. Staking\n\n0. Back',
		'5': 'Profile\n\n1. View Profile\n2. Update Phone\n3. Change PIN\n4. KYC Status\n\n0. Back',
		'6': 'Help & Support\n\nUSSD: *384*22948#\nSMS: Text HELP to 22948\nWeb: afritokeni.com\n\nSupport: +256 700 000 000',
		'0': 'Welcome to AfriTokeni\n\n1. Local Currency\n2. Bitcoin\n3. ckUSD\n4. DAO Governance\n5. Profile\n6. Help'
	};

	function processCommand(cmd: string): string {
		const trimmedCmd = cmd.trim();
		
		// Check for demo responses
		if (demoResponses[trimmedCmd]) {
			return demoResponses[trimmedCmd];
		}

		// Default response for unknown commands
		return '❌ Invalid option\n\nPlease try again or dial *384*22948# to restart';
	}

	function handleSendMessage() {
		if (!inputCommand.trim()) return;

		const now = new Date();
		const timestamp = now.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' });

		// Add sent message
		messages = [...messages, {
			type: 'sent',
			text: inputCommand,
			timestamp
		}];

		// Process command and add response
		const response = processCommand(inputCommand);
		messages = [...messages, {
			type: 'received',
			text: response,
			timestamp: new Date().toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' })
		}];

		inputCommand = '';

		// Scroll to bottom
		setTimeout(() => {
			if (messagesContainer) {
				messagesContainer.scrollTop = messagesContainer.scrollHeight;
			}
		}, 100);
	}

	function handleKeyPress(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			handleSendMessage();
		}
	}

	function quickDial() {
		inputCommand = '*384*22948#';
		setTimeout(() => handleSendMessage(), 100);
	}
</script>

<svelte:head>
	<title>Try USSD Banking - AfriTokeni</title>
	<meta name="description" content="Experience how AfriTokeni works on any phone - no internet required. Try the USSD demo!" />
</svelte:head>

<div class="min-h-screen bg-gray-50">
	<!-- Hero Section with Gradient -->
	<div class="bg-linear-to-br from-green-600 to-teal-600 text-white py-12 sm:py-16 lg:py-20">
		<div class="max-w-6xl mx-auto px-4 sm:px-6 text-center">
			<div class="flex flex-col sm:flex-row items-center justify-center gap-2 sm:gap-3 mb-4 sm:mb-6">
				<div class="inline-flex items-center gap-2 bg-white/20 backdrop-blur-sm text-white px-3 sm:px-4 py-2 rounded-full text-xs sm:text-sm font-semibold">
					<Globe class="w-3 h-3 sm:w-4 sm:h-4" />
					🌍 Demo Mode - Works Worldwide
				</div>
				<div class="inline-flex items-center gap-2 bg-white/20 backdrop-blur-sm text-white px-3 sm:px-4 py-2 rounded-full text-xs sm:text-sm font-semibold">
					<Smartphone class="w-3 h-3 sm:w-4 sm:h-4" />
					🇺🇬 Real USSD: Uganda Only
				</div>
			</div>
			<h1 class="text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold mb-4 sm:mb-6">
				Try USSD Banking
			</h1>
			<p class="text-lg sm:text-xl lg:text-2xl opacity-90 max-w-2xl mx-auto">
				Experience how AfriTokeni works on any phone - no internet required. Try the commands below!
			</p>
		</div>
	</div>

	<div class="py-8 sm:py-10 lg:py-12">
		<div class="max-w-6xl mx-auto px-4 sm:px-6">
			<!-- Uganda Box -->
			<div class="mb-8 bg-red-50 border-2 border-red-300 rounded-lg p-4 max-w-2xl mx-auto">
				<h3 class="font-bold text-red-900 mb-2 text-sm sm:text-base">🇺🇬 REAL USSD: UGANDA ONLY</h3>
				<p class="text-xs sm:text-sm text-red-800 mb-2">
					<strong>Uganda:</strong> Dial <code class="bg-red-100 px-1.5 py-0.5 rounded font-mono text-xs">*284*78909#</code> for real USSD!
				</p>
				<p class="text-xs text-red-700">
					<strong>Other countries:</strong> Coming soon! Use playground to test.
				</p>
			</div>

			<!-- Phone Simulator - Centered -->
			<div class="max-w-sm sm:max-w-md mx-auto">
				<div class="bg-black rounded-[2rem] sm:rounded-[3rem] shadow-2xl overflow-hidden p-2 sm:p-3">
					<!-- Phone Notch -->
					<div class="bg-gray-900 rounded-[1.5rem] sm:rounded-[2.5rem] overflow-hidden">
						<!-- Status Bar -->
						<div class="bg-gray-900 px-4 sm:px-6 py-2 flex items-center justify-between text-white text-xs">
							<span>9:21</span>
							<div class="flex items-center gap-1">
								<div class="w-3 h-2 sm:w-4 sm:h-3 border border-white rounded-sm"></div>
								<div class="w-2 h-2 sm:w-3 sm:h-3 bg-white rounded-full"></div>
								<div class="flex gap-0.5">
									<div class="w-0.5 h-2 sm:h-3 bg-white"></div>
									<div class="w-0.5 h-2 sm:h-3 bg-white"></div>
									<div class="w-0.5 h-2 sm:h-3 bg-white"></div>
									<div class="w-0.5 h-2 sm:h-3 bg-white"></div>
								</div>
							</div>
						</div>
						<!-- USSD Header -->
						<div class="bg-gray-800 text-white px-4 sm:px-6 py-2 sm:py-3 flex items-center justify-between border-b border-gray-700">
							<div class="flex items-center gap-2">
								<Smartphone class="w-3 h-3 sm:w-4 sm:h-4" />
								<span class="font-semibold text-xs sm:text-sm">*384*22948#</span>
							</div>
							<span class="text-xs text-gray-400 hidden sm:inline">{phoneNumber}</span>
						</div>

						<!-- Messages -->
						<div bind:this={messagesContainer} class="h-[400px] sm:h-[500px] lg:h-[600px] overflow-y-auto bg-gray-50 p-3 sm:p-4 space-y-3 sm:space-y-4 scroll-smooth">
							{#each messages as msg, idx (idx)}
								<div class="flex {msg.type === 'sent' ? 'justify-end' : 'justify-start'}">
									<div class="max-w-[280px] sm:max-w-xs {msg.type === 'sent' ? 'bg-blue-600 text-white' : 'bg-white text-gray-900'} rounded-2xl px-3 sm:px-4 py-2 sm:py-3 shadow-sm">
										<p class="text-xs sm:text-sm whitespace-pre-line">{msg.text}</p>
										<p class="text-xs mt-1 {msg.type === 'sent' ? 'text-blue-200' : 'text-gray-500'}">
											{msg.timestamp}
										</p>
									</div>
								</div>
							{/each}
						</div>

						<!-- Input -->
						<div class="bg-white border-t border-gray-200 p-3 sm:p-4">
							<!-- Quick Start Button -->
							<button
								onclick={quickDial}
								class="w-full mb-2 sm:mb-3 bg-green-500 hover:bg-green-600 text-white font-bold py-2.5 sm:py-3 rounded-xl transition-colors flex items-center justify-center gap-2 text-sm sm:text-base"
							>
								<Smartphone class="w-4 h-4 sm:w-5 sm:h-5" />
								Dial *384*22948#
							</button>
							
							<div class="flex gap-2">
								<input
									type="tel"
									inputmode="tel"
									bind:value={inputCommand}
									onkeypress={handleKeyPress}
									placeholder="Type USSD command..."
									class="flex-1 px-3 sm:px-4 py-2.5 sm:py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm sm:text-base"
								/>
								<button
									onclick={handleSendMessage}
									class="bg-blue-600 text-white px-4 sm:px-6 py-2.5 sm:py-3 rounded-xl hover:bg-blue-700 transition-colors"
								>
									<Send class="w-4 h-4 sm:w-5 sm:h-5" />
								</button>
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- CTA Section -->
			<div class="mt-8 sm:mt-10 lg:mt-12 bg-gradient-to-r from-green-600 to-teal-600 rounded-xl p-6 sm:p-8 lg:p-12 text-center text-white">
				<h2 class="text-xl sm:text-2xl lg:text-3xl font-bold mb-3 sm:mb-4">Ready to Start Banking?</h2>
				<p class="text-base sm:text-lg lg:text-xl mb-4 sm:mb-6 opacity-90">
					Experience the future of mobile banking with instant, fee-free transfers
				</p>
				<div class="flex flex-col sm:flex-row gap-3 sm:gap-4 justify-center">
					<a
						href="/how-it-works"
						class="bg-white text-green-600 px-6 sm:px-8 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-medium hover:bg-neutral-100 transition-colors"
					>
						Learn How It Works
					</a>
					<a
						href="/become-agent"
						class="bg-white text-green-600 px-6 sm:px-8 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-medium hover:bg-neutral-100 transition-colors"
					>
						Become an Agent
					</a>
				</div>
			</div>
		</div>
	</div>
</div>
