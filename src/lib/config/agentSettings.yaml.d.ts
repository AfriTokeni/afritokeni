declare module "./agentSettings.yaml" {
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

  const content: AgentSettingsConfig;
  export default content;
}
