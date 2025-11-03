import React, { useState, useEffect } from 'react';
import { Shield, Check, X, Eye, FileText, Calendar, User, DollarSign, AlertCircle } from 'lucide-react';
import PageLayout from '../../components/PageLayout';
import { KYCService, KYCSubmission } from '../../services/kycService';
import { UserService } from '../../services/userService';
import { AgentService } from '../../services/agentService';


const KYCAdmin: React.FC = () => {
  const [submissions, setSubmissions] = useState<KYCSubmission[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedSubmission, setSelectedSubmission] = useState<KYCSubmission | null>(null);
  const [reviewNotes, setReviewNotes] = useState('');
  const [rejectionReason, setRejectionReason] = useState('');
  const [processing, setProcessing] = useState(false);
  
  // Agent balance management state
  const [initializingBalances, setInitializingBalances] = useState(false);
  const [balanceResult, setBalanceResult] = useState<{success: boolean; updated: number; errors: string[]} | null>(null);

  useEffect(() => {
    loadSubmissions();
  }, []);

  const loadSubmissions = async () => {
    setLoading(true);
    try {
      const allSubmissions = await KYCService.getAllSubmissions();
      setSubmissions(allSubmissions);
    } catch (error) {
      console.error('Failed to load KYC submissions:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleInitializeAgentBalances = async () => {
    setInitializingBalances(true);
    setBalanceResult(null);
    try {
      console.log('🏦 Admin: Initializing all agent cash balances...');
      const result = await AgentService.initializeAllAgentsCashBalance();
      setBalanceResult(result);
      console.log('📊 Balance initialization result:', result);
    } catch (error) {
      console.error('Failed to initialize agent balances:', error);
      setBalanceResult({
        success: false,
        updated: 0,
        errors: [`Failed to initialize balances: ${error}`]
      });
    } finally {
      setInitializingBalances(false);
    }
  };

  const handleReview = async (submissionId: string, status: 'approved' | 'rejected') => {
    setProcessing(true);
    try {
      const result = await KYCService.reviewKYC(submissionId, {
        submissionId,
        status,
        reviewedBy: 'admin', // In production, use actual admin user ID
        notes: reviewNotes,
        rejectionReason: status === 'rejected' ? rejectionReason : undefined,
      });

      if (result.success) {
        // Update user's KYC status in main user record
        const submission = submissions.find(s => s.id === submissionId);
        if (submission) {
          await UserService.updateUser(submission.userId, {
            kycStatus: status,
          });
        }

        // Reload submissions
        await loadSubmissions();
        setSelectedSubmission(null);
        setReviewNotes('');
        setRejectionReason('');
        
        alert(`KYC ${status} successfully!`);
      } else {
        alert(`Failed to ${status} KYC: ${result.error}`);
      }
    } catch (error) {
      console.error(`Failed to ${status} KYC:`, error);
      alert(`Failed to ${status} KYC. Please try again.`);
    } finally {
      setProcessing(false);
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'pending': return 'bg-yellow-100 text-yellow-800';
      case 'approved': return 'bg-green-100 text-green-800';
      case 'rejected': return 'bg-red-100 text-red-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'pending': return <Calendar className="w-4 h-4" />;
      case 'approved': return <Check className="w-4 h-4" />;
      case 'rejected': return <X className="w-4 h-4" />;
      default: return <Shield className="w-4 h-4" />;
    }
  };

  if (loading) {
    return (
      <PageLayout>
        <div className="max-w-6xl mx-auto p-6">
          <div className="text-center py-12">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-neutral-900 mx-auto"></div>
            <p className="mt-4 text-neutral-600">Loading KYC submissions...</p>
          </div>
        </div>
      </PageLayout>
    );
  }

  return (
    <PageLayout>
      <div className="max-w-6xl mx-auto p-6">
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-neutral-900 mb-2">Admin Dashboard</h1>
          <p className="text-neutral-600">Manage KYC verification submissions and agent balances</p>
        </div>

        {/* Agent Balance Management Section */}
        <div className="mb-8 bg-white rounded-xl shadow-sm border border-neutral-200 p-6">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h2 className="text-xl font-semibold text-neutral-900 flex items-center">
                <DollarSign className="w-5 h-5 mr-2" />
                Agent Balance Management
              </h2>
              <p className="text-neutral-600 text-sm">Initialize agent cash balances for withdrawal processing</p>
            </div>
            <button
              onClick={handleInitializeAgentBalances}
              disabled={initializingBalances}
              className="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 disabled:bg-neutral-300 disabled:cursor-not-allowed flex items-center"
            >
              {initializingBalances ? (
                <>
                  <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
                  Initializing...
                </>
              ) : (
                <>
                  <DollarSign className="w-4 h-4 mr-2" />
                  Initialize All Agent Balances
                </>
              )}
            </button>
          </div>

          {balanceResult && (
            <div className={`p-4 rounded-lg ${balanceResult.success ? 'bg-green-50 border border-green-200' : 'bg-red-50 border border-red-200'}`}>
              {balanceResult.success ? (
                <div className="flex items-start">
                  <Check className="w-5 h-5 text-green-500 mr-2 mt-0.5" />
                  <div>
                    <p className="text-green-800 font-medium">Balance initialization successful!</p>
                    <p className="text-green-700 text-sm">Updated {balanceResult.updated} agent(s) with cash balance of 25,000,000 UGX</p>
                  </div>
                </div>
              ) : (
                <div className="flex items-start">
                  <AlertCircle className="w-5 h-5 text-red-500 mr-2 mt-0.5" />
                  <div>
                    <p className="text-red-800 font-medium">Errors occurred during initialization:</p>
                    <ul className="text-red-700 text-sm mt-1 list-disc list-inside">
                      {balanceResult.errors.map((error, index) => (
                        <li key={index}>{error}</li>
                      ))}
                    </ul>
                    {balanceResult.updated > 0 && (
                      <p className="text-red-700 text-sm mt-2">Successfully updated {balanceResult.updated} agent(s) before errors occurred.</p>
                    )}
                  </div>
                </div>
              )}
            </div>
          )}

          <div className="mt-4 p-4 bg-orange-50 border border-orange-200 rounded-lg">
            <div className="flex items-start">
              <AlertCircle className="w-5 h-5 text-orange-500 mr-2 mt-0.5" />
              <div>
                <p className="text-orange-800 font-medium">How Agent Cash Balances Work:</p>
                <ul className="text-orange-700 text-sm mt-1 list-disc list-inside space-y-1">
                  <li>Agents need cash balance to process customer withdrawals</li>
                  <li>When agents give cash to customers, their cash balance decreases</li>
                  <li>When customers deposit with agents, agent cash balance increases</li>
                  <li>Initial balance is set from VITE_AGENT_INITIAL_CASH_BALANCE environment variable (currently 25,000,000 UGX)</li>
                  <li>This button sets all existing agents to the environment variable amount</li>
                </ul>
              </div>
            </div>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* Submissions List */}
          <div className="lg:col-span-2">
            <div className="bg-white rounded-xl shadow-sm border border-neutral-200">
              <div className="p-6 border-b border-neutral-200">
                <h2 className="text-xl font-semibold text-neutral-900">
                  KYC Submissions ({submissions.length})
                </h2>
              </div>
              
              <div className="divide-y divide-neutral-200">
                {submissions.length === 0 ? (
                  <div className="p-8 text-center">
                    <Shield className="w-12 h-12 text-neutral-400 mx-auto mb-4" />
                    <p className="text-neutral-600">No KYC submissions found</p>
                  </div>
                ) : (
                  submissions.map((submission) => (
                    <div
                      key={submission.id}
                      className={`p-6 cursor-pointer hover:bg-neutral-50 transition-colors ${
                        selectedSubmission?.id === submission.id ? 'bg-blue-50 border-l-4 border-blue-500' : ''
                      }`}
                      onClick={() => setSelectedSubmission(submission)}
                    >
                      <div className="flex items-center justify-between mb-3">
                        <div className="flex items-center space-x-3">
                          <User className="w-5 h-5 text-neutral-600" />
                          <div>
                            <h3 className="font-semibold text-neutral-900">
                              {submission.firstName} {submission.lastName}
                            </h3>
                            <p className="text-sm text-neutral-600">{submission.phoneNumber}</p>
                          </div>
                        </div>
                        <div className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(submission.status)}`}>
                          {getStatusIcon(submission.status)}
                          <span className="ml-1 capitalize">{submission.status}</span>
                        </div>
                      </div>
                      
                      <div className="flex items-center justify-between text-sm text-neutral-600">
                        <span>Document: {submission.documentType.replace('_', ' ')}</span>
                        <span>{new Date(submission.submittedAt).toLocaleDateString()}</span>
                      </div>
                      
                      {submission.documentFileName && (
                        <div className="mt-2 flex items-center text-sm text-blue-600">
                          <FileText className="w-4 h-4 mr-1" />
                          {submission.documentFileName}
                        </div>
                      )}
                    </div>
                  ))
                )}
              </div>
            </div>
          </div>

          {/* Review Panel */}
          <div className="lg:col-span-1">
            {selectedSubmission ? (
              <div className="bg-white rounded-xl shadow-sm border border-neutral-200 sticky top-6">
                <div className="p-6 border-b border-neutral-200">
                  <h3 className="text-lg font-semibold text-neutral-900">Review Submission</h3>
                </div>
                
                <div className="p-6 space-y-4">
                  <div>
                    <label className="block text-sm font-medium text-neutral-700 mb-1">
                      Applicant
                    </label>
                    <p className="text-neutral-900">
                      {selectedSubmission.firstName} {selectedSubmission.lastName}
                    </p>
                    <p className="text-sm text-neutral-600">{selectedSubmission.phoneNumber}</p>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-neutral-700 mb-1">
                      Document Type
                    </label>
                    <p className="text-neutral-900 capitalize">
                      {selectedSubmission.documentType.replace('_', ' ')}
                    </p>
                  </div>

                  {selectedSubmission.documentNumber && (
                    <div>
                      <label className="block text-sm font-medium text-neutral-700 mb-1">
                        Document Number
                      </label>
                      <p className="text-neutral-900 font-mono">{selectedSubmission.documentNumber}</p>
                    </div>
                  )}

                  {selectedSubmission.documentFileUrl && (
                    <div>
                      <label className="block text-sm font-medium text-neutral-700 mb-1">
                        Document Image
                      </label>
                      <a
                        href={selectedSubmission.documentFileUrl}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="inline-flex items-center px-3 py-2 border border-neutral-300 rounded-md text-sm text-neutral-700 hover:bg-neutral-50"
                      >
                        <Eye className="w-4 h-4 mr-2" />
                        View Document
                      </a>
                    </div>
                  )}

                  <div>
                    <label className="block text-sm font-medium text-neutral-700 mb-1">
                      Submitted
                    </label>
                    <p className="text-neutral-900">
                      {new Date(selectedSubmission.submittedAt).toLocaleString()}
                    </p>
                  </div>

                  {selectedSubmission.status === 'pending' && (
                    <div className="space-y-4 pt-4 border-t border-neutral-200">
                      <div>
                        <label className="block text-sm font-medium text-neutral-700 mb-2">
                          Review Notes
                        </label>
                        <textarea
                          value={reviewNotes}
                          onChange={(e) => setReviewNotes(e.target.value)}
                          className="w-full px-3 py-2 border border-neutral-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                          rows={3}
                          placeholder="Add any notes about this review..."
                        />
                      </div>

                      <div>
                        <label className="block text-sm font-medium text-neutral-700 mb-2">
                          Rejection Reason (if rejecting)
                        </label>
                        <input
                          type="text"
                          value={rejectionReason}
                          onChange={(e) => setRejectionReason(e.target.value)}
                          className="w-full px-3 py-2 border border-neutral-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                          placeholder="Reason for rejection..."
                        />
                      </div>

                      <div className="flex space-x-3">
                        <button
                          onClick={() => handleReview(selectedSubmission.id, 'approved')}
                          disabled={processing}
                          className="flex-1 bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center"
                        >
                          <Check className="w-4 h-4 mr-2" />
                          Approve
                        </button>
                        <button
                          onClick={() => handleReview(selectedSubmission.id, 'rejected')}
                          disabled={processing || !rejectionReason.trim()}
                          className="flex-1 bg-red-600 text-white px-4 py-2 rounded-md hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center"
                        >
                          <X className="w-4 h-4 mr-2" />
                          Reject
                        </button>
                      </div>
                    </div>
                  )}

                  {selectedSubmission.status !== 'pending' && (
                    <div className="pt-4 border-t border-neutral-200">
                      <div className="space-y-2">
                        <p className="text-sm text-neutral-600">
                          <strong>Status:</strong> {selectedSubmission.status}
                        </p>
                        {selectedSubmission.reviewedAt && (
                          <p className="text-sm text-neutral-600">
                            <strong>Reviewed:</strong> {new Date(selectedSubmission.reviewedAt).toLocaleString()}
                          </p>
                        )}
                        {selectedSubmission.reviewedBy && (
                          <p className="text-sm text-neutral-600">
                            <strong>Reviewed by:</strong> {selectedSubmission.reviewedBy}
                          </p>
                        )}
                        {selectedSubmission.notes && (
                          <p className="text-sm text-neutral-600">
                            <strong>Notes:</strong> {selectedSubmission.notes}
                          </p>
                        )}
                        {selectedSubmission.rejectionReason && (
                          <p className="text-sm text-red-600">
                            <strong>Rejection reason:</strong> {selectedSubmission.rejectionReason}
                          </p>
                        )}
                      </div>
                    </div>
                  )}
                </div>
              </div>
            ) : (
              <div className="bg-white rounded-xl shadow-sm border border-neutral-200 p-8 text-center">
                <Shield className="w-12 h-12 text-neutral-400 mx-auto mb-4" />
                <p className="text-neutral-600">Select a submission to review</p>
              </div>
            )}
          </div>
        </div>
      </div>
    </PageLayout>
  );
};

export default KYCAdmin;
