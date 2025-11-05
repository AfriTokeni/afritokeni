<script lang="ts">
  import {
    Card,
    Button,
    Badge,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
    Select,
  } from "flowbite-svelte";
  import { TrendingUp, TrendingDown, BarChart3 } from 'lucide-svelte';

  let timeRange = $state("30d");

  // Mock revenue data
  let revenueStats = $state({
    totalRevenue: 45678.5,
    depositCommissions: 28450.3,
    withdrawalFees: 12340.2,
    exchangeSpread: 4888.0,
    growth: 12.5,
    previousPeriod: 40678.2,
  });

  let revenueBreakdown = $state([
    {
      source: "Deposit Commissions (5%)",
      amount: 28450.3,
      percentage: 62.3,
      transactions: 1234,
      color: "blue",
    },
    {
      source: "Withdrawal Fees",
      amount: 12340.2,
      percentage: 27.0,
      transactions: 856,
      color: "green",
    },
    {
      source: "Exchange Spread",
      amount: 4888.0,
      percentage: 10.7,
      transactions: 432,
      color: "purple",
    },
  ]);

  let topRevenueAgents = $state([
    {
      name: "Agent Lagos Central",
      revenue: 8500,
      transactions: 234,
      commission: 850,
    },
    {
      name: "Agent Nairobi East",
      revenue: 6200,
      transactions: 198,
      commission: 620,
    },
    {
      name: "Agent Accra West",
      revenue: 5800,
      transactions: 176,
      commission: 580,
    },
    {
      name: "Agent Kampala North",
      revenue: 4600,
      transactions: 145,
      commission: 460,
    },
  ]);

  let monthlyRevenue = $state([
    { month: "Oct 2024", revenue: 42340, growth: 8.2 },
    { month: "Sep 2024", revenue: 39150, growth: 12.5 },
    { month: "Aug 2024", revenue: 34820, growth: 15.3 },
    { month: "Jul 2024", revenue: 30200, growth: 9.8 },
    { month: "Jun 2024", revenue: 27500, growth: 11.2 },
  ]);
</script>

