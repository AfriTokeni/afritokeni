/**
 * Agent Settings Configuration Loader
 *
 * Loads configuration from TypeScript data file.
 * NEVER HARDCODE VALUES IN COMPONENTS!
 */

import { AGENT_SETTINGS_DATA } from "./agentSettingsData";

export const AGENT_SETTINGS_CONFIG = AGENT_SETTINGS_DATA;

/**
 * Get formatted label for slider markers
 */
export function getSliderLabel(value: number, suffix: string = ""): string {
  if (value >= 1000000) {
    return `${(value / 1000000).toFixed(1)}M${suffix}`;
  }
  if (value >= 1000) {
    return `${(value / 1000).toFixed(0)}K${suffix}`;
  }
  return `${value}${suffix}`;
}
