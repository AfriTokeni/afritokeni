import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuthentication } from '../../context/AuthenticationContext';
import UserKYC from '../../components/UserKYC';
import { UserKYCData } from '../../types/auth';

import { KYCService } from '../../services/kycService';
import { UserService } from '../../services/userService';

const UserKYCPage: React.FC = () => {
  const navigate = useNavigate();
  const { user, updateUser } = useAuthentication();
  const [isLoading, setIsLoading] = useState(false);

  const handleKYCSubmit = async (data: UserKYCData) => {
    setIsLoading(true);
    
    try {
      // Format phone number consistently
      const formattedPhone = data.phoneNumber.startsWith('+') ? 
        data.phoneNumber : 
        `+256${data.phoneNumber.replace(/^0/, '')}`;

      // Get current user ID
      const currentUser = user.user;
      if (!currentUser?.id) {
        throw new Error('User must be logged in to submit KYC');
      }

      // Submit KYC using the new KYCService with document upload
      const kycResult = await KYCService.submitKYC(currentUser.id, data);
      
      if (!kycResult.success) {
        throw new Error(kycResult.error || 'Failed to submit KYC');
      }

      // Update user's KYC status in the main user record
      const updateSuccess = await UserService.updateUser(currentUser.id, {
        firstName: data.firstName,
        lastName: data.lastName,
        kycStatus: 'pending',
        email: formattedPhone // Store phone number for financial operations
      });
      
      if (!updateSuccess) {
        console.warn('Failed to update user record, but KYC was submitted successfully');
      }

      // Update authentication context
      await updateUser({
        ...currentUser,
        firstName: data.firstName,
        lastName: data.lastName,
        email: formattedPhone,
        kycStatus: 'pending',
      });

      console.log('KYC submitted successfully with ID:', kycResult.submissionId);
      
      // Show success message and redirect
      alert('KYC verification submitted successfully! Your documents have been uploaded securely and will be reviewed within 24-48 hours.');
      navigate('/users/dashboard');
    } catch (error) {
      console.error('KYC submission failed:', error);
      alert(`Failed to submit KYC verification: ${error instanceof Error ? error.message : 'Unknown error'}`);
    } finally {
      setIsLoading(false);
    }
  };

  // Allow access to KYC page even if not logged in for new user registration
  // If user is logged in and is an agent, redirect to agent KYC
  if (user.user && user.user.userType === 'agent') {
    navigate('/agents/agent-kyc');
    return null;
  }

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
      <UserKYC onSubmit={handleKYCSubmit} isLoading={isLoading} />
    </div>
  );
};

export default UserKYCPage;
