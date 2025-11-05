/**
 * KYC Service
 * Handles all KYC document operations with Juno DB
 */

import { listDocs, setDoc, getDoc } from "@junobuild/core";
import type { Doc } from "@junobuild/core";
import { toast } from "$lib/stores/toast";
import type {
  KYCDocument,
  KYCStats,
  KYCFilters,
  KYCStatus,
  KYCDocumentType,
} from "$lib/types/admin";

const COLLECTION = "kyc_documents";

/**
 * Juno document data structure for KYC
 */
export interface KYCDocData {
  userId: string;
  userName: string;
  userEmail: string;
  userPhone?: string;
  documentType: KYCDocumentType;
  documentNumber: string;
  status: KYCStatus;
  submittedAt: string;
  reviewedAt?: string;
  reviewedBy?: string;
  adminNotes?: string;
  documentUrl?: string;
  selfieUrl?: string;
  documents?: string[];
  location?: string;
  businessLicense?: string;
  approvedAt?: string;
  approvedBy?: string;
  rejectedAt?: string;
  rejectedBy?: string;
  reason?: string;
}

/**
 * List KYC documents with filters
 */
export async function listKYCDocuments(
  filters: KYCFilters = {},
): Promise<KYCDocument[]> {
  try {
    const { items } = await listDocs<KYCDocData>({
      collection: COLLECTION,
      filter: {},
    });

    let documents = items.map((doc) => docToKYC(doc));

    // Apply status filter
    if (filters.status && filters.status !== "all") {
      documents = documents.filter((doc) => doc.status === filters.status);
    }

    // Apply search filter
    if (filters.searchQuery) {
      const query = filters.searchQuery.toLowerCase();
      documents = documents.filter(
        (doc) =>
          doc.userName.toLowerCase().includes(query) ||
          doc.userEmail.toLowerCase().includes(query) ||
          doc.documentNumber.toLowerCase().includes(query),
      );
    }

    // Sort by submitted date (newest first)
    documents.sort(
      (a, b) =>
        new Date(b.submittedAt).getTime() - new Date(a.submittedAt).getTime(),
    );

    // Apply pagination
    const offset = filters.offset ?? 0;
    const limit = filters.limit ?? 20;
    return documents.slice(offset, offset + limit);
  } catch (error) {
    console.error("Error listing KYC documents:", error);
    toast.show("error", "Failed to load KYC documents");
    throw error;
  }
}

/**
 * Get KYC document by ID
 */
export async function getKYCDocument(id: string): Promise<KYCDocument> {
  if (!id) {
    throw new Error("KYC document ID is required");
  }

  try {
    const doc = await getDoc<KYCDocData>({
      collection: COLLECTION,
      key: id,
    });

    if (!doc) {
      throw new Error("KYC document not found");
    }

    return docToKYC(doc);
  } catch (error) {
    console.error("Error getting KYC document:", error);
    toast.show("error", "Failed to load KYC document");
    throw error;
  }
}

/**
 * Update KYC document status
 */
export async function updateKYCStatus(
  docId: string,
  status: KYCStatus,
  adminNotes?: string,
): Promise<void> {
  if (!docId) {
    throw new Error("KYC document ID is required");
  }

  if (!status) {
    throw new Error("Status is required");
  }

  try {
    const doc = await getDoc({
      collection: COLLECTION,
      key: docId,
    });

    if (!doc) {
      throw new Error("KYC document not found");
    }

    await setDoc({
      collection: COLLECTION,
      doc: {
        ...doc,
        data: {
          ...(doc.data as KYCDocData),
          status,
          reviewedAt: new Date().toISOString(),
          adminNotes: adminNotes ?? (doc.data as KYCDocData).adminNotes,
        },
      },
    });

    toast.show("success", `KYC document ${status}`);
  } catch (error) {
    console.error("Error updating KYC status:", error);
    toast.show("error", "Failed to update KYC status");
    throw error;
  }
}

/**
 * Get KYC statistics
 */
export async function getKYCStats(): Promise<KYCStats> {
  try {
    const { items } = await listDocs<KYCDocData>({
      collection: COLLECTION,
      filter: {},
    });

    const stats: KYCStats = {
      total: items.length,
      pending: 0,
      approved: 0,
      rejected: 0,
    };

    items.forEach((doc) => {
      const status = doc.data.status;
      if (status === "pending") stats.pending++;
      if (status === "approved") stats.approved++;
      if (status === "rejected") stats.rejected++;
    });

    return stats;
  } catch (error) {
    console.error("Error getting KYC stats:", error);
    toast.show("error", "Failed to load KYC statistics");
    throw error;
  }
}

/**
 * Convert Juno doc to KYCDocument
 */
function docToKYC(doc: Doc<KYCDocData>): KYCDocument {
  const data = doc.data;

  return {
    id: doc.key,
    userId: data.userId,
    userName: data.userName,
    userEmail: data.userEmail,
    userPhone: data.userPhone,
    documentType: data.documentType,
    documentNumber: data.documentNumber,
    status: data.status,
    submittedAt: data.submittedAt,
    reviewedAt: data.reviewedAt,
    reviewedBy: data.reviewedBy,
    adminNotes: data.adminNotes,
    documentUrl: data.documentUrl,
    selfieUrl: data.selfieUrl,
    documents: data.documents,
    location: data.location,
    businessLicense: data.businessLicense,
    approvedAt: data.approvedAt,
    approvedBy: data.approvedBy,
    rejectedAt: data.rejectedAt,
    rejectedBy: data.rejectedBy,
    reason: data.reason,
  };
}
