import React, { useState, useEffect, useCallback } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuthentication } from '../../context/AuthenticationContext';
import { Agent, AgentService } from '../../services/agentService';
import { DepositWithdrawalService } from '../../services/depositWithdrawalService';
import { 
  AmountStep, 
  AgentSelectionStep, 
  ConfirmationStep, 
  ProgressIndicator,
  DepositStep,
  ViewMode,
  DepositPageState 
} from '.';

const DepositPage: React.FC = () => {
  const navigate = useNavigate();
  const { user } = useAuthentication();

  // State
  const [state, setState] = useState<DepositPageState>({
    step: 'amount',
    formData: {
      amount: '',
      selectedCurrency: 'UGX',
      selectedAgent: null,
      depositCode: '',
      userLocation: null,
    },
    viewMode: 'list',
    isLoadingAgents: false,
    isCreating: false,
    error: '',
    agents: [],
  });

  // Destructure state for convenience
  const { step, formData, viewMode, isLoadingAgents, isCreating, error, agents } = state;
  const { amount, selectedCurrency, selectedAgent, depositCode, userLocation } = formData;

  // Progress steps configuration
  const progressSteps = [
    { key: 'amount' as DepositStep, label: 'Amount', number: 1 },
    { key: 'agent' as DepositStep, label: 'Select Agent', number: 2 },
    { key: 'confirmation' as DepositStep, label: 'Confirmation', number: 3 },
  ];

  // Update state helper
  const updateState = useCallback((updates: Partial<DepositPageState>) => {
    setState(prev => ({ ...prev, ...updates }));
  }, []);

  // Update form data helper
  const updateFormData = useCallback((updates: Partial<typeof formData>) => {
    setState(prev => ({
      ...prev,
      formData: { ...prev.formData, ...updates }
    }));
  }, []);

  // Load nearby agents
  const loadNearbyAgents = useCallback(async () => {
    if (!userLocation) {
      console.log('No user location available, cannot load agents');
      return;
    }

    console.log('User location available:', userLocation);

    // Prevent duplicate loading
    if (isLoadingAgents) {
      console.log('Already loading agents, skipping...');
      return;
    }

    console.log('Loading nearby agents...');
    updateState({ isLoadingAgents: true, error: '' });
    try {
      const nearbyAgents = await AgentService.getNearbyAgents(
        userLocation[0], 
        userLocation[1], 
        10000
      );
      console.log('Loaded agents:', nearbyAgents.length, nearbyAgents.map(a => ({ id: a.id, name: a.businessName })));
      updateState({ agents: nearbyAgents, isLoadingAgents: false });
    } catch (error) {
      console.error('Error loading nearby agents:', error);
      updateState({ 
        error: 'Failed to load nearby agents', 
        isLoadingAgents: false,
        agents: []
      });
    }
  }, [userLocation]);

  // Get user location on mount
  useEffect(() => {
    const getUserLocation = () => {
      if (navigator.geolocation) {
        navigator.geolocation.getCurrentPosition(
          (position) => {
            const location: [number, number] = [
              position.coords.latitude,
              position.coords.longitude
            ];
            updateFormData({ userLocation: location });
          },
          (error) => {
            console.error('Error getting location:', error);
            console.log('Using default location (Kampala, Uganda) as fallback');
            // Use Kampala, Uganda as default location when geolocation fails
            const defaultLocation: [number, number] = [0.3476, 32.5825]; // Kampala coordinates
            updateFormData({ userLocation: defaultLocation });
            updateState({ error: 'Using default location. For better results, please enable location services.' });
          }
        );
      } else {
        console.log('Geolocation not supported, using default location (Kampala, Uganda)');
        // Use Kampala, Uganda as default location when geolocation is not supported
        const defaultLocation: [number, number] = [0.3476, 32.5825]; // Kampala coordinates
        updateFormData({ userLocation: defaultLocation });
        updateState({ error: 'Geolocation not supported. Using default location (Kampala, Uganda).' });
      }
    };

    getUserLocation();
  }, [updateFormData, updateState]);

  // Auto-load agents when location is available
  useEffect(() => {
    if (userLocation && step === 'agent' && agents.length === 0 && !isLoadingAgents) {
      console.log('Auto-loading agents from useEffect...');
      loadNearbyAgents();
    }
  }, [userLocation, step,loadNearbyAgents]);

  // Create deposit request
  const createDepositRequest = useCallback(async (agent: Agent) => {
    console.log('createDepositRequest called with agent:', agent);
    console.log('Current user:', user);
    console.log('User object keys:', user ? Object.keys(user) : 'user is null');
    console.log('user.user:', user?.user);
    
    if (!user?.user) {
      console.log('User not authenticated');
      updateState({ error: 'User not authenticated' });
      return;
    }

    console.log('User authenticated, proceeding with deposit creation');
    updateState({ isCreating: true });
    
    try {
      // Generate a unique deposit code
      // Save deposit request to datastore
      const depositRequest = await DepositWithdrawalService.createDepositRequest(
        user.user.id,
        agent.id,
        parseFloat(amount),
        selectedCurrency
      );
      
      const requestId = depositRequest.id;
      const code = depositRequest.depositCode;
      
      console.log('Deposit request created:', {
        requestId,
        userId: user.user.id,
        agentId: agent.id,
        amount: parseFloat(amount),
        currency: selectedCurrency,
        code,
      });
      
      updateFormData({ depositCode: code });
      updateState({ step: 'confirmation', isCreating: false });
    } catch (error) {
      console.error('Error creating deposit request:', error);
      updateState({ error: 'Failed to create deposit request', isCreating: false });
    }
  }, [user, amount, selectedCurrency, updateFormData, updateState]);

  // Step handlers
  const handleAmountContinue = useCallback(() => {
    updateState({ step: 'agent' });
    // Only load if no agents and not already loading and user location is available
    if (agents.length === 0 && !isLoadingAgents && userLocation) {
      console.log('No agents available, loading from handleAmountContinue...');
      loadNearbyAgents();
    }
  }, [agents.length, isLoadingAgents, userLocation, loadNearbyAgents, updateState]);

  const handleAgentSelect = useCallback((agent: Agent) => {
    updateFormData({ selectedAgent: agent });
  }, [updateFormData]);

  const handleReturnToDashboard = useCallback(() => {
    navigate('/users/dashboard');
  }, [navigate]);

  const handleMakeAnother = useCallback(() => {
    setState({
      step: 'amount',
      formData: {
        amount: '',
        selectedCurrency: 'UGX',
        selectedAgent: null,
        depositCode: '',
        userLocation: userLocation,
      },
      viewMode: 'list',
      isLoadingAgents: false,
      isCreating: false,
      error: '',
      agents: [],
    });
  }, [userLocation]);

  const handleAmountChange = useCallback((newAmount: string) => {
    updateFormData({ amount: newAmount });
  }, [updateFormData]);

  const handleCurrencyChange = useCallback((currency: string) => {
    updateFormData({ selectedCurrency: currency });
  }, [updateFormData]);

  const handleError = useCallback((errorMessage: string) => {
    updateState({ error: errorMessage });
  }, [updateState]);

  const handleViewModeChange = useCallback((mode: ViewMode) => {
    updateState({ viewMode: mode });
  }, [updateState]);

  return (
    <div className="space-y-4 sm:space-y-5 md:space-y-6">
      <div className="max-w-6xl mx-auto p-3 sm:p-4 md:p-5 lg:p-6">
        {/* Progress Indicator */}
        <ProgressIndicator
          steps={progressSteps}
          currentStep={step}
        />

        {/* Error Display */}
        {error && (
          <div className="bg-red-50 border border-red-200 text-red-700 px-3 py-2.5 sm:px-4 sm:py-3 rounded text-xs sm:text-sm mb-3 sm:mb-4 break-words">
            {error}
          </div>
        )}

        {/* Step Content */}
        {/* <div className="bg-white rounded-lg shadow-sm border p-6"> */}
          {step === 'amount' && (
            <AmountStep
              amount={amount}
              selectedCurrency={selectedCurrency}
              error={error}
              onAmountChange={handleAmountChange}
              onCurrencyChange={handleCurrencyChange}
              onContinue={handleAmountContinue}
              onError={handleError}
            />
          )}

          {step === 'agent' && (
            <AgentSelectionStep
              agents={agents}
              selectedAgent={selectedAgent}
              amount={amount}
              selectedCurrency={selectedCurrency}
              userLocation={userLocation}
              viewMode={viewMode}
              isLoadingAgents={isLoadingAgents}
              isCreating={isCreating}
              onAgentSelect={handleAgentSelect}
              onViewModeChange={handleViewModeChange}
              onCreateDepositRequest={createDepositRequest}
            />
          )}

          {step === 'confirmation' && (
            <ConfirmationStep
              amount={amount}
              selectedCurrency={selectedCurrency}
              selectedAgent={selectedAgent}
              depositCode={depositCode}
              onReturnToDashboard={handleReturnToDashboard}
              onMakeAnother={handleMakeAnother}
            />
          )}
        {/* </div> */}
      </div>
    </div>
  );
};

export default DepositPage;
