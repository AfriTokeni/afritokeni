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
    Input,
    Select,
  } from "flowbite-svelte";
  import { Search, Filter, Download } from 'lucide-svelte';
  import { onMount } from "svelte";

  let searchQuery = $state("");
  let filterType = $state("all");
  let filterStatus = $state("all");

  // Mock transaction data
  let transactions = $state([
    {
      id: "TXN-12345",
      type: "Deposit",
      user: "John Doe",
      agent: "Agent Lagos",
      amount: 500,
      currency: "NGN",
      fee: 25,
      status: "completed",
      timestamp: "2024-11-05 14:30:22",
      canisterId: "453vh-eqaaa...",
    },
    {
      id: "TXN-12344",
      type: "Withdrawal",
      user: "Jane Smith",
      agent: "Agent Nairobi",
      amount: 300,
      currency: "KES",
      fee: 15,
      status: "pending",
      timestamp: "2024-11-05 14:28:15",
      canisterId: "422tt-jiaaa...",
    },
    {
      id: "TXN-12343",
      type: "Exchange",
      user: "Bob Johnson",
      agent: null,
      amount: 1000,
      currency: "GHS",
      fee: 10,
      status: "completed",
      timestamp: "2024-11-05 14:25:45",
      canisterId: "4tzyp-7aaaa...",
    },
    {
      id: "TXN-12342",
      type: "Deposit",
      user: "Alice Brown",
      agent: "Agent Accra",
      amount: 750,
      currency: "NGN",
      fee: 37.5,
      status: "completed",
      timestamp: "2024-11-05 14:20:10",
      canisterId: "453vh-eqaaa...",
    },
    {
      id: "TXN-12341",
      type: "Withdrawal",
      user: "Charlie Wilson",
      agent: "Agent Kampala",
      amount: 200,
      currency: "KES",
      fee: 10,
      status: "failed",
      timestamp: "2024-11-05 14:15:33",
      canisterId: "422tt-jiaaa...",
    },
    {
      id: "TXN-12340",
      type: "Exchange",
      user: "David Lee",
      agent: null,
      amount: 500,
      currency: "NGN",
      fee: 5,
      status: "completed",
      timestamp: "2024-11-05 14:10:05",
      canisterId: "4tzyp-7aaaa...",
    },
    {
      id: "TXN-12339",
      type: "Deposit",
      user: "Emma Davis",
      agent: "Agent Lagos",
      amount: 1200,
      currency: "NGN",
      fee: 60,
      status: "completed",
      timestamp: "2024-11-05 14:05:18",
      canisterId: "453vh-eqaaa...",
    },
    {
      id: "TXN-12338",
      type: "Withdrawal",
      user: "Frank Miller",
      agent: "Agent Nairobi",
      amount: 450,
      currency: "KES",
      fee: 22.5,
      status: "pending",
      timestamp: "2024-11-05 14:00:42",
      canisterId: "422tt-jiaaa...",
    },
  ]);

  let stats = $state({
    total: 3542,
    completed: 3201,
    pending: 12,
    failed: 5,
    totalVolume: 1245678,
  });

  // Simulate real-time updates
  onMount(() => {
    const interval = setInterval(() => {
      // Simulate new transaction
      const newTx = {
        id: `TXN-${Math.floor(Math.random() * 99999)}`,
        type: ["Deposit", "Withdrawal", "Exchange"][
          Math.floor(Math.random() * 3)
        ],
        user: ["John Doe", "Jane Smith", "Bob Johnson"][
          Math.floor(Math.random() * 3)
        ],
        agent: Math.random() > 0.5 ? "Agent Lagos" : null,
        amount: Math.floor(Math.random() * 1000) + 100,
        currency: ["NGN", "KES", "GHS"][Math.floor(Math.random() * 3)],
        fee: Math.floor(Math.random() * 50) + 5,
        status: "completed",
        timestamp: new Date().toISOString().replace("T", " ").substring(0, 19),
        canisterId: "453vh-eqaaa...",
      };
      transactions = [newTx, ...transactions].slice(0, 20);
      stats.total++;
      stats.completed++;
    }, 10000); // Every 10 seconds

    return () => clearInterval(interval);
  });

  function getStatusColor(status: string) {
    switch (status) {
      case "completed":
        return "green";
      case "pending":
        return "yellow";
      case "failed":
        return "red";
      default:
        return "gray";
    }
  }

  function getTypeColor(type: string) {
    switch (type) {
      case "Deposit":
        return "blue";
      case "Withdrawal":
        return "purple";
      case "Exchange":
        return "indigo";
      default:
        return "gray";
    }
  }

  $effect(() => {
    // Filter logic would go here
    console.log("Filters:", { searchQuery, filterType, filterStatus });
  });
