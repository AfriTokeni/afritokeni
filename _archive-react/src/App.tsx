import { initSatellite } from "@junobuild/core";
import { FC, useEffect } from "react";
import junoConfig from "../juno.config";
import { Route, Routes, BrowserRouter, Navigate } from "react-router-dom";
import { AuthenticationProvider } from "./context/AuthenticationContext.tsx";
import { DemoModeProvider } from "./context/DemoModeContext.tsx";
import ModernLayout from "./components/ModernLayout.tsx";
import ScrollToTop from "./components/ScrollToTop.tsx";
import { user_desktop_routes } from "./routes/userRoutes.ts";
import { agent_desktop_routes } from "./routes/agentRoutes.ts";

// User Pages
import UserDashboard from "./pages/UserDashboard.tsx";
import SendMoney from "./pages/send/SendMoney.tsx";
import UserTransactions from "./pages/transactions/Transactions.tsx";
import UserProfile from "./pages/profile/Profile.tsx";
import AccountSettings from "./pages/profile/AccountSettings.tsx";
import SecurityPrivacy from "./pages/profile/SecurityPrivacy.tsx";
import TransactionLimits from "./pages/profile/TransactionLimits.tsx";
import HelpSupport from "./pages/profile/HelpSupport.tsx";
import WithdrawMoney from "./pages/withdraw/Withdraw.tsx";
import DepositPage from "./pages/deposit/DepositPage.tsx";
import BitcoinExchangePage from "./pages/BitcoinExchangePage.tsx";
import AgentMapPage from "./pages/users/AgentMapPage.tsx";
import DAODashboard from "./pages/dao/DAODashboard.tsx";
import DAOLandingPage from "./pages/dao/DAOLandingPage.tsx";
import EscrowPage from "./pages/EscrowPage.tsx";
import CkBTCPage from "./pages/CkBTCPage.tsx";
import CkUSDCPage from "./pages/CkUSDCPage.tsx";
import LeaderboardPage from "./pages/users/LeaderboardPage.tsx";
import { CkBTCSendPage } from "./pages/CkBTCSendPage.tsx";
import { CkBTCDepositPage } from "./pages/CkBTCDepositPage.tsx";
import { CkUSDCSendPage } from "./pages/CkUSDCSendPage.tsx";
import { CkUSDCDepositPage } from "./pages/CkUSDCDepositPage.tsx";
import { CryptoExchangePage } from "./pages/CryptoExchangePage.tsx";

//Agent Pages
import AgentDashboard from './pages/AgentDashboard';
import ProcessWithdrawal from './pages/withdraw/ProcessWithdrawal';
import ProcessDeposits from './pages/agents/ProcessDeposits';
import ProcessWithdrawals from './pages/agents/ProcessWithdrawals';
import AgentCustomers from './pages/AgentCustomers';
import AgentSettings from './pages/agent-settings';
import RoleSelection from './pages/auth/RoleSelection';
import AgentFunding from './pages/agent-liquidity/AgentFunding';
import AgentSettlement from './pages/agent-liquidity/AgentSettlement';
import AgentExchangePage from "./pages/agents/AgentExchangePage.tsx";

// Auth Pages - Only KYC pages needed
import UserKYCPage from "./pages/auth/UserKYCPage";
import AgentKYCPage from "./pages/auth/AgentKYCPage";

// Admin Pages
import KYCAdmin from './pages/admin/KYCAdmin';
import AdminLogin from './pages/auth/AdminLogin';
import AdminProtectedRoute from './components/AdminProtectedRoute';
import ProtectedRoute from './components/ProtectedRoute';

// Landing Page
import LandingPage from "./pages/LandingPage";
import TariffPage from "./pages/TariffPage";
import USSDPlayground from "./pages/USSDPlayground";
import BecomeAgentPage from "./pages/BecomeAgentPage";
import AboutPage from "./pages/AboutPage";
import WhitepaperPage from "./pages/WhitepaperPage";

