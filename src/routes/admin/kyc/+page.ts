/**
 * KYC Page Data Loader
 * Loads KYC data from Juno on page load
 */

import { listKYCDocuments, getKYCStats } from "$lib/services/juno/kycService";
import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
  try {
    // Load all KYC documents (we'll filter client-side for now)
    const [documents, stats] = await Promise.all([
      listKYCDocuments({ limit: 100 }),
      getKYCStats(),
    ]);

    return {
      documents,
      stats,
    };
  } catch (error) {
    console.error("Error loading KYC data:", error);
    // Return empty data on error - page will show empty state
    return {
      documents: [],
      stats: {
        total: 0,
        pending: 0,
        approved: 0,
        rejected: 0,
      },
    };
  }
};
