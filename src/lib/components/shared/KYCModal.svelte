<!--
 * KYC Verification Modal
 * Allows users to start KYC verification process
 -->
<script lang="ts">
  import { AlertCircle, Camera, FileText, Upload, X } from "lucide-svelte";
  import { toast } from "$lib/stores/toast";

  interface Props {
    isOpen: boolean;
    onClose: () => void;
    onSubmit: (kycData: any) => Promise<void>;
  }

  let { isOpen, onClose, onSubmit }: Props = $props();

  // Form state
  let documentType = $state<"national_id" | "passport" | "drivers_license">(
    "national_id",
  );
  let documentNumber = $state("");
  let documentFront = $state<File | null>(null);
  let documentBack = $state<File | null>(null);
  let selfie = $state<File | null>(null);
  let isSubmitting = $state(false);

  function handleFileChange(event: Event, type: "front" | "back" | "selfie") {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];

    if (file) {
      // Validate file size (max 5MB)
      if (file.size > 5 * 1024 * 1024) {
        toast.show("error", "File size must be less than 5MB");
        return;
      }

      // Validate file type
      if (!file.type.startsWith("image/")) {
        toast.show("error", "Only image files are allowed");
        return;
      }

      if (type === "front") documentFront = file;
      else if (type === "back") documentBack = file;
      else selfie = file;
    }
  }

  async function handleSubmit() {
    // Validation
    if (!documentNumber) {
      toast.show("warning", "Please enter your document number");
      return;
    }

    if (!documentFront) {
      toast.show("warning", "Please upload front of document");
      return;
    }

    if (documentType === "national_id" && !documentBack) {
      toast.show("warning", "Please upload back of national ID");
      return;
    }

    if (!selfie) {
      toast.show("warning", "Please upload a selfie");
      return;
    }

    isSubmitting = true;
    try {
      const kycData = {
        documentType,
        documentNumber,
        documentFront,
        documentBack,
        selfie,
        submittedAt: new Date().toISOString(),
      };

      await onSubmit(kycData);
      toast.show(
        "success",
        "KYC documents submitted! Review takes 24-48 hours.",
      );
      onClose();

      // Reset form
      documentType = "national_id";
      documentNumber = "";
      documentFront = null;
      documentBack = null;
      selfie = null;
    } catch (error: any) {
      toast.show("error", error.message || "Failed to submit KYC documents");
    } finally {
      isSubmitting = false;
    }
  }

  function handleClose() {
    if (!isSubmitting) {
      onClose();
    }
  }
</script>

