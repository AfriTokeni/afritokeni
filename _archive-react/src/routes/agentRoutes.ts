import {
  Users,
  CreditCard,
  Settings,
  LucideIcon,
  Banknote,
  Home,
  LayoutDashboard,
  Vote,
  Bitcoin,
} from "lucide-react";

export interface Route {
  id: string;
  path: string;
  label: string;
  icon: LucideIcon;
}

export const agent_desktop_routes: Route[] = [
  {
    id: "home",
    path: "/agents/dashboard",
    label: "Dashboard",
    icon: LayoutDashboard,
  },
  {
    id: "process-deposits",
    path: "/agents/process-deposits",
    label: "Process Deposits",
    icon: CreditCard,
  },
  {
    id: "process-withdrawals",
    path: "/agents/process-withdrawals",
    label: "Process Withdrawals",
    icon: Banknote,
  },
  {
    id: "exchanges",
    path: "/agents/exchanges",
    label: "Exchanges",
    icon: Bitcoin,
  },
  {
    id: "dao",
    path: "/agents/dao",
    label: "DAO Governance",
    icon: Vote,
  },
  {
    id: "customers",
    path: "/agents/customers",
    label: "Customers",
    icon: Users,
  },
  {
    id: "settings",
    path: "/agents/settings",
    label: "Settings",
    icon: Settings,
  },
];

export const agent_mobile_routes: Route[] = [
  {
    id: "home",
    path: "/agents/dashboard",
    label: "Home",
    icon: Home,
  },
  {
    id: "process-deposits",
    path: "/agents/process-deposits",
    label: "Deposits",
    icon: CreditCard,
  },
  {
    id: "process-withdrawals",
    path: "/agents/process-withdrawals",
    label: "Withdrawals",
    icon: Banknote,
  },
  {
    id: "exchanges",
    path: "/agents/exchanges",
    label: "Exchanges",
    icon: Bitcoin,
  },
  {
    id: "dao",
    path: "/agents/dao",
    label: "DAO",
    icon: Vote,
  },
  {
    id: "customers",
    path: "/agents/customers",
    label: "Customers",
    icon: Users,
  },
  {
    id: "settings",
    path: "/agents/settings",
    label: "Settings",
    icon: Settings,
  },
];
