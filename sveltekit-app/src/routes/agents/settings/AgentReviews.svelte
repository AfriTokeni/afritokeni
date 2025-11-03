<script lang="ts">
	import { ChevronDown, Star, Filter, TrendingUp } from '@lucide/svelte';
	import { onMount } from 'svelte';
	import { demoMode } from '$lib/stores/demoMode';
	import { principalId } from '$lib/stores/auth';
	import { listDocs } from '@junobuild/core';

	interface Props {
		agentData: any;
		expanded: boolean;
		onToggle: () => void;
	}

	let { agentData, expanded, onToggle }: Props = $props();

	let reviews = $state<any[]>([]);
	let selectedFilter = $state<number | null>(null);
	let isLoading = $state(false);

	// Load reviews when component mounts or demo mode changes
	$effect(() => {
		if (expanded) {
			loadReviews($demoMode, $principalId);
		}
	});

	async function loadReviews(isDemoMode: boolean, agentPrincipalId: string | null) {
		isLoading = true;

		if (isDemoMode) {
			// Demo data
			reviews = [
				{
					id: '1',
					customerName: 'John Kamau',
					rating: 5,
					comment: 'Excellent service! Very professional and quick transaction.',
					date: '2024-01-15',
					transactionType: 'Cash Deposit'
				},
				{
					id: '2',
					customerName: 'Mary Achieng',
					rating: 4,
					comment: 'Good agent, but had to wait a bit. Overall satisfied.',
					date: '2024-01-14',
					transactionType: 'Withdrawal'
				},
				{
					id: '3',
					customerName: 'David Omondi',
					rating: 5,
					comment: 'Best agent in the area! Always available and helpful.',
					date: '2024-01-13',
					transactionType: 'Bitcoin Exchange'
				},
				{
					id: '4',
					customerName: 'Sarah Wanjiku',
					rating: 3,
					comment: 'Service was okay, but location was hard to find.',
					date: '2024-01-12',
					transactionType: 'Cash Deposit'
				},
				{
					id: '5',
					customerName: 'Peter Mutua',
					rating: 5,
					comment: 'Very trustworthy and efficient. Highly recommend!',
					date: '2024-01-11',
					transactionType: 'Withdrawal'
				}
			];
			isLoading = false;
			return;
		}

		if (!agentPrincipalId) {
			reviews = [];
			isLoading = false;
			return;
		}

		try {
			// Load reviews from Juno
			const { items } = await listDocs({
				collection: 'agent_reviews',
				filter: {
					matcher: {
						key: agentPrincipalId
					}
				}
			});

			reviews = items.map(item => ({
				id: item.key,
				...item.data
			}));
		} catch (error) {
			console.error('Failed to load reviews:', error);
			reviews = [];
		} finally {
			isLoading = false;
		}
	}

	// Calculate rating distribution
	const ratingDistribution = $derived(() => {
		const dist = { 5: 0, 4: 0, 3: 0, 2: 0, 1: 0 };
		reviews.forEach(r => dist[r.rating as keyof typeof dist]++);
		return dist;
	});

	const filteredReviews = $derived(
		selectedFilter ? reviews.filter(r => r.rating === selectedFilter) : reviews
	);

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}

	function getStarPercentage(stars: number): number {
		const total = reviews.length;
		return (ratingDistribution()[stars as keyof ReturnType<typeof ratingDistribution>] / total) * 100;
	}
</script>

