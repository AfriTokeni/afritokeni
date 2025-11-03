/**
 * Agent Settings Configuration Loader
 * 
 * Loads configuration from YAML file.
 * NEVER HARDCODE VALUES IN COMPONENTS!
 */

import configYaml from './agentSettings.yaml';

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

interface AgentSettingsConfig {
	commissionRate: RangeConfig;
	serviceRadius: RangeConfig;
	maxCashLimit: RangeConfig;
	minimumTransaction: RangeConfig;
	autoAcceptLimit: RangeConfig;
	operatingHours: OperatingHoursConfig;
}

export const AGENT_SETTINGS_CONFIG = configYaml as AgentSettingsConfig;

/**
 * Get formatted label for slider markers
 */
export function getSliderLabel(value: number, suffix: string = ''): string {
	if (value >= 1000000) {
		return `${(value / 1000000).toFixed(1)}M${suffix}`;
	}
	if (value >= 1000) {
		return `${(value / 1000).toFixed(0)}K${suffix}`;
	}
	return `${value}${suffix}`;
}
