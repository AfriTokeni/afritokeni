<script lang="ts">
import { ChevronDown, ChevronUp, HelpCircle, MessageCircle } from '@lucide/svelte';
interface Props { expanded: boolean; onToggle: () => void; }
let { expanded, onToggle }: Props = $props();
</script>
<div class="bg-gray-50 rounded-lg overflow-hidden mb-2 sm:mb-3">
<button onclick={onToggle} class="w-full flex items-center justify-between px-3 py-3 sm:px-4 sm:py-3.5 md:px-5 md:py-4 hover:bg-gray-100 transition-colors duration-200">
<div class="flex items-center space-x-2 sm:space-x-3"><HelpCircle class="w-4 h-4 sm:w-5 sm:h-5 md:w-6 md:h-6 text-gray-600 shrink-0" /><span class="text-sm sm:text-base md:text-lg text-gray-900">Help & Support</span></div>
{#if expanded}<ChevronUp class="w-4 h-4 sm:w-5 sm:h-5 text-gray-400 shrink-0" />{:else}<ChevronDown class="w-4 h-4 sm:w-5 sm:h-5 text-gray-400 shrink-0" />{/if}
</button>
{#if expanded}
<div class="px-3 pb-3 sm:px-4 sm:pb-4 md:px-5 md:pb-5 border-t border-gray-200">
<div class="pt-3 sm:pt-4 space-y-3 sm:space-y-4">
<button class="w-full flex items-center justify-between p-2.5 sm:p-3 bg-white rounded-lg hover:bg-gray-50 transition-colors"><div class="flex items-center space-x-2 sm:space-x-3"><MessageCircle class="w-4 h-4 sm:w-5 sm:h-5 text-gray-600 shrink-0" /><span class="text-xs sm:text-sm md:text-base text-gray-900">Contact Support</span></div></button>
<div class="bg-white p-2.5 sm:p-3 rounded-lg"><p class="text-xs sm:text-sm text-gray-600 mb-2"><strong>Email:</strong> support@afritokeni.com</p><p class="text-xs sm:text-sm text-gray-600"><strong>Phone:</strong> +256 700 000 000</p></div>
</div>
</div>
{/if}
</div>
