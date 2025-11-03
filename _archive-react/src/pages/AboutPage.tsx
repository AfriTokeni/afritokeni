import React from 'react';
import { Users, Target, Zap, Globe, Award, Heart } from 'lucide-react';
import PublicHeader from '../components/PublicHeader';
import PublicFooter from '../components/PublicFooter';

const AboutPage: React.FC = () => {
  return (
    <div className="min-h-screen bg-gray-50">
      <PublicHeader />
      
      <div className="max-w-4xl mx-auto px-4 sm:px-6 py-8 sm:py-10 lg:py-12">
        {/* Hero */}
        <div className="text-center mb-12 sm:mb-14 lg:mb-16">
          <div className="inline-flex items-center gap-2 bg-orange-50 text-orange-700 px-3 sm:px-4 py-2 rounded-full text-xs sm:text-sm font-semibold mb-4 sm:mb-6">
            <Award className="w-3 h-3 sm:w-4 sm:h-4" />
            World Hackathon League - ICP Track
          </div>
          <h1 className="text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-bold text-gray-900 mb-4 sm:mb-6">
            Banking the Unbanked
          </h1>
          <p className="text-base sm:text-lg lg:text-xl text-gray-600 max-w-3xl mx-auto leading-relaxed">
            We&apos;re building the future of financial inclusion in Africa, one USSD transaction at a time.
          </p>
        </div>

        {/* Our Story */}
        <div className="bg-white rounded-2xl p-4 sm:p-6 lg:p-8 xl:p-12 shadow-sm border border-gray-200 mb-8 sm:mb-10 lg:mb-12">
          <h2 className="text-xl sm:text-2xl lg:text-3xl font-bold text-gray-900 mb-4 sm:mb-6">Our Story</h2>
          <div className="space-y-3 sm:space-y-4 text-sm sm:text-base text-gray-600 leading-relaxed">
            <p>
              AfriTokeni was born out of frustration. We watched as 54% of Uganda&apos;s adults—14.6 million people—remained excluded from the financial system, while big players charged exorbitant fees and left rural communities behind.
            </p>
            <p>
              The mobile money market in Africa is worth $133 billion and growing at 25.73% annually, yet the people who need it most pay the highest prices. Traditional banks won&apos;t serve remote villages. Mobile money agents charge up to 5% per transaction. And Bitcoin? Forget about it—you need a smartphone and internet.
            </p>
            <p>
              We said: <strong>enough is enough</strong>.
            </p>
            <p>
              Built for the World Hackathon League&apos;s ICP track, AfriTokeni leverages the Internet Computer Protocol to create a truly decentralized, USSD-accessible financial system. No internet required. No smartphone needed. Just your phone and a dream of financial freedom.
            </p>
          </div>
        </div>

        {/* Mission & Vision */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 sm:gap-6 mb-8 sm:mb-10 lg:mb-12">
          <div className="bg-gradient-to-br from-blue-50 to-cyan-50 rounded-2xl p-4 sm:p-6 lg:p-8 border-2 border-blue-200">
            <Target className="w-8 h-8 sm:w-10 sm:h-10 lg:w-12 lg:h-12 text-blue-600 mb-3 sm:mb-4" />
            <h3 className="text-lg sm:text-xl lg:text-2xl font-bold text-gray-900 mb-3 sm:mb-4">Our Mission</h3>
            <p className="text-sm sm:text-base text-gray-600 leading-relaxed">
              Provide USSD-accessible Bitcoin banking to every African, regardless of location, phone type, or internet access. Financial inclusion is a right, not a privilege.
            </p>
          </div>
          <div className="bg-gradient-to-br from-purple-50 to-pink-50 rounded-2xl p-4 sm:p-6 lg:p-8 border-2 border-purple-200">
            <Globe className="w-8 h-8 sm:w-10 sm:h-10 lg:w-12 lg:h-12 text-purple-600 mb-3 sm:mb-4" />
            <h3 className="text-lg sm:text-xl lg:text-2xl font-bold text-gray-900 mb-3 sm:mb-4">Our Vision</h3>
            <p className="text-sm sm:text-base text-gray-600 leading-relaxed">
              A world where every person, from Kampala to Karamoja, can save, send, and grow their money without exploitation. Where agents earn fair commissions for serving their communities.
            </p>
          </div>
        </div>

        {/* Why ICP */}
        <div className="bg-white rounded-2xl p-4 sm:p-6 lg:p-8 xl:p-12 shadow-sm border border-gray-200 mb-8 sm:mb-10 lg:mb-12">
          <h2 className="text-xl sm:text-2xl lg:text-3xl font-bold text-gray-900 mb-4 sm:mb-6">Why Internet Computer Protocol?</h2>
          <div className="space-y-3 sm:space-y-4 text-sm sm:text-base text-gray-600 leading-relaxed">
            <p>
              Traditional blockchain solutions require expensive infrastructure and constant internet connectivity. ICP changes the game:
            </p>
            <ul className="space-y-2 sm:space-y-3 ml-4 sm:ml-6">
              <li className="flex items-start gap-2 sm:gap-3">
                <Zap className="w-4 h-4 sm:w-5 sm:h-5 text-orange-600 flex-shrink-0 mt-0.5" />
                <span><strong>True Decentralization:</strong> No AWS, no Google Cloud—just pure blockchain infrastructure</span>
              </li>
              <li className="flex items-start gap-2 sm:gap-3">
                <Zap className="w-4 h-4 sm:w-5 sm:h-5 text-orange-600 flex-shrink-0 mt-0.5" />
                <span><strong>USSD Gateway Integration:</strong> Canisters can process USSD commands without internet</span>
              </li>
              <li className="flex items-start gap-2 sm:gap-3">
                <Zap className="w-4 h-4 sm:w-5 sm:h-5 text-orange-600 flex-shrink-0 mt-0.5" />
                <span><strong>Lightning Fast:</strong> Sub-second finality for instant transfers</span>
              </li>
              <li className="flex items-start gap-2 sm:gap-3">
                <Zap className="w-4 h-4 sm:w-5 sm:h-5 text-orange-600 flex-shrink-0 mt-0.5" />
                <span><strong>83% Cheaper:</strong> Than traditional mobile money solutions</span>
              </li>
            </ul>
          </div>
        </div>

        {/* The Team */}
        <div className="bg-white rounded-2xl p-4 sm:p-6 lg:p-8 xl:p-12 shadow-sm border border-gray-200 mb-8 sm:mb-10 lg:mb-12">
          <h2 className="text-xl sm:text-2xl lg:text-3xl font-bold text-gray-900 mb-4 sm:mb-6">The Team</h2>
          <div className="flex items-start gap-3 sm:gap-4">
            <Users className="w-8 h-8 sm:w-10 sm:h-10 lg:w-12 lg:h-12 text-gray-400 flex-shrink-0" />
            <div className="text-sm sm:text-base text-gray-600">
              <p className="mb-3 sm:mb-4">
                We&apos;re a team of builders, dreamers, and problem-solvers who believe technology should serve humanity—not exploit it.
              </p>
              <p>
                Built with ❤️ for the World Hackathon League, powered by the Internet Computer Protocol, and dedicated to the 400 million unbanked Africans waiting for their chance.
              </p>
            </div>
          </div>
        </div>

        {/* CTA */}
        <div className="bg-gradient-to-r from-orange-600 to-yellow-600 rounded-xl p-6 sm:p-8 lg:p-12 text-center text-white">
          <Heart className="w-10 h-10 sm:w-12 sm:h-12 lg:w-16 lg:h-16 mx-auto mb-4 sm:mb-6" />
          <h2 className="text-xl sm:text-2xl lg:text-3xl font-bold mb-3 sm:mb-4">Join the Revolution</h2>
          <p className="text-base sm:text-lg lg:text-xl mb-4 sm:mb-6 opacity-90">
            Whether you&apos;re a user, agent, or investor—help us bank the unbanked
          </p>
          <div className="flex flex-col sm:flex-row gap-3 sm:gap-4 justify-center">
            <a
              href="mailto:info@afritokeni.com"
              className="bg-white text-orange-600 px-6 sm:px-8 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-medium hover:bg-neutral-100 transition-colors"
            >
              Get in Touch
            </a>
            <a
              href="/become-agent"
              className="bg-white text-orange-600 px-6 sm:px-8 py-2.5 sm:py-3 rounded-lg text-sm sm:text-base font-medium hover:bg-neutral-100 transition-colors"
            >
              Become an Agent
            </a>
          </div>
        </div>
      </div>

      <PublicFooter />
    </div>
  );
};

export default AboutPage;
