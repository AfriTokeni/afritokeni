import { setDoc, uploadFile, listDocs, getDoc } from '@junobuild/core';
import { nanoid } from 'nanoid';
import { NotificationService } from './notificationService';

import { UserKYCData } from '../types/auth';
import { UserService } from './userService';

export interface KYCSubmission {
  id: string;
  userId: string;
  firstName: string;
  lastName: string;
  phoneNumber: string;
  documentType: string;
  documentNumber?: string;
  documentFileUrl?: string;
  documentFileName?: string;
  status: 'pending' | 'approved' | 'rejected';
  submittedAt: string;
  reviewedAt?: string;
  adminNotes?: string;
  reviewedBy?: string;
  rejectionReason?: string;
  notes?: string;
}

export interface KYCReview {
  submissionId: string;
  status: 'approved' | 'rejected';
  reviewedBy: string;
  notes?: string;
  rejectionReason?: string;
}

export class KYCService {
  private static readonly KYC_COLLECTION = 'kyc_submissions';
  private static readonly DOCUMENTS_COLLECTION = 'kyc_documents';

  /**
   * Submit KYC data with optional document upload
   */
  static async submitKYC(userId: string, data: UserKYCData): Promise<{ success: boolean; submissionId?: string; error?: string }> {
    try {
      const submissionId = nanoid();
      let documentFileUrl: string | undefined;
      let documentFileName: string | undefined;

      // Upload document if provided
      if (data.documentFile) {
        try {
          const uploadResult = await uploadFile({
            data: data.documentFile,
            collection: this.DOCUMENTS_COLLECTION,
            filename: `${userId}_${submissionId}_${data.documentFile.name || 'document'}`,
          });
          
          documentFileUrl = uploadResult.downloadUrl;
          documentFileName = data.documentFile.name;
        } catch (uploadError) {
          console.error('Failed to upload KYC document:', uploadError);
          return { success: false, error: 'Failed to upload document' };
        }
      }

      // Create KYC submission record
      const submission: KYCSubmission = {
        id: submissionId,
        userId,
        firstName: data.firstName,
        lastName: data.lastName,
        phoneNumber: data.phoneNumber,
        documentType: data.documentType || 'national_id',
        documentNumber: data.documentNumber || undefined,
        documentFileUrl,
        documentFileName,
        status: 'pending',
        submittedAt: new Date().toISOString(),
      };

      // Store submission in Juno datastore
      await setDoc({
        collection: this.KYC_COLLECTION,
        doc: {
          key: submissionId,
          data: submission,
        },
      });

      // Send notification to user about KYC submission
      try {
        const user = await UserService.getUserByKey(userId);
        if (user) {
          await NotificationService.sendNotification(user, {
            userId,
            type: 'kyc_update',
            status: 'submitted',
            message: 'Your KYC verification has been submitted and is under review.'
          });
        }
      } catch (notificationError) {
        console.error('Failed to send KYC submission notification:', notificationError);
      }

      return { success: true, submissionId };
    } catch (error) {
      console.error('KYC submission failed:', error);
      return { success: false, error: 'Failed to submit KYC' };
    }
  }

  /**
   * Get KYC submission by user ID
   */
  static async getKYCByUserId(userId: string): Promise<KYCSubmission | null> {
    try {
      const docs = await listDocs({
        collection: this.KYC_COLLECTION,
      });

      if (docs.items.length === 0) return null;

      // Filter by userId and get the most recent submission
      const submissions = docs.items
        .map(doc => doc.data as KYCSubmission)
        .filter(submission => submission.userId === userId)
        .sort((a, b) => new Date(b.submittedAt).getTime() - new Date(a.submittedAt).getTime());

      return submissions.length > 0 ? submissions[0] : null;
    } catch (error) {
      console.error('Failed to get KYC submission:', error);
      return null;
    }
  }

