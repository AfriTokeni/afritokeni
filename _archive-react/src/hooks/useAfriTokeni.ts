import { useState, useEffect, useCallback } from 'react';
import { useAuthentication } from '../context/AuthenticationContext';
import { Agent, AgentService } from '../services/agentService';
import { UserService } from '../services/userService';
import { TransactionService } from '../services/transactionService';
import { BalanceService } from '../services/balanceService';
import { SMSService } from '../services/smsService';
import { DepositWithdrawalService } from '../services/depositWithdrawalService';
import { CentralizedDemoService } from '../services/centralizedDemoService';
import { useDemoMode } from '../context/DemoModeContext';
import { Transaction } from '../types/transaction';

interface UserBalance {
  balance: number;
  currency: string;
}

// Transaction and Agent interfaces imported from their respective modules

export const useAfriTokeni = () => {
  const { user } = useAuthentication();
  const { isDemoMode } = useDemoMode();
  const [balance, setBalance] = useState<UserBalance | null>(null);
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [agentTransactions, setAgentTransactions] = useState<Transaction[]>([]);
  const [agent, setAgent] = useState<Agent | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  console.log('useAfriTokeni - user:', user);




  const loadUserData = useCallback(async () => {
    // Check if we have any user logged in
    if (!user?.user?.id && !user?.agent?.id) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      // Initialize arrays to store data for both user types
      const dataPromises: Promise<any>[] = [];
      
      // Load regular user data if user is logged in
      if (user?.user?.id) {
        if (isDemoMode) {
          // Use demo data from CentralizedDemoService
          const demoTransactions = await CentralizedDemoService.getTransactions(user.user.id);
          dataPromises.push(
            Promise.resolve({ balance: 500000, currency: 'UGX' }),
            Promise.resolve(demoTransactions)
          );
        } else {
          // User data initialization handled by services
          dataPromises.push(
            BalanceService.getUserBalance(user.user.id),
            TransactionService.getUserTransactions(user.user.id)
          );
        }
      }

      console.log('Data promises for user:', user.agent?.id);
      
      // Load agent data if agent is logged in
      if (user?.agent?.id) {
        // User data initialization handled by services
        
        // First get agent data, then get all agent-related transactions
        const agentData = await AgentService.getAgentByUserId(user.agent.id);
        const agentTransactionsData = agentData ? 
          await TransactionService.getAgentTransactions(agentData.id) : [];
        
        // Add to results manually instead of using promises array
        dataPromises.push(
          Promise.resolve(agentData),
          Promise.resolve(agentTransactionsData)
        );
      }
      
      // Execute all promises
      const results = await Promise.all(dataPromises);
      
      let resultIndex = 0;
      
      // Process regular user data if available
      if (user?.user?.id) {
        const userBalance = results[resultIndex++];
        const userTransactions = results[resultIndex++];
        
        setBalance(userBalance);
        setTransactions(userTransactions);
      } else {
        // Clear user data if no user is logged in
        setBalance(null);
        setTransactions([]);
      }
      
      // Process agent data if available
      if (user?.agent?.id) {
        const agentData = results[resultIndex++];
        const agentTransactionsData = results[resultIndex++];
        
        setAgent(agentData);
        setAgentTransactions(agentTransactionsData);
      } else {
        // Clear agent data if no agent is logged in
        setAgent(null);
        setAgentTransactions([]);
      }
      
    } catch (err) {
      setError('Failed to load user data');
      console.error('Error loading user data:', err);
    } finally {
      setIsLoading(false);
    }
  }, [user?.user?.id, user?.agent?.id]);

  // Load user data when user changes or demo mode changes
  useEffect(() => {
    if (user?.user?.id || user?.agent?.id) {
      loadUserData();
    } else {
      setBalance(null);
      setTransactions([]);
      setAgent(null);
      setAgentTransactions([]);
    }
  }, [user?.user?.id, user?.agent?.id, isDemoMode, loadUserData]);

  const calculateFee = (amount: number): number => {
    return Math.round(amount * 0.005); // 0.5% platform fee per whitepaper
  };

  const sendMoney = async (
    amount: number,
    recipientPhone: string,
    recipient: any
  ): Promise<{ 
    success: boolean; 
    message: string; 
    transaction?: Transaction;
    fee?: number;
  }> => {
    if (!user.user?.id) {
      return { success: false, message: 'User not authenticated' };
    }

    const fee = calculateFee(amount);
    const totalAmount = amount + fee;

    // Check if sender has sufficient balance
    const senderBalance = await BalanceService.getUserBalance(user.user.id);
    if (!senderBalance || senderBalance.balance < totalAmount) {
      return { success: false, message: 'Insufficient balance' };
    }

    setIsLoading(true);
    setError(null);

    try {
      // Create send transaction for sender
      const sendTransaction = await TransactionService.createTransaction({
        userId: user.user.id,
        type: 'send',
        amount: amount,
        currency: 'UGX',
        recipientId: recipient.id,
        recipientPhone: recipientPhone,
        recipientName: `${recipient.firstName} ${recipient.lastName}`,
        status: 'completed',
        description: `Money sent to ${recipient.firstName} ${recipient.lastName}`,
        completedAt: new Date(),
        metadata: {
          smsReference: `SEND${Date.now().toString().slice(-6)}`
        }
      });

      // Create receive transaction for recipient
      await TransactionService.createTransaction({
        userId: recipient.id,
        type: 'receive',
        amount: amount,
        currency: 'UGX',
        status: 'completed',
        description: `Money received from ${user.user.firstName} ${user.user.lastName}`,
        completedAt: new Date(),
        metadata: {
          smsReference: `RCV${Date.now().toString().slice(-6)}`
        }
      });

      // Update sender balance (deduct amount + fee)
      console.log('Updating sender balance:', {
        userId: user.user.id,
        currentBalance: senderBalance.balance,
        totalAmount,
        newBalance: senderBalance.balance - totalAmount
      });
      const senderUpdateSuccess = await BalanceService.updateUserBalance(user.user.id, senderBalance.balance - totalAmount);
      if (!senderUpdateSuccess) {
        throw new Error('Failed to update sender balance');
      }

      // Update recipient balance (add amount)
      const recipientBalance = await BalanceService.getUserBalance(recipient.id);
      const newRecipientBalance = (recipientBalance?.balance || 0) + amount;
      console.log('Updating recipient balance:', {
        userId: recipient.id,
        currentBalance: recipientBalance?.balance || 0,
        amount,
        newBalance: newRecipientBalance
      });
      const recipientUpdateSuccess = await BalanceService.updateUserBalance(recipient.id, newRecipientBalance);
      if (!recipientUpdateSuccess) {
        throw new Error('Failed to update recipient balance');
      }

      // Send SMS notifications
      try {
        // SMS to sender
        const senderSMS = `AfriTokeni: You sent UGX ${amount.toLocaleString()} to ${recipient.firstName} ${recipient.lastName} (${recipientPhone}). Fee: UGX ${fee.toLocaleString()}. New balance: UGX ${(senderBalance.balance - totalAmount).toLocaleString()}. Ref: ${sendTransaction.id}`;
        await fetch(`http://localhost:3001/api/send-sms`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            phoneNumber: user.user.email,
            message: senderSMS,
            transactionId: sendTransaction.id
          })
        });

        // SMS to recipient
        const recipientSMS = `AfriTokeni: You received UGX ${amount.toLocaleString()} from ${user.user.firstName} ${user.user.lastName} (${user.user.email}). New balance: UGX ${newRecipientBalance.toLocaleString()}. Ref: ${sendTransaction.id}`;
        await fetch(`http://localhost:3001/api/send-sms`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            phoneNumber: recipientPhone,
            message: recipientSMS,
            transactionId: sendTransaction.id
          })
        });
      } catch (smsError) {
        console.error('SMS sending failed:', smsError);
      }

      // Refresh data
      await loadUserData();

      return { 
        success: true, 
        message: `Successfully sent UGX ${amount.toLocaleString()} to ${recipient.firstName} ${recipient.lastName}`,
        transaction: sendTransaction,
        fee: fee
      };
    } catch (err) {
      setError('Failed to send money');
      console.error('Error sending money:', err);
      return { success: false, message: 'Failed to send money' };
    } finally {
      setIsLoading(false);
    }
  };

  const withdrawMoney = async (
    amount: number,
    currency: string,
    agentId?: string
  ): Promise<{ success: boolean; message: string; withdrawalCode?: string }> => {
    if (!user.user?.id) {
      return { success: false, message: 'User not authenticated' };
    }

    setIsLoading(true);
    setError(null);

    try {
      // Create withdrawal request
      const withdrawalRequest = await DepositWithdrawalService.createWithdrawalRequest(
        user.user.id,
        agentId || '',
        amount,
        currency
      );
      
      // Refresh data after successful withdrawal
      if (withdrawalRequest) {
        await loadUserData();
        return { 
          success: true, 
          message: 'Withdrawal request created',
          withdrawalCode: withdrawalRequest.withdrawalCode 
        };
      }

      return { success: false, message: 'Failed to create withdrawal request' };
    } catch (err) {
      setError('Failed to initiate withdrawal');
      console.error('Error withdrawing money:', err);
      return { success: false, message: 'Failed to initiate withdrawal' };
    } finally {
      setIsLoading(false);
    }
  };

  const depositMoney = async (amount: number): Promise<{ success: boolean; message: string }> => {
    if (!user.user?.id) {
      return { success: false, message: 'User not authenticated' };
    }

    setIsLoading(true);
    setError(null);

    try {
      // Create transaction
      await TransactionService.createTransaction({
        userId: user.user.id,
        type: 'deposit',
        amount,
        currency: 'UGX',
        status: 'completed',
        description: 'Cash deposit',
        completedAt: new Date()
      });

      // Update balance
      const newBalance = (balance?.balance || 0) + amount;
      await BalanceService.updateUserBalance(user.user.id, newBalance);

      // Refresh data
      await loadUserData();

      return { 
        success: true, 
        message: `Successfully deposited UGX ${amount.toLocaleString()}`
      };
    } catch (err) {
      console.error('Error processing deposit:', err);
      return { success: false, message: 'Sorry, there was an error processing your request.' };
    }
  };

  const refreshData = () => {
    loadUserData();
  };


  // Get nearby agents
  const getNearbyAgents = useCallback(async (lat: number, lng: number, radius: number = 5, includeStatuses?: ('available' | 'busy' | 'cash_out' | 'offline')[]) => {
    try {
      return await AgentService.getNearbyAgents(lat, lng, radius, includeStatuses);
    } catch (error) {
      console.error('Error getting nearby agents:', error);
      return [];
    }
  }, []);


  // Process SMS command
  const processSMSCommand = useCallback(async (command: string) => {
    try {
      return await SMSService.processSMSCommand(command, user?.user?.id || '');
    } catch (error) {
      console.error('Error processing SMS command:', error);
      return 'Error processing command';
    }
  }, [user?.user?.id]);

  // Agent status management
  const updateAgentStatus = async (status: 'available' | 'busy' | 'cash_out' | 'offline'): Promise<boolean> => {
    if (!user.agent?.id || user.agent.userType !== 'agent') {
      return false;
    }

    try {
      const success = await AgentService.updateAgentStatusByUserId(user.agent.id, status);
      if (success) {
        // Refresh agent data to reflect the status change
        await loadUserData();
      }
      return success;
    } catch (error) {
      console.error('Error updating agent status:', error);
      return false;
    }
  };

  return {
    // Data
    user,
    balance,
    transactions,
    agentTransactions,
    agent,
    isLoading,
    error,
    
    // Actions
    sendMoney,
    withdrawMoney,
    depositMoney,
    getNearbyAgents,
    processSMSCommand,
    refreshData,
    calculateFee,
    updateAgentStatus,
    
    // Computed values
    formattedBalance: balance ? `UGX ${balance.balance.toLocaleString()}` : 'UGX 0',
    hasTransactions: transactions.length > 0,
    recentTransactions: transactions.slice(0, 5)
  };
};
