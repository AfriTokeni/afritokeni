# AfriTokeni

## 🌍 Project Overview

AfriTokeni is a decentralized, USSD-powered mobile money platform built on the Internet Computer (ICP). It enables users in Africa to send, receive, and withdraw digital tokens using any mobile phone, without requiring internet access or a smartphone. By leveraging Motoko smart contracts and seamless USSD integration, AfriTokeni bridges the gap between blockchain technology and the unbanked population, empowering financial inclusion across the continent.

**Key Features:**

- USSD-based wallet access (works on any phone)
- Secure registration and PIN-based authentication
- Send, receive, and withdraw tokens instantly
- Agent/merchant support for cash-out
- Built on the Internet Computer for scalability and low fees

---

## 🎥 Demo Video

[Watch the demo on YouTube](https://your-demo-video-link-here)

## 🚀 Getting Started

### 1. Setting up the Repo on VSCode using GitHub Codespaces

- Open the repo in **GitHub Codespaces** (recommended for instant setup).
- The devcontainer will auto-install all dependencies.

### 2. Starting and Deploying the Canister

- Open a terminal in Codespaces.
- Start the local Internet Computer replica:
  ```bash
  dfx start --clean
  ```
- Deploy the canisters:
  ```bash
  dfx deploy
  ```

### 3. Running the Backend and Exposing the Local URL with Ngrok

- Install dependencies:
  ```bash
  npm install
  ```
- Add the port number to the created `.env` file in the project root with the following (fill in your values):
  ```
  PORT=3000
  ```
- Start the backend (for development):

  ```bash
  npm run build:backend
  npm run start:backend
  ```

- In a new terminal, expose your local server with ngrok:
  ```bash
  ngrok http 3000
  ```
- Copy the generated HTTPS URL (e.g., `https://abcd1234.ngrok.io`).

### 4. Africa's Talking Configuration

**Setup USSD Short Code**

- Login to Africa's Talking dashboard
- Go to USSD → Create Channel
- Configure:
  - Service Code: Choose your USSD code (e.g., \*384#)
  - Callback URL (Genrated ngrok url): https://abcd1234.ngrok.io/ussd

**Generate Your API Key**

- In the dashboard, go to **Settings** > **API Key**.
- Click **Generate New API Key**.
- Copy the generated API key and keep it safe (you’ll use it in your `.env` file as `AT_API_KEY`).

**Testing with the Sandbox**

- In the dashboard, go to **Settings** > **Sandbox Settings**.
- Under **Test Phone Numbers**, add your phone number (in international format, e.g., `+2567XXXXXXX`).
- Dial the sandbox short code (e.g., `*384*123#` or similar).
- Interact with your USSD app as a real user.

**Environment Variables Recap**

Add these to your `.env` file:

```
AT_API_KEY=your_africas_talking_api_key
```
