import {
  Home,
  Send,
  History,
  User,
  MapPin,
  Vote,
  Trophy,
  LayoutDashboard,
  Banknote,
} from "@lucide/svelte";

export interface Route {
  id: string;
  path: string;
  label: string;
  icon: any;
}

export const user_desktop_routes: Route[] = [
  {
    id: "home",
    path: "/users/dashboard",
    label: "Dashboard",
    icon: LayoutDashboard,
  },
  {
    id: "send",
    path: "/users/send",
    label: "Send",
    icon: Send,
  },
  {
    id: "withdraw",
    path: "/users/withdraw",
    label: "Withdraw",
    icon: Banknote,
  },
  {
    id: "dao",
    path: "/users/dao",
    label: "DAO Governance",
    icon: Vote,
  },
  {
    id: "leaderboard",
    path: "/users/leaderboard",
    label: "Leaderboard",
    icon: Trophy,
  },
  {
    id: "agents",
    path: "/users/agents",
    label: "Find Agents",
    icon: MapPin,
  },
  {
    id: "history",
    path: "/users/history",
    label: "Transaction History",
    icon: History,
  },
  {
    id: "profile",
    path: "/users/profile",
    label: "Profile & Settings",
    icon: User,
  },
];

export const user_mobile_routes: Route[] = [
  {
    id: "home",
    path: "/users/dashboard",
    label: "Home",
    icon: Home,
  },
  {
    id: "send",
    path: "/users/send",
    label: "Send",
    icon: Send,
  },
  {
    id: "withdraw",
    path: "/users/withdraw",
    label: "Withdraw",
    icon: Banknote,
  },
  {
    id: "dao",
    path: "/users/dao",
    label: "DAO",
    icon: Vote,
  },
  {
    id: "agents",
    path: "/users/agents",
    label: "Agents",
    icon: MapPin,
  },
  {
    id: "history",
    path: "/users/history",
    label: "History",
    icon: History,
  },
  {
    id: "profile",
    path: "/users/profile",
    label: "Profile",
    icon: User,
  },
];
