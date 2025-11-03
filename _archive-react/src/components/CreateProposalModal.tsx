import { useState } from 'react';
import { X, FileText, DollarSign, Globe, Shield, Lightbulb } from 'lucide-react';
import { GovernanceService, ProposalType } from '../services/governanceService';
import { useDemoMode } from '../context/DemoModeContext';

interface CreateProposalModalProps {
  isOpen: boolean;
  onClose: () => void;
  userId: string;
  userTokens: number;
  onSuccess: () => void;
}

export default function CreateProposalModal({
  isOpen,
  onClose,
  userId,
  userTokens,
  onSuccess,
}: CreateProposalModalProps) {
  const { isDemoMode } = useDemoMode();
  const [type, setType] = useState<ProposalType>('other');
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [loading, setLoading] = useState(false);

  const proposalTypes = [
    { value: 'fee_adjustment' as ProposalType, label: 'Fee Adjustment', icon: DollarSign, color: 'text-green-600' },
    { value: 'currency_addition' as ProposalType, label: 'Add Currency', icon: Globe, color: 'text-blue-600' },
    { value: 'agent_standards' as ProposalType, label: 'Agent Standards', icon: Shield, color: 'text-gray-600' },
    { value: 'treasury' as ProposalType, label: 'Treasury Management', icon: FileText, color: 'text-orange-600' },
    { value: 'other' as ProposalType, label: 'Other', icon: Lightbulb, color: 'text-gray-600' },
  ];

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!title || !description) {
      alert('Please fill in all fields');
      return;
    }

    setLoading(true);
    try {
      if (isDemoMode) {
        // In demo mode, create a local demo proposal
        await new Promise(resolve => setTimeout(resolve, 1000));
        await GovernanceService.createDemoProposal({
          type,
          title,
          description,
          proposer: userId,
          userTokens,
        });
      } else {
        await GovernanceService.createProposal(
          userId,
          {
            type,
            title,
            description,
            executionData: {},
          },
          userTokens
        );
      }
      
      onSuccess();
      onClose();
      setTitle('');
      setDescription('');
      setType('other');
    } catch (error: any) {
      console.error('Error creating proposal:', error);
      alert(error.message || 'Failed to create proposal on SNS governance');
    } finally {
      setLoading(false);
    }
  };

  if (!isOpen) return null;

  const minTokensRequired = GovernanceService.MIN_TOKENS_TO_PROPOSE;
  const canCreateProposal = userTokens >= minTokensRequired;

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-2xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
        {/* Header */}
        <div className="sticky top-0 bg-white border-b border-gray-200 p-6 flex items-center justify-between">
          <h2 className="text-2xl font-bold text-gray-900">Create Proposal</h2>
          <button
            onClick={onClose}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Content */}
        <form onSubmit={handleSubmit} className="p-6 space-y-6">
          {/* Token Requirement Warning */}
          {!canCreateProposal && (
            <div className="bg-red-50 border border-red-200 rounded-lg p-4">
              <p className="text-red-800 font-semibold">Insufficient Tokens</p>
              <p className="text-red-600 text-sm mt-1">
                You need at least {minTokensRequired.toLocaleString()} AFRI tokens to create a proposal.
                You currently have {userTokens.toLocaleString()} AFRI.
              </p>
            </div>
          )}

          {/* Proposal Type */}
          <div>
            <label className="block text-sm font-semibold text-gray-900 mb-3">
              Proposal Type
            </label>
            <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
              {proposalTypes.map((proposalType) => {
                const Icon = proposalType.icon;
                return (
                  <button
                    key={proposalType.value}
                    type="button"
                    onClick={() => setType(proposalType.value)}
                    className={`p-4 rounded-lg border-2 transition-all ${
                      type === proposalType.value
                        ? 'border-gray-900 bg-gray-50'
                        : 'border-gray-200 hover:border-gray-300'
                    }`}
                  >
                    <Icon className={`w-6 h-6 ${proposalType.color} mx-auto mb-2`} />
                    <p className="text-sm font-medium text-gray-900 text-center">
                      {proposalType.label}
                    </p>
                  </button>
                );
              })}
            </div>
          </div>

          {/* Title */}
          <div>
            <label className="block text-sm font-semibold text-gray-900 mb-2">
              Proposal Title
            </label>
            <input
              type="text"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              placeholder="e.g., Reduce transaction fees by 10%"
              className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-black focus:border-transparent"
              required
            />
          </div>

          {/* Description */}
          <div>
            <label className="block text-sm font-semibold text-gray-900 mb-2">
              Description
            </label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="Explain your proposal in detail. Include rationale, expected impact, and implementation details..."
              rows={6}
              className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-black focus:border-transparent resize-none"
              required
            />
          </div>

          {/* Info Box */}
          <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
            <p className="text-blue-800 font-semibold mb-2">Voting Parameters</p>
            <ul className="text-blue-700 text-sm space-y-1">
              <li>• Voting period: 7 days</li>
              <li>• Quorum required: 10% of total supply</li>
              <li>• Passing threshold: 51% yes votes</li>
              <li>• Your voting power: {userTokens.toLocaleString()} AFRI</li>
            </ul>
          </div>

          {/* Actions */}
          <div className="flex gap-3 pt-4">
            <button
              type="button"
              onClick={onClose}
              className="flex-1 px-6 py-3 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors font-semibold"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={loading || !canCreateProposal}
              className="flex-1 px-6 py-3 bg-gray-900 text-white rounded-lg hover:bg-gray-800 transition-colors font-semibold disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {loading ? 'Creating...' : 'Create Proposal'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
