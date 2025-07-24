import {
  BrowserRouter as Router,
  Routes,
  Route,
  Navigate,
} from "react-router-dom";
import UserRegistrationPage from "./pages/UserRegistrationPage";
import AgentRegistrationPage from "./pages/AgentRegistrationPage";
import UserDashboardPage from "./pages/UserDashboardPage";
import AgentDashboardPage from "./pages/AgentDashboardPage";

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/register" element={<UserRegistrationPage />} />
        <Route path="/register-agent" element={<AgentRegistrationPage />} />
        <Route path="/dashboard" element={<UserDashboardPage />} />
        <Route path="/agent-dashboard" element={<AgentDashboardPage />} />
        <Route path="*" element={<Navigate to="/register" replace />} />
      </Routes>
    </Router>
  );
}

export default App;
