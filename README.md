# AfriTokeni

## 📋 Project Overview

**AfriTokeni** is a decentralized finance (DeFi) application built on the Internet Computer Protocol (ICP) blockchain that bridges the gap between traditional mobile money systems and modern blockchain technology. It addresses critical challenges in Africa's mobile money ecosystem to enable true financial inclusion.

### The Problem

Despite being a leader in financial innovation, Africa's mobile money ecosystem continues to face systemic inefficiencies:

- **High Transaction Costs**: The average cost of sending $200 across African borders is around 8.2% —significantly higher than the global average of 6.3%. Domestic cash-out fees can reach up to 4–6%.
- **Limited Interoperability**: Over 1.1 billion mobile money accounts exist in Africa, yet less than 15% of mobile money platforms are interoperable across networks or countries.
- **Fragmented Financial Access**: Only 43% of adults in Sub-Saharan Africa have access to formal financial services. Cross-border remittances cost an average of $7 billion annually in fees.
- **Digital Divide**: Over 60% of Africans still use feature phones without smartphone capabilities, locking them out of modern fintech apps and DeFi innovations.

### The Solution

**AfriTokeni** provides a unified, interoperable, and borderless payment layer that works seamlessly across devices and networks using:

- **Frontend**: React + Vite for a fast, modern web interface
- **Blockchain**: Internet Computer Protocol (ICP) for decentralized backend services
- **Communication**: Africastalking API for USSD and SMS messaging
- **Stablecoins**: USD-backed tokens for stable value transfer
- **Universal Access**: Dual interface supporting both smartphones and feature phones

### How It Works

1. **Universal Access via Dual Interface**
   - **Smartphone Users**: Use the AfriTokeni mobile app with biometric authentication
   - **Feature Phone Users**: Access all core services via USSD code (*789#), no internet required

2. **Core Functions** (Available via app or USSD):
   - **Send Money**: Instantly transfer stablecoins to any registered user
   - **Receive Money**: Get funds from anyone on the network
   - **Check Balance**: View real-time balance
   - **Withdraw Cash**: Convert stablecoins into physical cash through trusted agents

3. **Agent-Based Cash Withdrawal**:
   - Users initiate withdrawal and select nearby verified agents
   - PIN authentication and unique verification codes ensure security
   - Agents dispense cash with automatic blockchain settlement

## 🎥 Demo Video

[Watch the demo video on YouTube](https://www.youtube.com/watch?v=46GGEBX4JAA)

## 🚀 Local Development Setup

### Prerequisites

Before you begin, ensure you have the following installed:
- [Docker](https://www.docker.com/get-started)
- [ngrok](https://ngrok.com/download)

### Step 1: Clone Repository and Open in Dev Container

```bash
git clone https://github.com/AfriTokeni/afritokeni.git
cd afritokeni
```

Open the project in your development container (VS Code Dev Containers or similar).

### Step 2: Install Dependencies

Install both npm and mops dependencies:

```bash
npm install
mops install
```

### Step 3: Start and Deploy Canisters

Deploy the ICP canisters to your local development environment:

```bash
dfx start --clean
dfx deploy
```

### Step 4: Start Frontend Development Server

For browser-based interaction, start the Vite development server:

```bash
npm start
```

The frontend will be available at `http://localhost:5173`

### Step 5: Setup Africastalking Integration

#### 5.1 Create Africastalking Account

1. Visit [Africastalking](https://africastalking.com) and create an account
2. Complete the registration process

#### 5.2 Generate API Key

1. Login to your Africastalking account
2. Click **"Go to Sandbox"** button
3. In the sidebar, navigate to **Settings → API Key**
4. Generate a new API key
5. Copy the generated API key

#### 5.3 Configure Environment Variables

Open the `.env` file in your project root and add your API key:

```env
AT_API_KEY=your_api_key_here
```

#### 5.4 Run Backend Server

1. Build and start the backend server:
   ```bash
   npm run build:backend && npm run start:backend
   ```

2. The backend will start on port 3000. Ensure this port is exposed in your container configuration.

3. Expose the backend using ngrok:
   ```bash
   ngrok http http://localhost:3000
   ```

4. Copy the generated ngrok URL (e.g., `https://abc123.ngrok.io`)

#### 5.5 Setup USSD Channel

1. In your Africastalking sandbox, navigate to **USSD → Create Channel**
2. Input your desired channel number
3. Add the ngrok URL as the callback URL: `https://your-ngrok-url.ngrok.io/ussd`
4. Save the channel configuration

#### 5.6 Setup SMS Shortcode

1. In the sidebar, click **SMS → Shortcodes → Create Shortcode**
2. Generate a shortcode for your application
3. Configure the shortcode settings as needed

#### 5.7 Update Environment Variables

Add the following to your `.env` file:

```env
AT_API_KEY=your_api_key_here
AT_USERNAME=sandbox
AT_SHORT_CODE=your_generated_shortcode
```

## 📱 Testing via USSD

### Using Africastalking Simulator

1. In your Africastalking sandbox, click **"Launch Simulator"**
2. Click the **phone icon** to open the dial pad
3. Type your USSD service code (e.g., `*384*1234#`)
4. Follow the prompts to interact with your application
5. Test various user flows and scenarios

### Testing Workflow

1. Ensure your backend is running and accessible via ngrok
2. Verify your USSD channel is properly configured
3. Use the simulator to test different user interactions

