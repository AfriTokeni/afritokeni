import {
  LayoutDashboard,
  ShieldCheck,
  TrendingUp,
  Receipt,
  Users,
  MapPin,
  Activity,
} from "@lucide/svelte";

export interface Route {
  id: string;
  path: string;
  label: string;
  icon: any;
}

export const admin_desktop_routes: Route[] = [
  {
    id: "overview",
    path: "/admin/dashboard",
    label: "Overview",
    icon: LayoutDashboard,
  },
  {
    id: "kyc",
    path: "/admin/kyc",
    label: "KYC Management",
    icon: ShieldCheck,
  },
  {
    id: "revenue",
    path: "/admin/revenue",
    label: "Revenue",
    icon: TrendingUp,
  },
  {
    id: "transactions",
    path: "/admin/transactions",
    label: "Transactions",
    icon: Receipt,
  },
  {
    id: "users",
    path: "/admin/users",
    label: "Users",
    icon: Users,
  },
  {
    id: "agents",
    path: "/admin/agents",
    label: "Agents",
    icon: MapPin,
  },
  {
    id: "system",
    path: "/admin/system",
    label: "System Health",
    icon: Activity,
  },
];
