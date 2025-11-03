import type { LucideIcon } from 'lucide-react';
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
} from 'lucide-react';
import { RouteID } from '../types/routes';


export interface Route {
  id: string;
  path: string;
  label: string;
  icon: LucideIcon;
}

export const user_desktop_routes: Route[] = [
  {
    id: "home" as RouteID,
    path: "/users/dashboard",
    label: "Dashboard",
    icon: LayoutDashboard,
  },
    {
        id: "send" as RouteID,
        path: "/users/send",
        label: "Send",
        icon: Send,
    },
    {
        id: "withdraw" as RouteID,
        path: "/users/withdraw",
        label: "Withdraw",
        icon: Banknote,
    },
    {
        id: "dao" as RouteID,
        path: "/users/dao",
        label: "DAO Governance",
        icon: Vote,
    },
    {
        id: "leaderboard" as RouteID,
        path: "/users/leaderboard",
        label: "Leaderboard",
        icon: Trophy,
    },
    {
        id: "agents" as RouteID,
        path: "/users/agents",
        label: "Find Agents",
        icon: MapPin,
    },
    {
        id: "history" as RouteID,
        path: "/users/history",
        label: "Transaction History",
        icon: History,
    },
    {
        id: "profile" as RouteID,
        path: "/users/profile",
        label: "Profile & Settings",
        icon: User,
    },
];

export const user_mobile_routes: Route[] = [
  {
    id: "home" as RouteID,
    path: "/users/dashboard",
    label: "Home",
    icon: Home,
  },
  {
    id: "send" as RouteID,
    path: "/users/send",
    label: "Send",
    icon: Send,
  },
  {
    id: "withdraw" as RouteID,
    path: "/users/withdraw",
    label: "Withdraw",
    icon: Banknote,
  },
  {
    id: "dao" as RouteID,
    path: "/users/dao",
    label: "DAO",
    icon: Vote,
  },
  {
    id: "agents" as RouteID,
    path: "/users/agents",
    label: "Agents",
    icon: MapPin,
  },
  {
    id: "history" as RouteID,
    path: "/users/history",
    label: "History",
    icon: History,
  },
  {
    id: "profile" as RouteID,
    path: "/users/profile",
    label: "Profile",
    icon: User,
  },
]