</script>

<div class="space-y-6">
  <!-- Page Header -->
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-bold text-gray-900">Transaction Monitoring</h1>
      <p class="mt-1 text-sm text-gray-500">
        Real-time transaction feed and analytics
      </p>
    </div>
    <div class="flex gap-2">
      <Button size="sm" color="light">
        <Download class="mr-2 h-4 w-4" />
        Export
      </Button>
      <Button size="sm">Refresh</Button>
    </div>
  </div>

  <!-- Stats Overview -->
  <div class="grid grid-cols-1 gap-4 md:grid-cols-5">
    <Card>
      <p class="text-sm text-gray-500">Total Transactions</p>
      <p class="mt-2 text-2xl font-bold text-gray-900">
        {stats.total.toLocaleString()}
      </p>
    </Card>
    <Card>
      <p class="text-sm text-gray-500">Completed</p>
      <p class="mt-2 text-2xl font-bold text-green-600">
        {stats.completed.toLocaleString()}
      </p>
    </Card>
    <Card>
      <p class="text-sm text-gray-500">Pending</p>
      <p class="mt-2 text-2xl font-bold text-yellow-600">{stats.pending}</p>
    </Card>
    <Card>
      <p class="text-sm text-gray-500">Failed</p>
      <p class="mt-2 text-2xl font-bold text-red-600">{stats.failed}</p>
    </Card>
    <Card>
      <p class="text-sm text-gray-500">Total Volume</p>
      <p class="mt-2 text-2xl font-bold text-blue-600">
        ${stats.totalVolume.toLocaleString()}
      </p>
    </Card>
  </div>

  <!-- Filters -->
  <Card>
    <div class="flex flex-wrap gap-4">
      <div class="flex-1">
        <Input
          bind:value={searchQuery}
          placeholder="Search by ID, user, or agent..."
          size="sm"
        >
          {#snippet left()}
            <Search class="h-4 w-4" />
          {/snippet}
        </Input>
      </div>
      <Select bind:value={filterType} size="sm" class="w-48">
        <option value="all">All Types</option>
        <option value="deposit">Deposits</option>
        <option value="withdrawal">Withdrawals</option>
        <option value="exchange">Exchanges</option>
      </Select>
      <Select bind:value={filterStatus} size="sm" class="w-48">
        <option value="all">All Status</option>
        <option value="completed">Completed</option>
        <option value="pending">Pending</option>
        <option value="failed">Failed</option>
      </Select>
    </div>
  </Card>

  <!-- Transactions Table -->
  <Card>
    <div class="mb-4 flex items-center justify-between">
      <h3 class="text-lg font-semibold text-gray-900">Recent Transactions</h3>
      <Badge color="blue" large>
        <span class="animate-pulse">●</span>
        <span class="ml-1">Live</span>
      </Badge>
    </div>
    <div class="overflow-x-auto">
      <Table>
        <TableHead>
          <TableHeadCell>Transaction ID</TableHeadCell>
          <TableHeadCell>Type</TableHeadCell>
          <TableHeadCell>User</TableHeadCell>
          <TableHeadCell>Agent</TableHeadCell>
          <TableHeadCell>Amount</TableHeadCell>
          <TableHeadCell>Fee</TableHeadCell>
          <TableHeadCell>Status</TableHeadCell>
          <TableHeadCell>Timestamp</TableHeadCell>
          <TableHeadCell>Canister</TableHeadCell>
        </TableHead>
        <TableBody>
          {#each transactions as tx}
            <TableBodyRow>
              <TableBodyCell class="font-mono text-xs font-medium"
                >{tx.id}</TableBodyCell
              >
              <TableBodyCell>
                <Badge color={getTypeColor(tx.type)}>{tx.type}</Badge>
              </TableBodyCell>
              <TableBodyCell class="font-medium">{tx.user}</TableBodyCell>
              <TableBodyCell class="text-sm text-gray-500"
                >{tx.agent || "-"}</TableBodyCell
              >
              <TableBodyCell class="font-semibold"
                >{tx.amount} {tx.currency}</TableBodyCell
              >
              <TableBodyCell class="text-sm text-gray-600"
                >${tx.fee}</TableBodyCell
              >
              <TableBodyCell>
                <Badge color={getStatusColor(tx.status)}>{tx.status}</Badge>
              </TableBodyCell>
              <TableBodyCell class="text-sm text-gray-500"
                >{tx.timestamp}</TableBodyCell
              >
              <TableBodyCell class="font-mono text-xs text-gray-400"
                >{tx.canisterId}</TableBodyCell
              >
            </TableBodyRow>
          {/each}
        </TableBody>
      </Table>
    </div>
  </Card>
</div>
