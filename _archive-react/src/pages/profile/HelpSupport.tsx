import React, { useState } from 'react';
import { ArrowLeft, HelpCircle, MessageCircle, Phone, Mail, FileText, ExternalLink } from 'lucide-react';
import { useNavigate } from 'react-router-dom';

const HelpSupport: React.FC = () => {
  const navigate = useNavigate();
  const [selectedFAQ, setSelectedFAQ] = useState<number | null>(null);

  const faqs = [
    {
      question: "How do I send money via SMS?",
      answer: "Send '*AFRI*SEND*AMOUNT*PHONE#' to 6789. For example: '*AFRI*SEND*5000*256701234567#' to send 5,000 UGX to +256701234567."
    },
    {
      question: "What are the transaction fees?",
      answer: "Fees vary by location and urgency. Urban areas: 2.5-4%, Rural areas: 4-7%, Remote areas: 7-12%. Use our smart pricing calculator for exact fees."
    },
    {
      question: "How do I check my Bitcoin balance?",
      answer: "Send 'BTC BAL' to 6789 via SMS, or check the Bitcoin section in your web dashboard."
    },
    {
      question: "How do I withdraw cash?",
      answer: "Use '*AFRI*WITHDRAW*AMOUNT*AGENT_ID#' via SMS or use the withdraw feature in the app to find nearby agents."
    },
    {
      question: "What if I forget my PIN?",
      answer: "Contact support at +256 700 123 456 or email support@afritokeni.com with your registered phone number for PIN reset assistance."
    },
    {
      question: "How do I become an agent?",
      answer: "Complete agent KYC verification, maintain minimum cash and digital balances, and pass our location-based assessment."
    }
  ];

  const contactMethods = [
    {
      icon: Phone,
      title: "Phone Support",
      description: "24/7 customer support hotline",
      value: "+256 700 123 456",
      action: "tel:+256700123456"
    },
    {
      icon: MessageCircle,
      title: "SMS Support",
      description: "Send HELP to get instant assistance",
      value: "Send 'HELP' to 6789",
      action: "sms:6789?body=HELP"
    },
    {
      icon: Mail,
      title: "Email Support",
      description: "Get detailed help via email",
      value: "support@afritokeni.com",
      action: "mailto:support@afritokeni.com"
    }
  ];

  const resources = [
    {
      title: "SMS Command Reference",
      description: "Complete list of all SMS commands",
      link: "/sms"
    },
    {
      title: "Smart Pricing Guide",
      description: "Understanding our dynamic fee structure",
      link: "/tariff"
    },
    {
      title: "Bitcoin Exchange Guide",
      description: "How to buy and sell Bitcoin safely",
      link: "/bitcoin-exchange"
    },
    {
      title: "Agent Network Map",
      description: "Find agents near your location",
      link: "/agents/map"
    }
  ];

  return (
    <div className="space-y-6">
      <div className="max-w-4xl mx-auto space-y-6">
        {/* Header */}
        <div className="flex items-center space-x-4 mb-6">
          <button
            onClick={() => navigate('/users/profile')}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <ArrowLeft className="w-5 h-5 text-gray-600" />
          </button>
          <h1 className="text-2xl font-bold text-gray-900">Help & Support</h1>
        </div>

        {/* Quick Actions */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          {contactMethods.map((method, index) => (
            <a
              key={index}
              href={method.action}
              className="bg-white rounded-xl shadow-sm border border-gray-200 p-6 hover:shadow-md transition-shadow"
            >
              <div className="flex items-center space-x-3 mb-3">
                <method.icon className="w-6 h-6 text-gray-600" />
                <h3 className="font-semibold text-gray-900">{method.title}</h3>
              </div>
              <p className="text-sm text-gray-600 mb-2">{method.description}</p>
              <p className="text-sm font-medium text-gray-900">{method.value}</p>
            </a>
          ))}
        </div>

        {/* FAQ Section */}
        <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
          <h2 className="text-lg font-semibold text-gray-900 mb-4 flex items-center space-x-2">
            <HelpCircle className="w-5 h-5" />
            <span>Frequently Asked Questions</span>
          </h2>

          <div className="space-y-3">
            {faqs.map((faq, index) => (
              <div key={index} className="border border-gray-200 rounded-lg">
                <button
                  onClick={() => setSelectedFAQ(selectedFAQ === index ? null : index)}
                  className="w-full px-4 py-3 text-left hover:bg-gray-50 transition-colors flex items-center justify-between"
                >
                  <span className="font-medium text-gray-900">{faq.question}</span>
                  <span className="text-gray-400 text-lg">
                    {selectedFAQ === index ? '−' : '+'}
                  </span>
                </button>
                {selectedFAQ === index && (
                  <div className="px-4 pb-3 border-t border-gray-200">
                    <p className="text-sm text-gray-600 pt-3">{faq.answer}</p>
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>

        {/* Resources */}
        <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
          <h2 className="text-lg font-semibold text-gray-900 mb-4 flex items-center space-x-2">
            <FileText className="w-5 h-5" />
            <span>Helpful Resources</span>
          </h2>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {resources.map((resource, index) => (
              <a
                key={index}
                href={resource.link}
                className="flex items-center justify-between p-4 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors"
              >
                <div>
                  <h3 className="font-medium text-gray-900">{resource.title}</h3>
                  <p className="text-sm text-gray-600">{resource.description}</p>
                </div>
                <ExternalLink className="w-4 h-4 text-gray-400" />
              </a>
            ))}
          </div>
        </div>

        {/* Emergency Contact */}
        <div className="bg-red-50 border border-red-200 rounded-xl p-6">
          <h2 className="text-lg font-semibold text-red-900 mb-2">Emergency Support</h2>
          <p className="text-sm text-red-700 mb-4">
            If you're experiencing urgent issues with transactions or security concerns:
          </p>
          <div className="flex flex-col sm:flex-row space-y-2 sm:space-y-0 sm:space-x-4">
            <a
              href="tel:+256700123456"
              className="inline-flex items-center justify-center px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors"
            >
              <Phone className="w-4 h-4 mr-2" />
              Call Emergency Line
            </a>
            <a
              href="sms:6789?body=EMERGENCY"
              className="inline-flex items-center justify-center px-4 py-2 bg-red-100 text-red-700 border border-red-300 rounded-lg hover:bg-red-200 transition-colors"
            >
              <MessageCircle className="w-4 h-4 mr-2" />
              SMS "EMERGENCY" to 6789
            </a>
          </div>
        </div>
      </div>
    </div>
  );
};

export default HelpSupport;
