import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import AgentKYC from '../../components/AgentKYC';
import { AgentKYCData } from '../../types/auth';
import { useAuthentication } from '../../context/AuthenticationContext';
import { AgentService } from '../../services/agentService';


const AgentKYCPage: React.FC = () => {
  const navigate = useNavigate();
  const { user, updateUser } = useAuthentication();
  const [isLoading, setIsLoading] = useState(false);

  const handleKYCSubmit = async (data: AgentKYCData) => {
    setIsLoading(true);
    
    try {
      if (!user.agent) {
        throw new Error('No user found');
      }

      // Ensure coordinates are available
      if (!data.location.coordinates) {
        throw new Error('Location coordinates are required');
      }

      // Complete the agent KYC process
      const result = await AgentService.completeAgentKYC({
        userId: user.agent.id,
        firstName: data.firstName,
        lastName: data.lastName,
        phoneNumber: data.phoneNumber,
        businessName: `${data.firstName} ${data.lastName} Agent Service`, // Generate business name
        location: {
          country: data.location.country,
          state: data.location.state,
          city: data.location.city,
          address: data.location.address,
          coordinates: data.location.coordinates
        },
        operatingHours: data.operatingHours,
        operatingDays: data.operatingDays ? data.operatingDays.split(',').map(d => d.trim()) : [], // Convert string to array
        documentType: data.documentType,
        documentNumber: data.documentNumber,
        businessLicense: data.businessLicense
      });

      console.log('Agent KYC completed successfully:', result);

      // Update agent in context with new data
      await updateUser(result.user);
      
      // Show success message and redirect
      alert(`Agent verification completed successfully! 

Agent Details:
- Business: ${result.agent.businessName}
- Location: ${result.agent.location.city}, ${result.agent.location.state}
- Cash Balance: UGX ${result.agent.cashBalance.toLocaleString()}
- Digital Balance: UGX ${result.agent.digitalBalance.toLocaleString()}

Welcome to AfriTokeni Agent Network!`);
      
      navigate('/agents/dashboard');
    } catch (error) {
      console.error('KYC submission failed:', error);
      alert(`Failed to complete agent verification: ${error instanceof Error ? error.message : 'Unknown error'}. Please try again.`);
    } finally {
      setIsLoading(false);
    }
  };

  // Redirect if agent is not logged in
  if (!user.agent) {
    navigate('/');
    return null;
  }

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
      <AgentKYC onSubmit={handleKYCSubmit} isLoading={isLoading} />
    </div>
  );
};

export default AgentKYCPage;