<div class="bg-white border border-gray-200 rounded-xl overflow-hidden">
	<!-- Header -->
	<button
		onclick={onToggle}
		class="w-full px-4 sm:px-6 py-4 flex items-center justify-between hover:bg-gray-50 transition-colors"
	>
		<div class="flex items-center gap-3">
			<div class="p-2 bg-yellow-100 rounded-lg">
				<Star class="w-5 h-5 text-yellow-600" />
			</div>
			<div class="text-left">
				<h3 class="text-base sm:text-lg font-semibold text-gray-900">Reviews & Ratings</h3>
				<p class="text-xs sm:text-sm text-gray-600">
					{agentData.totalReviews} reviews • {agentData.rating.toFixed(1)} average
				</p>
			</div>
		</div>
		<ChevronDown
			class="w-5 h-5 text-gray-400 transition-transform {expanded ? 'rotate-180' : ''}"
		/>
	</button>

	<!-- Content -->
	{#if expanded}
		<div class="px-4 sm:px-6 pb-6 space-y-6">
			<!-- Rating Overview -->
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<!-- Average Rating -->
				<div class="bg-linear-to-br from-yellow-50 to-orange-50 rounded-xl p-6">
					<div class="text-center">
						<div class="text-5xl font-bold text-gray-900 mb-2">{agentData.rating.toFixed(1)}</div>
						<div class="flex items-center justify-center gap-1 mb-2">
							{#each Array(5) as _, i}
								<Star
									class="w-5 h-5 {i < Math.round(agentData.rating) ? 'text-yellow-500 fill-yellow-500' : 'text-gray-300'}"
								/>
							{/each}
						</div>
						<p class="text-sm text-gray-600">{agentData.totalReviews} total reviews</p>
					</div>
				</div>

				<!-- Rating Distribution -->
				<div class="space-y-2">
					{#each [5, 4, 3, 2, 1] as stars}
						<button
							onclick={() => selectedFilter = selectedFilter === stars ? null : stars}
							class="w-full flex items-center gap-3 hover:bg-gray-50 rounded-lg p-2 transition-colors {selectedFilter === stars ? 'bg-yellow-50' : ''}"
						>
							<div class="flex items-center gap-1 w-16">
								<span class="text-sm font-medium text-gray-700">{stars}</span>
								<Star class="w-4 h-4 text-yellow-500 fill-yellow-500" />
							</div>
							<div class="flex-1 bg-gray-200 rounded-full h-2">
								<div
									class="bg-yellow-500 h-2 rounded-full transition-all"
									style="width: {getStarPercentage(stars)}%"
								></div>
							</div>
							<span class="text-sm text-gray-600 w-12 text-right">
								{ratingDistribution()[stars as keyof ReturnType<typeof ratingDistribution>]}
							</span>
						</button>
					{/each}
				</div>
			</div>

			<!-- Filter Info -->
			{#if selectedFilter}
				<div class="flex items-center justify-between bg-yellow-50 border border-yellow-200 rounded-lg p-3">
					<div class="flex items-center gap-2">
						<Filter class="w-4 h-4 text-yellow-600" />
						<span class="text-sm font-medium text-yellow-900">
							Showing {selectedFilter}-star reviews
						</span>
					</div>
					<button
						onclick={() => selectedFilter = null}
						class="text-sm text-yellow-700 hover:text-yellow-900 font-medium"
					>
						Clear filter
					</button>
				</div>
			{/if}

			<!-- Reviews List with Scroll -->
			<div class="max-h-96 overflow-y-auto space-y-4 pr-2">
				{#if isLoading}
					<div class="text-center py-8 text-gray-500">
						<div class="w-8 h-8 border-4 border-gray-300 border-t-gray-600 rounded-full animate-spin mx-auto mb-3"></div>
						<p>Loading reviews...</p>
					</div>
				{:else if filteredReviews.length === 0}
					<div class="text-center py-8 text-gray-500">
						<Star class="w-12 h-12 mx-auto mb-3 text-gray-300" />
						<p>{selectedFilter ? 'No reviews match this filter' : 'No reviews yet'}</p>
					</div>
				{:else}
					{#each filteredReviews as review (review.id)}
						<div class="border border-gray-200 rounded-lg p-4 hover:border-gray-300 transition-colors">
							<div class="flex items-start justify-between mb-2">
								<div>
									<h4 class="font-semibold text-gray-900">{review.customerName}</h4>
									<p class="text-xs text-gray-500">{formatDate(review.date)} • {review.transactionType}</p>
								</div>
								<div class="flex items-center gap-1">
									{#each Array(5) as _, i}
										<Star
											class="w-4 h-4 {i < review.rating ? 'text-yellow-500 fill-yellow-500' : 'text-gray-300'}"
										/>
									{/each}
								</div>
							</div>
							<p class="text-sm text-gray-700">{review.comment}</p>
						</div>
					{/each}
				{/if}
			</div>

			<!-- Stats -->
			<div class="grid grid-cols-2 gap-4 pt-4 border-t border-gray-200">
				<div class="text-center">
					<div class="flex items-center justify-center gap-2 mb-1">
						<TrendingUp class="w-5 h-5 text-green-600" />
						<span class="text-2xl font-bold text-gray-900">
							{Math.round((ratingDistribution()[5] / reviews.length) * 100)}%
						</span>
					</div>
					<p class="text-sm text-gray-600">5-star reviews</p>
				</div>
				<div class="text-center">
					<div class="flex items-center justify-center gap-2 mb-1">
						<Star class="w-5 h-5 text-yellow-500 fill-yellow-500" />
						<span class="text-2xl font-bold text-gray-900">{agentData.rating.toFixed(2)}</span>
					</div>
					<p class="text-sm text-gray-600">Average rating</p>
				</div>
			</div>
		</div>
	{/if}
</div>
