<script lang="ts">
  import { AlertTriangle, ArrowRight, Clock, XCircle } from "@lucide/svelte";
  import { goto } from "$app/navigation";

  interface Props {
    userType: "user" | "agent";
    kycStatus?: "not_started" | "pending" | "rejected" | "approved";
  }

  let { userType, kycStatus = "not_started" }: Props = $props();

  function getStatusConfig() {
    switch (kycStatus) {
      case "not_started":
        return {
          icon: AlertTriangle,
          bgColor: "bg-yellow-50",
          borderColor: "border-yellow-200",
          textColor: "text-yellow-800",
          title: "Complete Your Verification",
          message:
            "To use all features of AfriTokeni, please complete your identity verification.",
          actionText: "Start Verification",
          actionPath:
            userType === "user" ? "/users/user-kyc" : "/agents/agent-kyc",
          buttonBg: "bg-yellow-600 hover:bg-yellow-700",
          focusRing: "focus:ring-yellow-500",
        };
      case "pending":
        return {
          icon: Clock,
          bgColor: "bg-blue-50",
          borderColor: "border-blue-200",
          textColor: "text-blue-800",
          title: "Verification Under Review",
          message:
            "Your documents are being reviewed. We'll notify you within 24-48 hours.",
          actionText: null,
          actionPath: null,
          buttonBg: "",
          focusRing: "",
        };
      case "rejected":
        return {
          icon: XCircle,
          bgColor: "bg-red-50",
          borderColor: "border-red-200",
          textColor: "text-red-800",
          title: "Verification Rejected",
          message:
            "Your verification was rejected. Please update your documents and try again.",
          actionText: "Re-submit Documents",
          actionPath:
            userType === "user" ? "/users/user-kyc" : "/agents/agent-kyc",
          buttonBg: "bg-red-600 hover:bg-red-700",
          focusRing: "focus:ring-red-500",
        };
      case "approved":
        return null;
      default:
        return null;
    }
  }

  const statusConfig = getStatusConfig();

  function handleActionClick() {
    if (statusConfig?.actionPath) {
      goto(statusConfig.actionPath);
    }
  }
</script>

{#if statusConfig}
  {@const Icon = statusConfig.icon}
  <div
    class="{statusConfig.bgColor} {statusConfig.borderColor} mb-6 rounded-lg border p-4"
  >
    <div class="flex">
      <div class="shrink-0">
        <Icon
          class="h-5 w-5 {statusConfig.textColor.replace('text-', 'text-')}"
        />
      </div>
      <div class="ml-3 flex-1">
        <h3 class="text-sm font-medium {statusConfig.textColor}">
          {statusConfig.title}
        </h3>
        <div class="mt-2 text-sm {statusConfig.textColor}">
          <p>{statusConfig.message}</p>
        </div>
        {#if statusConfig.actionText && statusConfig.actionPath}
          <div class="mt-4">
            <button
              onclick={handleActionClick}
              class="inline-flex items-center rounded-md border border-transparent px-3 py-2 text-sm leading-4 font-medium text-white {statusConfig.buttonBg} focus:ring-2 focus:ring-offset-2 focus:outline-none {statusConfig.focusRing} transition-colors"
            >
              {statusConfig.actionText}
              <ArrowRight class="ml-2 h-4 w-4" />
            </button>
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}
