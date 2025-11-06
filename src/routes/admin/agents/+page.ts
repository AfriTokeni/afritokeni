/**
 * Agents Page Data Loader
 * Data will be loaded in component after Juno initialization
 */

import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
  // Return empty initial data - component will load from Juno after initialization
  return {
    agents: [],
    stats: {
      total: 0,
      active: 0,
      busy: 0,
      offline: 0,
      totalRevenue: 0,
      totalCommission: 0,
    },
  };
};