<div class="space-y-6">
  <!-- Page Header -->
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-bold text-gray-900">Revenue Analytics</h1>
      <p class="mt-1 text-sm text-gray-500">
        Track platform earnings and revenue sources
      </p>
    </div>
    <div class="flex gap-2">
      <Select bind:value={timeRange} size="sm">
        <option value="7d">Last 7 days</option>
        <option value="30d">Last 30 days</option>
        <option value="90d">Last 90 days</option>
        <option value="1y">Last year</option>
      </Select>
      <Button size="sm">Export Report</Button>
    </div>
  </div>

  <!-- Revenue Overview -->
  <div class="grid grid-cols-1 gap-4 md:grid-cols-4">
    <Card class="col-span-2">
      <div>
        <p class="text-sm font-medium text-gray-500">Total Revenue</p>
        <p class="mt-2 text-4xl font-bold text-gray-900">
          ${revenueStats.totalRevenue.toLocaleString()}
        </p>
        <div class="mt-3 flex items-center text-sm">
          <TrendingUp class="h-4 w-4 text-green-500" />
          <span class="ml-1 font-semibold text-green-600"
            >{revenueStats.growth}%</span
          >
          <span class="ml-2 text-gray-500">vs previous period</span>
        </div>
        <div class="mt-4 text-sm text-gray-500">
          Previous: ${revenueStats.previousPeriod.toLocaleString()}
        </div>
      </div>
    </Card>

    <Card>
      <p class="text-sm font-medium text-gray-500">Deposit Commissions</p>
      <p class="mt-2 text-2xl font-bold text-blue-600">
        ${revenueStats.depositCommissions.toLocaleString()}
      </p>
      <p class="mt-2 text-xs text-gray-500">62.3% of total</p>
    </Card>

    <Card>
      <p class="text-sm font-medium text-gray-500">Withdrawal Fees</p>
      <p class="mt-2 text-2xl font-bold text-green-600">
        ${revenueStats.withdrawalFees.toLocaleString()}
      </p>
      <p class="mt-2 text-xs text-gray-500">27.0% of total</p>
    </Card>
  </div>

  <!-- Revenue Breakdown -->
  <Card>
    <h3 class="mb-4 text-lg font-semibold text-gray-900">Revenue Sources</h3>
    <div class="space-y-4">
      {#each revenueBreakdown as item}
        <div class="space-y-2">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-2">
              <div class="h-3 w-3 rounded-full bg-{item.color}-500"></div>
              <span class="font-medium text-gray-900">{item.source}</span>
            </div>
            <div class="text-right">
              <p class="font-semibold text-gray-900">
                ${item.amount.toLocaleString()}
              </p>
              <p class="text-sm text-gray-500">
                {item.transactions} transactions
              </p>
            </div>
          </div>
          <div class="h-2 w-full rounded-full bg-gray-200">
            <div
              class="h-2 rounded-full bg-{item.color}-500"
              style="width: {item.percentage}%"
            ></div>
          </div>
        </div>
      {/each}
    </div>
  </Card>

  <!-- Charts Placeholder -->
  <div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
    <Card>
      <h3 class="mb-4 text-lg font-semibold text-gray-900">Revenue Trend</h3>
      <div class="flex h-64 items-center justify-center rounded-lg bg-gray-100">
        <div class="text-center">
          <BarChart3 class="mx-auto h-12 w-12 text-gray-400" />
          <p class="mt-2 text-sm text-gray-500">
            Line chart showing revenue over time
          </p>
          <p class="text-xs text-gray-400">Will use Chart.js or similar</p>
        </div>
      </div>
    </Card>

    <Card>
      <h3 class="mb-4 text-lg font-semibold text-gray-900">
        Revenue Distribution
      </h3>
      <div class="flex h-64 items-center justify-center rounded-lg bg-gray-100">
        <div class="text-center">
          <BarChart3 class="mx-auto h-12 w-12 text-gray-400" />
          <p class="mt-2 text-sm text-gray-500">
            Pie chart showing revenue sources
          </p>
          <p class="text-xs text-gray-400">Will use Chart.js or similar</p>
        </div>
      </div>
    </Card>
  </div>

  <!-- Top Revenue Agents -->
  <Card>
    <div class="mb-4 flex items-center justify-between">
      <h3 class="text-lg font-semibold text-gray-900">
        Top Revenue Generating Agents
      </h3>
      <Button href="/admin/agents" size="xs" color="light"
        >View All Agents</Button
      >
    </div>
    <Table>
      <TableHead>
        <TableHeadCell>Agent</TableHeadCell>
        <TableHeadCell>Revenue Generated</TableHeadCell>
        <TableHeadCell>Transactions</TableHeadCell>
        <TableHeadCell>Commission Owed</TableHeadCell>
      </TableHead>
      <TableBody>
        {#each topRevenueAgents as agent}
          <TableBodyRow>
            <TableBodyCell class="font-medium">{agent.name}</TableBodyCell>
            <TableBodyCell class="font-semibold text-green-600"
              >${agent.revenue.toLocaleString()}</TableBodyCell
            >
            <TableBodyCell>{agent.transactions}</TableBodyCell>
            <TableBodyCell class="font-semibold text-blue-600"
              >${agent.commission.toLocaleString()}</TableBodyCell
            >
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
  </Card>

  <!-- Monthly Revenue History -->
  <Card>
    <h3 class="mb-4 text-lg font-semibold text-gray-900">
      Monthly Revenue History
    </h3>
    <Table>
      <TableHead>
        <TableHeadCell>Month</TableHeadCell>
        <TableHeadCell>Revenue</TableHeadCell>
        <TableHeadCell>Growth</TableHeadCell>
      </TableHead>
      <TableBody>
        {#each monthlyRevenue as month}
          <TableBodyRow>
            <TableBodyCell class="font-medium">{month.month}</TableBodyCell>
            <TableBodyCell class="font-semibold"
              >${month.revenue.toLocaleString()}</TableBodyCell
            >
            <TableBodyCell>
              <div class="flex items-center gap-1">
                <TrendingUp class="h-4 w-4 text-green-500" />
                <span class="font-semibold text-green-600">{month.growth}%</span
                >
              </div>
            </TableBodyCell>
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
  </Card>
</div>
