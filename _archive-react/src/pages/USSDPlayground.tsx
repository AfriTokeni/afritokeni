import React, { useState, useEffect, useRef } from 'react';
import { Link } from 'react-router-dom';
import { Smartphone, Send, Globe } from 'lucide-react';
import PublicHeader from '../components/PublicHeader';
import PublicFooter from '../components/PublicFooter';
import { CentralizedDemoService } from '../services/centralizedDemoService';
import { USSDService } from '../services/ussdService';
import { USSDSession } from '../services/ussd/types';

interface Message {
  type: 'sent' | 'received';
  text: string;
  timestamp: string;
}

const USSDPlayground: React.FC = () => {
  const [phoneNumber] = useState('+256 700 123 456');
  const [demoUserId] = useState('demo_ussd_user');
  const [sessionId] = useState('playground_session_' + Date.now());
  const [inputCommand, setInputCommand] = useState('');
  const [isInitialized, setIsInitialized] = useState(false);
  const [ussdText, setUssdText] = useState('');
  const sessionRef = useRef<USSDSession | null>(null);
  const [messages, setMessages] = useState<Message[]>([
    {
      type: 'received',
      text: '🌍 USSD Demo Mode\n\nWelcome to AfriTokeni!\n\nDial *384*22948# to start\n\n🔐 Demo PIN: 1234',
      timestamp: new Date().toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' })
    }
  ]);
  const messagesEndRef = React.useRef<HTMLDivElement>(null);
  const messagesContainerRef = React.useRef<HTMLDivElement>(null);

  // Initialize demo user (playground uses in-memory mock data, no Juno)
  useEffect(() => {
    const init = async () => {
      console.log('🎭 Playground: Using in-memory mock data (no backend calls)');
      // Playground doesn't need to initialize anything - all data is mocked
      setIsInitialized(true);
    };
    init();
  }, [demoUserId]);

  // Auto-scroll to bottom when messages change (only scroll the messages container, not the page)
  useEffect(() => {
    if (messagesContainerRef.current) {
      messagesContainerRef.current.scrollTop = messagesContainerRef.current.scrollHeight;
    }
  }, [messages]);


  const processCommand = async (cmd: string): Promise<string> => {
    const trimmedCmd = cmd.trim();
    
    // Ensure initialization is complete
    if (!isInitialized) {
      return '⏳ Initializing... Please wait.';
    }

    // Handle USSD dial code
    if (trimmedCmd.toUpperCase() === '*384*22948#' || trimmedCmd === '*229#') {
      setUssdText(''); // Reset USSD chain
      const result = await USSDService.processUSSDRequest(sessionId, phoneNumber, trimmedCmd);
      return result.response.replace(/^(CON |END )/, '');
    }

    // For menu navigation, send just the current input, not chained
    // Only chain if this is the initial dial (starts with *)
    let newUssdText = trimmedCmd;
    if (trimmedCmd.startsWith('*')) {
      // Initial USSD dial - use as is
      newUssdText = trimmedCmd;
      setUssdText(''); // Reset chain for menu navigation
    } else {
      // Menu navigation - just send the digit
      newUssdText = trimmedCmd;
    }
    
    // Call real USSD backend
    console.log(`📱 USSD Playground: text="${newUssdText}"`);
    const result = await USSDService.processUSSDRequest(sessionId, phoneNumber, newUssdText);
    
    // If session ended, reset
    if (!result.continueSession) {
      setUssdText('');
    }
    
    // Clean up response
    return result.response.replace(/^(CON |END )/, '');
  };

  const handleSendMessage = async () => {
    if (!inputCommand.trim()) return;

    const now = new Date();
    const timestamp = now.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' });

    // Add sent message
    const sentMessage: Message = {
      type: 'sent',
      text: inputCommand,
      timestamp
    };

    setMessages(prev => [...prev, sentMessage]);

    // Process command immediately (no artificial delay)
    const response = await processCommand(inputCommand);
    const receivedMessage: Message = {
      type: 'received',
      text: response,
      timestamp: new Date().toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' })
    };
    setMessages(prev => [...prev, receivedMessage]);

    setInputCommand('');
  };

  return (
    <div className="min-h-screen bg-gray-50">
      <PublicHeader />
      <div className="bg-gradient-to-br from-gray-50 to-gray-100 py-8 sm:py-10 lg:py-12">
        <div className="max-w-6xl mx-auto px-4 sm:px-6">
          {/* Hero */}
          <div className="text-center mb-8 sm:mb-10 lg:mb-12">
            <div className="flex flex-col sm:flex-row items-center justify-center gap-2 sm:gap-3 mb-4 sm:mb-6">
              <div className="inline-flex items-center gap-2 bg-orange-50 text-orange-700 px-3 sm:px-4 py-2 rounded-full text-xs sm:text-sm font-semibold">
                <Globe className="w-3 h-3 sm:w-4 sm:h-4" />
                🌍 Demo Mode - Works Worldwide
              </div>
              <div className="inline-flex items-center gap-2 bg-green-50 text-green-700 px-3 sm:px-4 py-2 rounded-full text-xs sm:text-sm font-semibold">
                <Smartphone className="w-3 h-3 sm:w-4 sm:h-4" />
                🇺🇬 Real USSD: Uganda Only
              </div>
            </div>
            <h1 className="text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-bold text-gray-900 mb-3 sm:mb-4">
              Try USSD Banking
            </h1>
            <p className="text-base sm:text-lg lg:text-xl text-gray-600 max-w-2xl mx-auto leading-relaxed">
              Experience how AfriTokeni works on any phone - no internet required. Try the commands below!
            </p>
            <div className="mt-6 bg-red-50 border-2 border-red-300 rounded-lg p-4 max-w-2xl mx-auto">
              <h3 className="font-bold text-red-900 mb-2 text-sm sm:text-base">🇺🇬 REAL USSD: UGANDA ONLY</h3>
              <p className="text-xs sm:text-sm text-red-800 mb-2">
                <strong>Uganda:</strong> Dial <code className="bg-red-100 px-1.5 py-0.5 rounded font-mono text-xs">*284*78909#</code> for real USSD!
              </p>
              <p className="text-xs text-red-700">
                <strong>Other countries:</strong> Coming soon! Use playground to test.
              </p>
            </div>
          </div>

          {/* Phone Simulator - Centered */}
          <div className="max-w-sm sm:max-w-md mx-auto">
            <div className="bg-black rounded-[2rem] sm:rounded-[3rem] shadow-2xl overflow-hidden p-2 sm:p-3">
                {/* Phone Notch */}
                <div className="bg-gray-900 rounded-[1.5rem] sm:rounded-[2.5rem] overflow-hidden">
                  {/* Status Bar */}
                  <div className="bg-gray-900 px-4 sm:px-6 py-2 flex items-center justify-between text-white text-xs">
                    <span>9:21</span>
                    <div className="flex items-center gap-1">
                      <div className="w-3 h-2 sm:w-4 sm:h-3 border border-white rounded-sm"></div>
                      <div className="w-2 h-2 sm:w-3 sm:h-3 bg-white rounded-full"></div>
                      <div className="flex gap-0.5">
                        <div className="w-0.5 h-2 sm:h-3 bg-white"></div>
                        <div className="w-0.5 h-2 sm:h-3 bg-white"></div>
                        <div className="w-0.5 h-2 sm:h-3 bg-white"></div>
                        <div className="w-0.5 h-2 sm:h-3 bg-white"></div>
                      </div>
                    </div>
                  </div>
                  {/* USSD Header */}
                  <div className="bg-gray-800 text-white px-4 sm:px-6 py-2 sm:py-3 flex items-center justify-between border-b border-gray-700">
                    <div className="flex items-center gap-2">
                      <Smartphone className="w-3 h-3 sm:w-4 sm:h-4" />
                      <span className="font-semibold text-xs sm:text-sm">*384*22948#</span>
                    </div>
                    <span className="text-xs text-gray-400 hidden sm:inline">{phoneNumber}</span>
                  </div>

                  {/* Messages */}
                  <div ref={messagesContainerRef} className="h-[400px] sm:h-[500px] lg:h-[600px] overflow-y-auto bg-gray-50 p-3 sm:p-4 space-y-3 sm:space-y-4 scroll-smooth">
                    {messages.map((msg, idx) => (
                      <div key={idx} className={`flex ${msg.type === 'sent' ? 'justify-end' : 'justify-start'}`}>
                        <div className={`max-w-[280px] sm:max-w-xs ${msg.type === 'sent' ? 'bg-blue-600 text-white' : 'bg-white text-gray-900'} rounded-2xl px-3 sm:px-4 py-2 sm:py-3 shadow-sm`}>
                          <p className="text-xs sm:text-sm whitespace-pre-line">{msg.text}</p>
                          <p className={`text-xs mt-1 ${msg.type === 'sent' ? 'text-blue-200' : 'text-gray-500'}`}>
                            {msg.timestamp}
                          </p>
                        </div>
                      </div>
                    ))}
                    <div ref={messagesEndRef} />
                  </div>

                  {/* Input */}
                  <div className="bg-white border-t border-gray-200 p-3 sm:p-4">
                    {/* Quick Start Button */}
                    <button
                      onClick={() => {
                        setInputCommand('*384*22948#');
                        setTimeout(() => handleSendMessage(), 100);
                      }}
                      className="w-full mb-2 sm:mb-3 bg-green-500 hover:bg-green-600 text-white font-bold py-2.5 sm:py-3 rounded-xl transition-colors flex items-center justify-center gap-2 text-sm sm:text-base"
                    >
                      <Smartphone className="w-4 h-4 sm:w-5 sm:h-5" />
                      Dial *384*22948#
                    </button>
                    
                    <div className="flex gap-2">
                      <input
                        type="tel"
                        inputMode="tel"
                        value={inputCommand}
                        onChange={(e) => setInputCommand(e.target.value)}
                        onKeyPress={(e) => e.key === 'Enter' && handleSendMessage()}
                        placeholder="Type USSD command..."
                        className="flex-1 px-3 sm:px-4 py-2.5 sm:py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm sm:text-base"
                      />
                      <button
                        onClick={handleSendMessage}
                        className="bg-blue-600 text-white px-4 sm:px-6 py-2.5 sm:py-3 rounded-xl hover:bg-blue-700 transition-colors"
                      >
                        <Send className="w-4 h-4 sm:w-5 sm:h-5" />
                      </button>
                    </div>
                  </div>
                </div>
              </div>

          </div>

          {/* CTA Section */}
                    <div className="mt-8 sm:mt-10 lg:mt-12 bg-gradient-to-r from-blue-600 to-cyan-600 rounded-xl p-6 sm:p-8 lg:p-12 text-center text-white">
            <h2 className="text-xl sm:text-2xl lg:text-3xl font-bold mb-3 sm:mb-4">Ready to Start Banking?</h2>
            <p className="text-base sm:text-lg lg:text-xl mb-4 sm:mb-6 opacity-90">
              Experience the future of mobile banking with instant, fee-free transfers
            </p>
            <div className="flex flex-col sm:flex-row gap-3 sm:gap-4 justify-center">
              <Link
                to="/bitcoin-exchange"
                className="bg-white text-blue-600 px-6 sm:px-8 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-medium hover:bg-neutral-100 transition-colors"
              >
                Start Banking Now
              </Link>
              <Link
                to="/become-agent"
                className="bg-white text-blue-600 px-6 sm:px-8 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-medium hover:bg-neutral-100 transition-colors"
              >
                Become an Agent
              </Link>
            </div>
          </div>
        </div>
      </div>
      <PublicFooter />
    </div>
  );
};

export default USSDPlayground;