  /**
   * Get all pending KYC submissions for admin review
   */
  static async getPendingSubmissions(): Promise<KYCSubmission[]> {
    try {
      const docs = await listDocs({
        collection: this.KYC_COLLECTION,
      });

      return docs.items
        .map(doc => doc.data as KYCSubmission)
        .filter(submission => submission.status === 'pending')
        .sort((a, b) => new Date(a.submittedAt).getTime() - new Date(b.submittedAt).getTime());
    } catch (error) {
      console.error('Failed to get pending KYC submissions:', error);
      return [];
    }
  }

  /**
   * Review and approve/reject KYC submission
   */
  static async reviewKYC(submissionId: string, review: KYCReview): Promise<{ success: boolean; error?: string }> {
    try {
      // Get existing submission
      const doc = await getDoc({
        collection: this.KYC_COLLECTION,
        key: submissionId,
      });

      if (!doc) {
        return { success: false, error: 'KYC submission not found' };
      }

      const submission = doc.data as KYCSubmission;

      // Update submission with review
      const updatedSubmission: KYCSubmission = {
        ...submission,
        status: review.status,
        reviewedAt: new Date().toISOString(),
        reviewedBy: review.reviewedBy,
        notes: review.notes,
        rejectionReason: review.rejectionReason,
      };

      // Update in datastore
      await setDoc({
        collection: this.KYC_COLLECTION,
        doc: {
          key: submissionId,
          data: updatedSubmission,
        },
      });

      return { success: true };
    } catch (error) {
      console.error('Failed to review KYC:', error);
      return { success: false, error: 'Failed to review KYC submission' };
    }
  }

  /**
   * Update KYC status (admin function)
   */
  static async updateKYCStatus(
    submissionId: string,
    status: 'approved' | 'rejected',
    adminNotes?: string
  ): Promise<{ success: boolean; error?: string }> {
    try {
      const doc = await getDoc({
        collection: this.KYC_COLLECTION,
        key: submissionId,
      });

      if (!doc) {
        return { success: false, error: 'KYC submission not found' };
      }

      const submission = doc.data as KYCSubmission;
      const updatedSubmission: KYCSubmission = {
        ...submission,
        status,
        adminNotes,
        reviewedAt: new Date().toISOString(),
      };

      await setDoc({
        collection: this.KYC_COLLECTION,
        doc: {
          key: submissionId,
          data: updatedSubmission,
        },
      });

      // Send notification to user about KYC status update
      try {
        const user = await UserService.getUserByKey(submission.userId);
        if (user) {
          const message = status === 'approved' 
            ? 'Congratulations! Your KYC verification has been approved. You can now access all platform features.'
            : `Your KYC verification was not approved. ${adminNotes || 'Please contact support for more information.'}`;

          await NotificationService.sendNotification(user, {
            userId: submission.userId,
            type: 'kyc_update',
            status,
            message
          });
        }
      } catch (notificationError) {
        console.error('Failed to send KYC status notification:', notificationError);
      }

      return { success: true };
    } catch (error) {
      console.error('Failed to update KYC status:', error);
      return { success: false, error: 'Failed to update KYC status' };
    }
  }

  /**
   * Get KYC submission by ID
   */
  static async getKYCSubmission(submissionId: string): Promise<KYCSubmission | null> {
    try {
      const doc = await getDoc({
        collection: this.KYC_COLLECTION,
        key: submissionId,
      });

      return doc ? (doc.data as KYCSubmission) : null;
    } catch (error) {
      console.error('Failed to get KYC submission:', error);
      return null;
    }
  }

  /**
   * Get all KYC submissions for admin dashboard
   */
  static async getAllSubmissions(): Promise<KYCSubmission[]> {
    try {
      const docs = await listDocs({
        collection: this.KYC_COLLECTION,
      });

      return docs.items
        .map(doc => doc.data as KYCSubmission)
        .sort((a, b) => new Date(b.submittedAt).getTime() - new Date(a.submittedAt).getTime());
    } catch (error) {
      console.error('Failed to get all KYC submissions:', error);
      return [];
    }
  }
}
