/**
 * Demo Proposals Store
 * Manages demo mode proposals in localStorage
 */

import { writable } from "svelte/store";
import { browser } from "$app/environment";

export interface DemoProposal {
  id: string;
  type: string;
  title: string;
  description: string;
  proposer: string;
  createdAt: number;
  votingEndsAt: number;
  status: "active" | "passed" | "rejected";
  votes: {
    yes: number;
    no: number;
    abstain: number;
  };
  quorum: number;
  threshold: number;
}

const STORAGE_KEY = "afritokeni_demo_proposals";

function createDemoProposalsStore() {
  // Initialize from localStorage if in browser
  const loadStoredProposals = (): DemoProposal[] => {
    if (!browser) return [];

    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (!stored) return [];
      return JSON.parse(stored);
    } catch (error) {
      console.error("Failed to load demo proposals from localStorage:", error);
      return [];
    }
  };

  const { subscribe, set, update } = writable<DemoProposal[]>(
    loadStoredProposals(),
  );

  const saveToStorage = (proposals: DemoProposal[]) => {
    if (browser) {
      try {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(proposals));
      } catch (error) {
        console.error("Failed to save demo proposals to localStorage:", error);
      }
    }
  };

  return {
    subscribe,

    /**
     * Add a new demo proposal
     */
    addProposal: (
      proposal: Omit<
        DemoProposal,
        | "id"
        | "createdAt"
        | "votingEndsAt"
        | "votes"
        | "status"
        | "quorum"
        | "threshold"
      >,
    ) => {
      update((proposals) => {
        const now = Date.now();
        const votingPeriodMs = 7 * 24 * 60 * 60 * 1000; // 7 days

        const newProposal: DemoProposal = {
          ...proposal,
          id: `DEMO-${Date.now()}`,
          createdAt: now,
          votingEndsAt: now + votingPeriodMs,
          status: "active",
          votes: {
            yes: 0,
            no: 0,
            abstain: 0,
          },
          quorum: 20,
          threshold: 51,
        };

        const updated = [newProposal, ...proposals];
        saveToStorage(updated);
        return updated;
      });
    },

    /**
     * Vote on a demo proposal
     */
    vote: (
      proposalId: string,
      vote: "yes" | "no" | "abstain",
      tokenAmount: number,
    ) => {
      update((proposals) => {
        const updated = proposals.map((p) => {
          if (p.id === proposalId) {
            return {
              ...p,
              votes: {
                ...p.votes,
                [vote]: p.votes[vote] + tokenAmount,
              },
            };
          }
          return p;
        });
        saveToStorage(updated);
        return updated;
      });
    },

    /**
     * Reset to initial demo proposals (loads from JSON)
     */
    reset: async () => {
      try {
        const response = await fetch("/data/demo/proposals.json");
        if (response.ok) {
          const proposals = await response.json();
          set(proposals);
          saveToStorage(proposals);
        }
      } catch (error) {
        console.error("Failed to reset demo proposals:", error);
      }
    },

    /**
     * Clear all demo proposals
     */
    clear: () => {
      set([]);
      if (browser) {
        localStorage.removeItem(STORAGE_KEY);
      }
    },
  };
}

export const demoProposals = createDemoProposalsStore();
