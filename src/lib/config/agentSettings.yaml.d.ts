declare module './agentSettings.yaml' {
  const content: {
    commissionRate: {
      min: number;
      max: number;
      step: number;
      default: number;
    };
    serviceRadius: {
      min: number;
      max: number;
      step: number;
      default: number;
    };
    maxCashLimit: {
      min: number;
      max: number;
      step: number;
      default: number;
    };
    minimumTransaction: {
      min: number;
      max: number;
      step: number;
      default: number;
    };
    autoAcceptLimit: {
      min: number;
      max: number;
      step: number;
      default: number;
    };
    operatingHours: {
      default: {
        start: string;
        end: string;
      };
    };
  };
  export default content;
}