{#if isOpen}
  <div
    class="bg-opacity-50 fixed inset-0 z-50 flex items-center justify-center bg-black p-4"
  >
    <div
      class="max-h-[90vh] w-full max-w-2xl overflow-y-auto rounded-2xl bg-white"
    >
      <!-- Header -->
      <div
        class="sticky top-0 flex items-center justify-between border-b border-gray-200 bg-white p-6"
      >
        <div>
          <h2 class="text-2xl font-bold text-gray-900">KYC Verification</h2>
          <p class="mt-1 text-sm text-gray-600">
            Upload your documents for verification
          </p>
        </div>
        <button
          aria-label="Toggle"
          onclick={handleClose}
          disabled={isSubmitting}
          class="rounded-lg p-2 transition-colors hover:bg-gray-100 disabled:opacity-50"
          type="button"
        >
          <X class="h-5 w-5" />
        </button>
      </div>

      <!-- Content -->
      <div class="space-y-6 p-6">
        <!-- Info Banner -->
        <div
          class="flex gap-3 rounded-lg border border-blue-200 bg-blue-50 p-4"
        >
          <AlertCircle class="mt-0.5 h-5 w-5 shrink-0 text-blue-600" />
          <div class="text-sm text-blue-800">
            <p class="mb-1 font-semibold">Required for full access</p>
            <p>
              KYC verification is required to unlock all features including
              higher transaction limits and agent services. Review typically
              takes 24-48 hours.
            </p>
          </div>
        </div>

        <!-- Document Type -->
        <div>
          <label
            for="documentType"
            class="mb-2 block text-sm font-medium text-gray-700"
          >
            Document Type *
          </label>
          <select
            id="documentType"
            bind:value={documentType}
            class="w-full rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-purple-600"
          >
            <option value="national_id">National ID</option>
            <option value="passport">Passport</option>
            <option value="drivers_license">Driver's License</option>
          </select>
        </div>

        <!-- Document Number -->
        <div>
          <label
            for="documentNumber"
            class="mb-2 block text-sm font-medium text-gray-700"
          >
            Document Number *
          </label>
          <input
            id="documentNumber"
            type="text"
            bind:value={documentNumber}
            placeholder="Enter your document number"
            class="w-full rounded-lg border border-gray-300 px-4 py-3 focus:border-transparent focus:ring-2 focus:ring-purple-600"
          />
        </div>

        <!-- Document Front -->
        <div>
          <span class="mb-2 block text-sm font-medium text-gray-700">
            Document Front * {documentFront ? "✓" : ""}
          </span>
          <label
            class="flex h-32 w-full cursor-pointer flex-col items-center justify-center rounded-lg border-2 border-dashed border-gray-300 transition-colors hover:bg-gray-50"
          >
            <div class="flex flex-col items-center justify-center pt-5 pb-6">
              <Upload class="mb-2 h-8 w-8 text-gray-400" />
              <p class="text-sm text-gray-600">
                {documentFront
                  ? documentFront.name
                  : "Click to upload front of document"}
              </p>
              <p class="mt-1 text-xs text-gray-500">PNG, JPG up to 5MB</p>
            </div>
            <input
              type="file"
              accept="image/*"
              onchange={(e) => handleFileChange(e, "front")}
              class="hidden"
            />
          </label>
        </div>

        <!-- Document Back (for National ID) -->
        {#if documentType === "national_id"}
          <div>
            <span class="mb-2 block text-sm font-medium text-gray-700">
              Document Back * {documentBack ? "✓" : ""}
            </span>
            <label
              class="flex h-32 w-full cursor-pointer flex-col items-center justify-center rounded-lg border-2 border-dashed border-gray-300 transition-colors hover:bg-gray-50"
            >
              <div class="flex flex-col items-center justify-center pt-5 pb-6">
                <Upload class="mb-2 h-8 w-8 text-gray-400" />
                <p class="text-sm text-gray-600">
                  {documentBack
                    ? documentBack.name
                    : "Click to upload back of document"}
                </p>
                <p class="mt-1 text-xs text-gray-500">PNG, JPG up to 5MB</p>
              </div>
              <input
                type="file"
                accept="image/*"
                onchange={(e) => handleFileChange(e, "back")}
                class="hidden"
              />
            </label>
          </div>
        {/if}

        <!-- Selfie -->
        <div>
          <span class="mb-2 block text-sm font-medium text-gray-700">
            Selfie with Document * {selfie ? "✓" : ""}
          </span>
          <label
            class="flex h-32 w-full cursor-pointer flex-col items-center justify-center rounded-lg border-2 border-dashed border-gray-300 transition-colors hover:bg-gray-50"
          >
            <div class="flex flex-col items-center justify-center pt-5 pb-6">
              <Camera class="mb-2 h-8 w-8 text-gray-400" />
              <p class="text-sm text-gray-600">
                {selfie
                  ? selfie.name
                  : "Click to upload selfie holding document"}
              </p>
              <p class="mt-1 text-xs text-gray-500">PNG, JPG up to 5MB</p>
            </div>
            <input
              type="file"
              accept="image/*"
              onchange={(e) => handleFileChange(e, "selfie")}
              class="hidden"
            />
          </label>
        </div>
      </div>

      <!-- Footer -->
      <div class="flex gap-3 border-t border-gray-200 p-6">
        <button
          onclick={handleClose}
          disabled={isSubmitting}
          class="flex-1 rounded-lg border border-gray-300 px-6 py-3 font-semibold text-gray-700 transition-colors hover:bg-gray-50 disabled:opacity-50"
        >
          Cancel
        </button>

        <button
          onclick={handleSubmit}
          disabled={isSubmitting}
          class="flex flex-1 items-center justify-center gap-2 rounded-lg bg-purple-600 px-6 py-3 font-semibold text-white transition-colors hover:bg-purple-700 disabled:opacity-50"
        >
          {#if isSubmitting}
            <div
              class="h-5 w-5 animate-spin rounded-full border-2 border-white border-t-transparent"
            ></div>
            Submitting...
          {:else}
            <FileText class="h-5 w-5" />
            Submit for Review
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}
