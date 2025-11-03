import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import "./index.css";
import Userback from "@userback/widget";

// Initialize Userback
const initUserback = async () => {
  const token = import.meta.env.VITE_USERBACK_TOKEN;
  
  if (!token) {
    console.warn('Userback token not found in .env');
    return;
  }
  
  await Userback(token);
  console.log("✅ Userback initialized");
};

initUserback();

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
