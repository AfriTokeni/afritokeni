/**
 * Agent Settings Configuration Data
 *
 * NEVER HARDCODE VALUES IN COMPONENTS!
 * All configurable limits and ranges should be defined here.
 * This allows easy updates without touching component code.
 */

interface RangeConfig {
  min: number;
  max: number;
  step: number;
  default: number;
}

interface OperatingHoursConfig {
  default: {
    start: string;
    end: string;
  };
}

export interface AgentSettingsConfig {
  commissionRate: RangeConfig;
  serviceRadius: RangeConfig;
  maxCashLimit: RangeConfig;
  minimumTransaction: RangeConfig;
  autoAcceptLimit: RangeConfig;
  operatingHours: OperatingHoursConfig;
}

export const AGENT_SETTINGS_DATA: AgentSettingsConfig = {
  // Commission Rate Configuration
  commissionRate: {
    min: 0,
    max: 15, // Platform maximum commission rate (%)
    step: 0.5,
    default: 2.5,
  },

  // Service Radius Configuration (km)
  serviceRadius: {
    min: 1,
    max: 100, // Maximum service radius in km
    step: 1,
    default: 5,
  },

  // Max Cash Limit Configuration (UGX)
  maxCashLimit: {
    min: 100000, // 100K UGX
    max: 10000000, // 10M UGX
    step: 100000,
    default: 500000,
  },

  // Minimum Transaction Configuration (UGX)
  minimumTransaction: {
    min: 500,
    max: 100000, // 100K UGX
    step: 500,
    default: 1000,
  },

  // Auto-Accept Limit Configuration (UGX)
  autoAcceptLimit: {
    min: 1000,
    max: 100000,
    step: 1000,
    default: 50000,
  },

  // Operating Hours
  operatingHours: {
    default: {
      start: "08:00",
      end: "18:00",
    },
  },
};
