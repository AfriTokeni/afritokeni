/**
 * Customer Data Service (Demo Mode Only)
 *
 * Loads customer demo data from static JSON files.
 * NO BUSINESS LOGIC - For UI display only.
 *
 * For real customer management, use data_canister.
 */

export interface Customer {
  id: string;
  name: string;
  phone: string;
  totalTransactions: number;
  totalVolume: number;
  currency: string;
  lastTransaction?: number;
  status?: string;
}

/**
 * Fetch agent customers from demo data
 */
export async function fetchAgentCustomers(
  agentId: string,
  _demoMode?: boolean,
): Promise<Customer[]> {
  try {
    const response = await fetch("/data/demo/agent-customers.json");
    if (!response.ok) {
      throw new Error(`Failed to fetch demo customers: ${response.statusText}`);
    }
    const data = await response.json();

    // Demo data might be an array or object with agent IDs
    if (Array.isArray(data)) {
      return data;
    } else if (data[agentId]) {
      return data[agentId];
    }

    return [];
  } catch (error) {
    console.error("Error loading demo customers:", error);
    return [];
  }
}

/**
 * UI helper: Format customer status
 */
export function formatCustomerStatus(status?: string): string {
  const statusMap: Record<string, string> = {
    active: "Active",
    inactive: "Inactive",
    suspended: "Suspended",
    new: "New",
  };
  return statusMap[status?.toLowerCase() || ""] || "Unknown";
}

/**
 * UI helper: Get status color
 */
export function getStatusColor(status?: string): string {
  const colorMap: Record<string, string> = {
    active: "text-green-600",
    inactive: "text-gray-600",
    suspended: "text-red-600",
    new: "text-blue-600",
  };
  return colorMap[status?.toLowerCase() || ""] || "text-gray-600";
}
