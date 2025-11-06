/**
 * System Service
 * Handles system health, canister status, and monitoring data
 * Fetches from both Juno DB and ICP Network
 */

// Use DFX-generated canister IDs (source of truth)
import { getCanisterStatus as getICPCanisterStatus } from "$lib/services/icp/canisterService";

export interface CanisterStatus {
  name: string;
  id: string;
  cycles: number | null;
  status: "healthy" | "warning" | "error";
  uptime: string;
  memorySize: number | null;
  lastUpdated: string;
}

/**
 * Get canister list from env vars
 */
function getCanisterList(): Array<{ id: string; name: string }> {
  const depositId = process.env.CANISTER_ID_DEPOSIT_CANISTER;
  const withdrawalId = process.env.CANISTER_ID_WITHDRAWAL_CANISTER;
  const exchangeId = process.env.CANISTER_ID_EXCHANGE_CANISTER;

  if (!depositId) {
    throw new Error("CANISTER_ID_DEPOSIT_CANISTER not found in environment");
  }
  if (!withdrawalId) {
    throw new Error("CANISTER_ID_WITHDRAWAL_CANISTER not found in environment");
  }
  if (!exchangeId) {
    throw new Error("CANISTER_ID_EXCHANGE_CANISTER not found in environment");
  }

  return [
    { id: depositId, name: "Deposit Canister" },
    { id: withdrawalId, name: "Withdrawal Canister" },
    { id: exchangeId, name: "Exchange Canister" },
  ];
}

export interface SystemLog {
  id: string;
  timestamp: string;
  level: "error" | "warning" | "info";
  message: string;
  canister: string;
  details?: string;
}

export interface APIStatus {
  name: string;
  status: "operational" | "degraded" | "down";
  responseTime: string;
  lastChecked: string;
}

export interface SystemHealth {
  overall: "healthy" | "warning" | "error";
  uptime: string;
  totalCycles: number;
  lastDeployment: string;
  uptimeTrend?: number;
  cycleTrend?: number;
}

export interface CyclesChartData {
  categories: string[];
  series: {
    name: string;
    data: number[];
  }[];
}

/**
 * Fetch canister status from ICP network
 */
export async function getCanisterStatus(): Promise<CanisterStatus[]> {
  try {
    const canisterList = getCanisterList();

    // Fetch REAL status from each canister
    const statusPromises = canisterList.map(async (canister) => {
      const icpStatus = await getICPCanisterStatus(canister.id);

      return {
        name: canister.name,
        id: canister.id,
        cycles: icpStatus.cycles,
        status: icpStatus.status,
        uptime: icpStatus.uptime,
        memorySize: icpStatus.memorySize,
        lastUpdated: new Date().toISOString(),
      };
    });

    return await Promise.all(statusPromises);
  } catch (error) {
    console.error("Error fetching canister status:", error);
    throw error;
  }
}

/**
 * Get system health overview
 */
export async function getSystemHealth(): Promise<SystemHealth> {
  try {
    const canisters = await getCanisterStatus();
    // Can't calculate total cycles without controller access
    const totalCycles = canisters.every((c) => c.cycles !== null)
      ? canisters.reduce((sum, c) => sum + (c.cycles as number), 0)
      : 0;

    // Calculate average uptime
    const uptimes = canisters.map((c) => parseFloat(c.uptime.replace("%", "")));
    const avgUptime = uptimes.reduce((sum, u) => sum + u, 0) / uptimes.length;

    // Determine overall health
    let overall: SystemHealth["overall"] = "healthy";
    if (canisters.some((c) => c.status === "error")) {
      overall = "error";
    } else if (canisters.some((c) => c.status === "warning")) {
      overall = "warning";
    }

    // No historical data for trends yet
    const uptimeTrend: number | undefined = undefined;
    const cycleTrend: number | undefined = undefined;

    return {
      overall,
      uptime: `${avgUptime.toFixed(1)}%`,
      totalCycles: parseFloat(totalCycles.toFixed(1)),
      lastDeployment: new Date().toLocaleString(),
      uptimeTrend,
      cycleTrend,
    };
  } catch (error) {
    console.error("Error fetching system health:", error);
    throw error;
  }
}

/**
 * Get system logs - returns empty for now to show placeholder UI
 */
export async function getSystemLogs(_filters?: {
  level?: "error" | "warning" | "info" | "all";
  limit?: number;
}): Promise<SystemLog[]> {
  // TODO: Implement real logging system with Juno collection
  return [];
}

/**
 * Get API status for external services
 */
export async function getAPIStatus(): Promise<APIStatus[]> {
  const now = new Date().toLocaleString();
  const apis: APIStatus[] = [];

  // Check ICP Network by pinging canisters
  try {
    const startTime = performance.now();
    await getCanisterStatus();
    const responseTime = performance.now() - startTime;

    apis.push({
      name: "ICP Network",
      status: "operational",
      responseTime: `${Math.round(responseTime)}ms`,
      lastChecked: now,
    });
  } catch {
    apis.push({
      name: "ICP Network",
      status: "down",
      responseTime: "N/A",
      lastChecked: now,
    });
  }

  // Check AfricasTalking SMS Gateway - check status page
  try {
    const smsStart = performance.now();
    const response = await fetch("https://status.africastalking.com");
    const smsTime = performance.now() - smsStart;

    apis.push({
      name: "AfricasTalking (SMS)",
      status: response.ok ? "operational" : "degraded",
      responseTime: `${Math.round(smsTime)}ms`,
      lastChecked: now,
    });
  } catch {
    apis.push({
      name: "AfricasTalking (SMS)",
      status: "down",
      responseTime: "N/A",
      lastChecked: now,
    });
  }

  return apis;
}

/**
 * Get cycles usage chart data - returns current snapshot only
 */
export async function getCyclesChartData(
  days: number,
): Promise<CyclesChartData> {
  try {
    const canisters = await getCanisterStatus();

    // Generate date range
    const dateRange = Array.from({ length: days }, (_, i) => {
      const date = new Date();
      date.setDate(date.getDate() - (days - 1 - i));
      return date;
    });

    const categories = dateRange.map((date) =>
      date.toLocaleDateString("en-US", { month: "short", day: "numeric" }),
    );

    // Use current canister data for all points (no historical data yet)
    // If cycles is null (no controller access), use 0
    const series = canisters.map((canister) => ({
      name: canister.name,
      data: Array(days).fill(canister.cycles !== null ? canister.cycles : 0),
    }));

    return {
      categories,
      series,
    };
  } catch (error) {
    console.error("Error fetching cycles chart data:", error);
    throw error;
  }
}