const App: FC = () => {
  useEffect(() => {
    const initJuno = async () => {
      // Skip Juno initialization for playground/demo routes
      const isPlaygroundRoute = window.location.pathname.includes('/playground');
      
      if (isPlaygroundRoute) {
        console.log('🎭 Playground mode: Skipping Juno initialization (using mocked data)');
        return;
      }
      
      try {
        // Use development satellite for localhost, production for deployed
        const isProduction = window.location.hostname !== 'localhost' && window.location.hostname !== '127.0.0.1';
        const satelliteId = isProduction 
          ? (junoConfig.satellite.ids?.production || "dkk74-oyaaa-aaaal-askxq-cai")
          : (junoConfig.satellite.ids?.development || "uxrrr-q7777-77774-qaaaq-cai");
        
        console.log(`Initializing Juno with ${isProduction ? 'production' : 'development'} satellite:`, satelliteId);
        
        await initSatellite({
          satelliteId,
          workers: {
            auth: true,
          },
        });
        console.log("✅ Juno satellite initialized successfully");
      } catch (error) {
        console.error("❌ Failed to initialize Juno satellite:", error);
      }
    };
    
    initJuno();
  }, []);

  return (
    <BrowserRouter>
      <ScrollToTop />
      <DemoModeProvider>
        <AuthenticationProvider>
          <Routes>
          {/* Auth Routes - Only Juno/ICP authentication used */}
    
          {/* User Routes */}
          <Route path="/users/*" element={
            <ProtectedRoute requiredRole="user">
              <ModernLayout routes={user_desktop_routes} userType="user">
                <Routes>
                <Route path="dashboard" element={<UserDashboard />} />
                <Route path="send" element={<SendMoney />} />
                <Route path="agents" element={<AgentMapPage />} />
                <Route path="deposit" element={<DepositPage />} />
                <Route path="history" element={<UserTransactions />} />
                <Route path="withdraw" element={<WithdrawMoney />} />
                <Route path="profile" element={<UserProfile />} />
                <Route path="profile/account-settings" element={<AccountSettings />} />
                <Route path="profile/security-privacy" element={<SecurityPrivacy />} />
                <Route path="profile/transaction-limits" element={<TransactionLimits />} />
                <Route path="profile/help-support" element={<HelpSupport />} />
                <Route path="user-kyc" element={<UserKYCPage />} />
                <Route path="ckbtc/send" element={<CkBTCSendPage />} />
                <Route path="ckbtc/deposit" element={<CkBTCDepositPage />} />
                <Route path="ckbtc/exchange" element={<CryptoExchangePage />} />
                <Route path="ckusdc/send" element={<CkUSDCSendPage />} />
                <Route path="ckusdc/deposit" element={<CkUSDCDepositPage />} />
                <Route path="ckusdc/exchange" element={<CryptoExchangePage />} />
                <Route path="dao" element={<DAODashboard />} />
                <Route path="leaderboard" element={<LeaderboardPage />} />
              </Routes>
            </ModernLayout>
            </ProtectedRoute>
          } />
          
          {/* Agent Routes */}
          <Route path="/agents/*" element={
            <ProtectedRoute requiredRole="agent">
              <ModernLayout routes={agent_desktop_routes} userType="agent">
                <Routes>
                <Route path="dashboard" element={<AgentDashboard />} />
                <Route path="withdraw" element={<ProcessWithdrawal/>} />
                <Route path="process-deposits" element={<ProcessDeposits/>} />
                <Route path="process-withdrawals" element={<ProcessWithdrawals/>} />
                <Route path="deposit" element={<ProcessDeposits/>} />
                <Route path="customers" element={<AgentCustomers/>} />
                <Route path="exchanges" element={<AgentExchangePage />} />
                <Route path="location" element={<div>Location Page</div>} />
                <Route path="dao" element={<DAODashboard />} />
                <Route path="settings" element={<AgentSettings/>} />
                <Route path="funding" element={<AgentFunding/>} />
                <Route path="settlement" element={<AgentSettlement/>} />
                <Route path="agent-kyc" element={<AgentKYCPage />} />
              </Routes>
              </ModernLayout>
            </ProtectedRoute>
          } />
          
          {/* Auth Routes */}
          <Route path="/auth/role-selection" element={<RoleSelection />} />
          <Route path="/auth/user-kyc" element={<UserKYCPage />} />
          <Route path="/auth/admin-login" element={<AdminLogin />} />
          
          {/* Admin Routes */}
          <Route path="/admin/kyc" element={
            <AdminProtectedRoute>
              <KYCAdmin />
            </AdminProtectedRoute>
          } />
          
          {/* Landing Page */}
          <Route path="/" element={<LandingPage />} />
          
          {/* How It Works Page */}
          <Route path="/how-it-works" element={<BitcoinExchangePage />} />
          <Route path="/bitcoin-exchange" element={<Navigate to="/how-it-works" replace />} />
          
          {/* Tariff Page */}
          <Route path="/tariff" element={<TariffPage />} />
          
          {/* DAO Landing Page */}
          <Route path="/dao-info" element={<DAOLandingPage />} />
          
          {/* Feature Pages */}
          <Route path="/escrow" element={<EscrowPage />} />
          <Route path="/ckbtc" element={<CkBTCPage />} />
          <Route path="/ckusdc" element={<CkUSDCPage />} />
          
          {/* About Page */}
          <Route path="/about" element={<AboutPage />} />
          
          {/* Whitepaper Page */}
          <Route path="/whitepaper" element={<WhitepaperPage />} />
          
          {/* USSD Playground */}
          <Route path="/ussd" element={<USSDPlayground />} />
          
          {/* Become an Agent */}
          <Route path="/become-agent" element={<BecomeAgentPage />} />
          
          {/* Default redirects */}
          <Route path="/dashboard" element={<Navigate to="/users/dashboard" replace />} />
        </Routes>
        </AuthenticationProvider>
      </DemoModeProvider>
    </BrowserRouter>
  );
};

export default App;
