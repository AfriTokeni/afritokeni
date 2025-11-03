import { Agent as DBAgent } from '../services/agentService';

export type DepositStep = 'amount' | 'agent' | 'confirmation';
export type ViewMode = 'list' | 'map';

export interface DepositFormData {
  amount: string;
  selectedCurrency: string;
  selectedAgent: DBAgent | null;
  depositCode: string;
  userLocation: [number, number] | null;
}

export interface AmountStepProps {
  amount: string;
  selectedCurrency: string;
  error: string;
  onAmountChange: (amount: string) => void;
  onCurrencyChange: (currency: string) => void;
  onContinue: () => void;
  onError: (error: string) => void;
}

export interface AgentSelectionStepProps {
  agents: DBAgent[];
  selectedAgent: DBAgent | null;
  amount: string;
  selectedCurrency: string;
  userLocation: [number, number] | null;
  viewMode: ViewMode;
  isLoadingAgents: boolean;
  isCreating: boolean;
  onAgentSelect: (agent: DBAgent) => void;
  onViewModeChange: (mode: ViewMode) => void;
  onCreateDepositRequest: (agent: DBAgent) => void;
}

export interface ConfirmationStepProps {
  amount: string;
  selectedCurrency: string;
  selectedAgent: DBAgent | null;
  depositCode: string;
  onReturnToDashboard: () => void;
  onMakeAnother: () => void;
}

export interface ProgressIndicatorProps {
  currentStep: DepositStep;
  steps: Array<{
    key: DepositStep;
    label: string;
    number: number;
  }>;
}

export interface DepositPageState {
  step: DepositStep;
  formData: DepositFormData;
  viewMode: ViewMode;
  isLoadingAgents: boolean;
  isCreating: boolean;
  error: string;
  agents: DBAgent[];
}