import React, { useState, useEffect } from 'react';
import { Users, DollarSign, MapPin, Shield, TrendingUp, CheckCircle } from 'lucide-react';
import { useAuthentication } from '../context/AuthenticationContext';
import { LoginFormData } from '../types/auth';
import PublicHeader from '../components/PublicHeader';
import PublicFooter from '../components/PublicFooter';

const BecomeAgentPage: React.FC = () => {
  const { user, login } = useAuthentication();

  // Currency rotation for hero
  const currencies = [
    { code: 'UGX', min: '500K', max: '2M', country: 'Uganda' },
    { code: 'NGN', min: '200K', max: '800K', country: 'Nigeria' },
    { code: 'KES', min: '60K', max: '250K', country: 'Kenya' },
    { code: 'GHS', min: '3K', max: '12K', country: 'Ghana' },
    { code: 'ZAR', min: '8K', max: '35K', country: 'South Africa' },
    { code: 'TZS', min: '1.2M', max: '4.8M', country: 'Tanzania' },
    { code: 'XOF', min: '300K', max: '1.2M', country: 'West Africa' },
    { code: 'EGP', min: '25K', max: '100K', country: 'Egypt' },
    { code: 'MAD', min: '5K', max: '20K', country: 'Morocco' }
  ];

  const [currentCurrencyIndex, setCurrentCurrencyIndex] = useState(0);

  useEffect(() => {
    const interval = setInterval(() => {
      setCurrentCurrencyIndex((prev) => (prev + 1) % currencies.length);
    }, 3000); // Change every 3 seconds

    return () => clearInterval(interval);
  }, []);

  const currentCurrency = currencies[currentCurrencyIndex];

  const handleGetStarted = async () => {
    if (!user) {
      // Trigger ICP login, then redirect to role selection
      try {
        await login({} as LoginFormData, 'web');
        // After successful ICP login, go to role selection
        window.location.href = '/auth/role-selection';
      } catch (error) {
        console.error('Login failed:', error);
      }
    } else {
      // Already logged in, go to role selection or agent dashboard
      window.location.href = '/auth/role-selection';
    }
  };

  // Dynamic earnings based on current currency
  const getEarnings = (currency: typeof currencies[0]) => [
    { 
      location: 'Urban Centers', 
      commission: '2.5-4%', 
      monthly: `${currency.code} ${currency.min} - ${(parseFloat(currency.max.replace(/[KM]/g, '')) * 0.5).toFixed(0)}${currency.max.includes('M') ? 'M' : 'K'}`,
      description: 'High volume, competitive rates'
    },
    { 
      location: 'Suburban Areas', 
      commission: '3-5%', 
      monthly: `${currency.code} ${(parseFloat(currency.min.replace(/[KM]/g, '')) * 1.2).toFixed(0)}${currency.min.includes('M') ? 'M' : 'K'} - ${(parseFloat(currency.max.replace(/[KM]/g, '')) * 0.6).toFixed(1)}${currency.max.includes('M') ? 'M' : 'K'}`,
      description: 'Balanced volume and rates'
    },
    { 
      location: 'Rural Towns', 
      commission: '4-7%', 
      monthly: `${currency.code} ${(parseFloat(currency.min.replace(/[KM]/g, '')) * 1.6).toFixed(0)}${currency.min.includes('M') ? 'M' : 'K'} - ${(parseFloat(currency.max.replace(/[KM]/g, '')) * 0.75).toFixed(1)}${currency.max.includes('M') ? 'M' : 'K'}`,
      description: 'Less competition, higher rates'
    },
    { 
      location: 'Remote Villages', 
      commission: '7-12%', 
      monthly: `${currency.code} ${currency.min} - ${currency.max}`,
      description: 'Premium rates, high demand'
    }
  ];

  const earnings = getEarnings(currentCurrency);

  const benefits = [
    {
      icon: DollarSign,
      title: 'Earn Commission',
      description: 'Earn 2-12% on every transaction. Higher rates for serving remote areas.'
    },
    {
      icon: Shield,
      title: 'Zero Risk',
      description: 'Escrow system protects you. Get paid before releasing cash to customers.'
    },
    {
      icon: TrendingUp,
      title: 'Flexible Schedule',
      description: 'Work when you want. Set your own hours and service areas.'
    },
    {
      icon: MapPin,
      title: 'Serve Your Community',
      description: 'Bring financial services to your neighborhood. Help the unbanked.'
    }
  ];

  const requirements = [
    'Valid government ID',
    'Mobile phone (any type)',
    'Initial liquidity ($100-200)',
    'Mobile or fixed location',
    'Pass KYC verification',
    'Set your own hours'
  ];

  return (
    <div className="min-h-screen bg-gray-50">
      <PublicHeader />

      <div className="max-w-6xl mx-auto px-4 sm:px-6 py-12 sm:py-16">
        {/* Hero */}
        <div className="text-center mb-16">
          <div className="inline-flex items-center gap-2 bg-green-50 text-green-700 px-4 py-2 rounded-full text-sm font-semibold mb-6">
            <Users className="w-4 h-4" />
            Agent Program
          </div>
          <h1 className="text-4xl sm:text-5xl md:text-6xl font-bold text-gray-900 mb-6">
            <span className="block mb-2">Earn</span>
            <span 
              key={currentCurrencyIndex} 
              className="inline-block text-orange-600 animate-fade-in"
            >
              {currentCurrency.code} {currentCurrency.min} - {currentCurrency.max}
            </span>
            <span className="block mt-2">Monthly as an Agent</span>
          </h1>
          <p className="text-lg text-gray-500 mb-8">
            <span className="inline-block animate-fade-in" key={`country-${currentCurrencyIndex}`}>
              📍 {currentCurrency.country}
            </span>
          </p>
          <p className="text-xl sm:text-2xl text-gray-600 mb-8 max-w-3xl mx-auto">
            Help your community access Bitcoin banking while earning consistent income. No technical knowledge required.
          </p>
          <button
            onClick={handleGetStarted}
            className="bg-black text-white px-10 py-5 rounded-xl text-lg font-semibold hover:bg-gray-800 transform hover:scale-105 transition-all duration-200 shadow-lg"
          >
            {user.user || user.agent ? 'Apply Now' : 'Sign In to Apply'}
          </button>
        </div>

        {/* Earnings Potential */}
        <div className="bg-gradient-to-br from-green-50 to-emerald-50 rounded-2xl shadow-lg p-8 sm:p-12 mb-12 border-2 border-green-200">
          <div className="text-center mb-10">
            <h2 className="text-3xl sm:text-4xl font-bold text-gray-900 mb-3">Real Earnings Potential</h2>
            <p className="text-lg text-gray-600">Based on 20-30 transactions per month</p>
          </div>
          <div className="grid md:grid-cols-2 gap-6">
            {earnings.map((item, idx) => (
              <div key={idx} className="bg-white rounded-xl p-6 shadow-md hover:shadow-xl transition-shadow">
                <div className="flex items-start justify-between mb-4">
                  <div>
                    <h3 className="text-xl font-bold text-gray-900 mb-1">{item.location}</h3>
                    <p className="text-sm text-gray-600">{item.description}</p>
                  </div>
                  <div className="text-2xl font-bold text-green-600">{item.commission}</div>
                </div>
                <div className="bg-green-50 rounded-lg p-4 border border-green-200">
                  <div className="text-sm text-gray-600 mb-1">Monthly Potential</div>
                  <div className="text-2xl font-bold text-green-700">{item.monthly}</div>
                </div>
              </div>
            ))}
          </div>
          <div className="mt-8 bg-white rounded-lg p-6 border-2 border-green-300">
            <div className="flex items-start gap-3">
              <TrendingUp className="w-6 h-6 text-green-600 flex-shrink-0 mt-1" />
              <div>
                <p className="font-semibold text-gray-900 mb-2">Top agents earn even more:</p>
                <p className="text-gray-600">High-volume agents in remote areas can earn {currentCurrency.code} {(parseFloat(currentCurrency.max.replace(/[KM]/g, '')) * 1.5).toFixed(0)}{currentCurrency.max.includes('M') ? 'M' : 'K'}+ monthly by serving underserved communities.</p>
              </div>
            </div>
          </div>
        </div>

        {/* Benefits */}
        <div className="mb-12">
          <h2 className="text-3xl font-bold text-gray-900 mb-8 text-center">Why Agents Love AfriTokeni</h2>
          <div className="grid md:grid-cols-2 gap-6">
            {benefits.map((benefit, idx) => (
              <div key={idx} className="bg-white rounded-xl p-8 shadow-lg border border-gray-100 hover:border-orange-300 transition-colors">
                <div className="flex items-start gap-4">
                  <div className="w-14 h-14 bg-orange-100 rounded-xl flex items-center justify-center flex-shrink-0">
                    <benefit.icon className="w-7 h-7 text-orange-600" />
                  </div>
                  <div>
                    <h3 className="text-xl font-bold text-gray-900 mb-2">{benefit.title}</h3>
                    <p className="text-gray-600 leading-relaxed">{benefit.description}</p>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Requirements */}
        <div className="bg-white rounded-2xl shadow-lg p-8 sm:p-12 mb-12 border border-gray-200">
          <h2 className="text-3xl font-bold text-gray-900 mb-3 text-center">Simple Requirements</h2>
          <p className="text-center text-gray-600 mb-8">Everything you need to get started</p>
          <div className="grid md:grid-cols-2 gap-6 max-w-4xl mx-auto">
            {requirements.map((req, idx) => (
              <div key={idx} className="flex items-start gap-4 bg-gray-50 rounded-xl p-5 border border-gray-200 min-h-[80px]">
                <div className="w-8 h-8 bg-green-100 rounded-full flex items-center justify-center flex-shrink-0">
                  <CheckCircle className="w-5 h-5 text-green-600" />
                </div>
                <span className="text-gray-900 font-medium text-lg flex-1">{req}</span>
              </div>
            ))}
          </div>
        </div>

        {/* How It Works */}
        <div className="bg-white rounded-2xl shadow-lg p-8 mb-12 border border-gray-100">
          <h2 className="text-3xl font-bold text-gray-900 mb-8 text-center">How It Works</h2>
          <div className="grid md:grid-cols-3 gap-8">
            <div className="text-center">
              <div className="w-16 h-16 bg-orange-100 rounded-full flex items-center justify-center mx-auto mb-4 text-2xl font-bold text-orange-600">
                1
              </div>
              <h3 className="text-xl font-bold text-gray-900 mb-2">Apply & Verify</h3>
              <p className="text-gray-600">
                Sign up, complete KYC verification, and set up your agent profile.
              </p>
            </div>
            <div className="text-center">
              <div className="w-16 h-16 bg-orange-100 rounded-full flex items-center justify-center mx-auto mb-4 text-2xl font-bold text-orange-600">
                2
              </div>
              <h3 className="text-xl font-bold text-gray-900 mb-2">Fund Your Account</h3>
              <p className="text-gray-600">
                Add initial liquidity to process customer transactions.
              </p>
            </div>
            <div className="text-center">
              <div className="w-16 h-16 bg-orange-100 rounded-full flex items-center justify-center mx-auto mb-4 text-2xl font-bold text-orange-600">
                3
              </div>
              <h3 className="text-xl font-bold text-gray-900 mb-2">Start Earning</h3>
              <p className="text-gray-600">
                Process transactions and earn commission on every exchange.
              </p>
            </div>
          </div>
        </div>

        {/* CTA */}
        <div className="text-center bg-gradient-to-r from-orange-50 to-red-50 rounded-2xl p-12 border-2 border-orange-200">
          <h2 className="text-3xl font-bold text-gray-900 mb-4">Ready to Get Started?</h2>
          <p className="text-xl text-gray-600 mb-8 max-w-2xl mx-auto">
            Join hundreds of agents earning money while serving their communities.
          </p>
          <button
            onClick={handleGetStarted}
            className="bg-black text-white px-10 py-5 rounded-xl text-lg font-semibold hover:bg-gray-800 transform hover:scale-105 transition-all duration-200 shadow-lg"
          >
            {user.user || user.agent ? 'Apply Now' : 'Sign In to Apply'}
          </button>
        </div>
      </div>

      <PublicFooter />
    </div>
  );
};

export default BecomeAgentPage;
