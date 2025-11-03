import React, { useState, useEffect } from 'react';
import { MapContainer, TileLayer, Marker, Popup } from 'react-leaflet';
import { Search, MapPin, Star, Phone, Clock, List, MapIcon as Map, Navigation, Users } from 'lucide-react';
import L from 'leaflet';
import { listDocs } from '@junobuild/core';
import { useDemoMode } from '../../context/DemoModeContext';

interface AgentReview {
  id: string;
  agentId: string;
  userId: string;
  userName: string;
  rating: number;
  comment: string;
  transactionId?: string;
  createdAt: string;
}

interface Agent {
  id: string;
  userId: string;
  businessName: string;
  location: {
    country: string;
    state: string;
    city: string;
    address: string;
    coordinates: {
      lat: number;
      lng: number;
    };
  };
  isActive: boolean;
  cashBalance: number;
  digitalBalance: number;
  commissionRate: number;
  createdAt: string;
  rating?: number;
  reviewCount?: number;
  reviews?: AgentReview[];
}

interface UserLocation {
  lat: number;
  lng: number;
}

// Create custom icons for the map
const createUserIcon = () => {
  return L.divIcon({
    html: `
      <div class="relative">
        <div class="absolute inset-0 bg-blue-500 rounded-full animate-ping opacity-75"></div>
        <div class="relative flex items-center justify-center w-10 h-10 bg-blue-600 rounded-full text-white">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/>
            <circle cx="12" cy="7" r="4"/>
          </svg>
        </div>
      </div>
    `,
    className: 'bg-transparent border-none',
    iconSize: [40, 40],
    iconAnchor: [20, 40],
    popupAnchor: [0, -40]
  });
};

const createAgentIcon = (isActive: boolean) => {
  const bgColor = isActive ? 'bg-green-500' : 'bg-gray-400';
  
  return L.divIcon({
    html: `
      <div class="relative flex items-center justify-center w-8 h-8 ${bgColor} rounded-full text-white border-2 border-white shadow-lg">
        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0Z"/>
          <circle cx="12" cy="10" r="3"/>
        </svg>
      </div>
    `,
    className: 'bg-transparent border-none',
    iconSize: [32, 32],
    iconAnchor: [16, 32],
    popupAnchor: [0, -32]
  });
};

const AgentMapPage: React.FC = () => {
  const { isDemoMode } = useDemoMode();
  const [agents, setAgents] = useState<Agent[]>([]);
  const [userLocation, setUserLocation] = useState<UserLocation | null>(null);
  const [selectedAgent, setSelectedAgent] = useState<Agent | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [filterRadius, setFilterRadius] = useState<number>(10); // km
  const [showOnlineOnly, setShowOnlineOnly] = useState(false);
  const [loading, setLoading] = useState(true);
  const [locationPermission, setLocationPermission] = useState<'granted' | 'denied' | 'prompt'>('prompt');
  const [viewMode, setViewMode] = useState<'list' | 'map'>('list');

  // Calculate distance between two coordinates using Haversine formula
  const calculateDistance = (lat1: number, lng1: number, lat2: number, lng2: number): number => {
    const R = 6371; // Earth's radius in kilometers
    const dLat = (lat2 - lat1) * Math.PI / 180;
    const dLng = (lng2 - lng1) * Math.PI / 180;
    const a = 
      Math.sin(dLat/2) * Math.sin(dLat/2) +
      Math.cos(lat1 * Math.PI / 180) * Math.cos(lat2 * Math.PI / 180) * 
      Math.sin(dLng/2) * Math.sin(dLng/2);
    const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1-a));
    return R * c;
  };

  // Get user's current location
  const getUserLocation = () => {
    if (navigator.geolocation) {
      navigator.geolocation.getCurrentPosition(
        (position) => {
          setUserLocation({
            lat: position.coords.latitude,
            lng: position.coords.longitude
          });
          setLocationPermission('granted');
        },
        (error) => {
          console.error('Error getting location:', error);
          setLocationPermission('denied');
          // Don't set any default location - let user search by name only
          setUserLocation(null);
        }
      );
    } else {
      console.error('Geolocation not supported by browser');
      setLocationPermission('denied');
      // Don't set any default location - let user search by name only
      setUserLocation(null);
    }
  };

  // Load agents data from Juno datastore
  useEffect(() => {
    const loadAgents = async () => {
      try {
        setLoading(true);
        
        if (isDemoMode) {
          // Load from /data/agents.json in demo mode
          console.log('Loading agents from /data/agents.json...');
          const [agentsResponse, reviewsResponse] = await Promise.all([
            fetch('/data/agents.json'),
            fetch('/data/agent-reviews.json')
          ]);
          const agentsData = await agentsResponse.json();
          const reviewsData = await reviewsResponse.json();
          
          // Attach reviews and ratings to agents
          const agentsWithReviews = agentsData.map((agent: any) => {
            const agentReviews = reviewsData.filter((r: any) => r.agentId === agent.id);
            const avgRating = agentReviews.length > 0
              ? agentReviews.reduce((sum: number, r: any) => sum + r.rating, 0) / agentReviews.length
              : 0;
            
            return {
              ...agent,
              reviews: agentReviews.slice(0, 3), // Show top 3 reviews
              rating: avgRating,
              reviewCount: agentReviews.length
            };
          });
          
          console.log('Loaded agents with reviews:', agentsWithReviews);
          setAgents(agentsWithReviews);
        } else {
          // Get agents from Juno datastore in production
          console.log('Loading agents from Juno datastore...');
          const result = await listDocs({
            collection: 'agents'
          });
          
          console.log('Raw agent docs:', result);
          
          // Transform Juno docs to Agent interface
          const agentsData = result.items.map((doc: any) => {
            const data = doc.data;
            return {
              id: doc.key,
              userId: data.userId || '',
              businessName: data.businessName || 'Unknown Agent',
              location: {
                country: data.location?.country || 'Uganda',
                state: data.location?.state || 'Central',
                city: data.location?.city || 'Kampala',
                address: data.location?.address || 'Address not available',
                coordinates: {
                  lat: data.location?.coordinates?.lat || (0.3476 + (Math.random() - 0.5) * 0.1),
                  lng: data.location?.coordinates?.lng || (32.5825 + (Math.random() - 0.5) * 0.1)
                }
              },
              isActive: data.status === 'available' || data.status === 'online',
              cashBalance: data.cashBalance || 0,
              digitalBalance: data.digitalBalance || 0,
              commissionRate: data.commissionRate || 0.025,
              createdAt: data.createdAt || new Date().toISOString()
            };
          });
          
          console.log('Transformed agents:', agentsData);
          setAgents(agentsData);
        }
        
      } catch (error) {
        console.error('Error loading agents:', error);
        setAgents([]);
      } finally {
        setLoading(false);
      }
    };

    loadAgents();
    getUserLocation();
  }, [isDemoMode]);

  // Filter agents based on search, distance, and online status
  const filteredAgents = agents.filter(agent => {
    // Search filter
    if (searchQuery && !agent.businessName.toLowerCase().includes(searchQuery.toLowerCase()) &&
        !agent.location.address.toLowerCase().includes(searchQuery.toLowerCase())) {
      return false;
    }

    // Online status filter
    if (showOnlineOnly && !agent.isActive) {
      return false;
    }

    // Distance filter
    if (userLocation) {
      const distance = calculateDistance(
        userLocation.lat,
        userLocation.lng,
        agent.location.coordinates.lat,
        agent.location.coordinates.lng
      );
      if (distance > filterRadius) {
        return false;
      }
    }

    return true;
  });

  // Sort agents by distance from user
  const sortedAgents = userLocation ? filteredAgents.sort((a, b) => {
    const distanceA = calculateDistance(
      userLocation.lat,
      userLocation.lng,
      a.location.coordinates.lat,
      a.location.coordinates.lng
    );
    const distanceB = calculateDistance(
      userLocation.lat,
      userLocation.lng,
      b.location.coordinates.lat,
      b.location.coordinates.lng
    );
    return distanceA - distanceB;
  }) : filteredAgents;

  const formatDistance = (agent: Agent): string => {
    if (!userLocation) return '';
    const distance = calculateDistance(
      userLocation.lat,
      userLocation.lng,
      agent.location.coordinates.lat,
      agent.location.coordinates.lng
    );
    return distance < 1 ? `${Math.round(distance * 1000)}m` : `${distance.toFixed(1)}km`;
  };

  const formatBalance = (amount: number): string => {
    if (amount >= 1000000) {
      return `${(amount / 1000000).toFixed(1)}M`;
    } else if (amount >= 1000) {
      return `${(amount / 1000).toFixed(0)}K`;
    }
    return amount.toString();
  };

  if (loading) {
    return (
      <div className="space-y-6">
        <div className="flex items-center justify-center min-h-screen">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-gray-900"></div>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-4 sm:space-y-6">
        {/* Search and Filters */}
        <div className="bg-white rounded-xl sm:rounded-2xl shadow-sm border border-gray-200 p-4 sm:p-6 mb-6 sm:mb-8">
          <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between mb-4 sm:mb-6 gap-3">
            <h2 className="text-base sm:text-lg font-semibold text-gray-900">Search & Filter Agents</h2>
            {/* View Mode Toggle */}
            <div className="flex items-center bg-gray-100 rounded-lg p-1 w-full sm:w-auto">
              <button
                onClick={() => setViewMode('list')}
                className={`flex items-center justify-center flex-1 sm:flex-initial px-3 py-2 rounded-md text-xs sm:text-sm font-medium transition-colors duration-200 ${
                  viewMode === 'list'
                    ? 'bg-white text-gray-900 shadow-sm'
                    : 'text-gray-600 hover:text-gray-900'
                }`}
              >
                <List className="h-3.5 w-3.5 sm:h-4 sm:w-4 mr-1.5 sm:mr-2" />
                List
              </button>
              <button
                onClick={() => setViewMode('map')}
                className={`flex items-center justify-center flex-1 sm:flex-initial px-3 py-2 rounded-md text-xs sm:text-sm font-medium transition-colors duration-200 ${
                  viewMode === 'map'
                    ? 'bg-white text-gray-900 shadow-sm'
                    : 'text-gray-600 hover:text-gray-900'
                }`}
              >
                <Map className="h-3.5 w-3.5 sm:h-4 sm:w-4 mr-1.5 sm:mr-2" />
                Map
              </button>
            </div>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-3 sm:gap-4">
            {/* Search */}
            <div className="relative md:col-span-1">
              <Search className="absolute left-2.5 sm:left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 sm:h-5 sm:w-5 text-gray-400" />
              <input
                type="text"
                placeholder="Search agents or locations..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full pl-8 sm:pl-10 pr-3 sm:pr-4 py-2 text-sm sm:text-base border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500 focus:border-gray-500"
              />
            </div>

            {/* Distance Filter */}
            <div>
              <select
                value={filterRadius}
                onChange={(e) => setFilterRadius(Number(e.target.value))}
                disabled={!userLocation}
                className={`w-full px-2.5 sm:px-3 py-2 text-sm sm:text-base border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-500 focus:border-gray-500 ${
                  !userLocation ? 'bg-gray-100 text-gray-400 cursor-not-allowed' : ''
                }`}
              >
                <option value={5}>Within 5km</option>
                <option value={10}>Within 10km</option>
                <option value={20}>Within 20km</option>
                <option value={50}>Within 50km</option>
                <option value={100}>Within 100km</option>
              </select>
            </div>

            {/* Online Status Filter */}
            <div className="flex items-center">
              <input
                type="checkbox"
                id="onlineOnly"
                checked={showOnlineOnly}
                onChange={(e) => setShowOnlineOnly(e.target.checked)}
                className="h-3.5 w-3.5 sm:h-4 sm:w-4 text-gray-600 focus:ring-gray-500 border-gray-300 rounded"
              />
              <label htmlFor="onlineOnly" className="ml-2 text-xs sm:text-sm text-gray-700">
                Online agents only
              </label>
            </div>
          </div>

          {/* Location Status - Moved below filters */}
          <div className="mt-3 sm:mt-4 flex flex-col sm:flex-row items-start sm:items-center justify-between text-xs sm:text-sm gap-2">
            {locationPermission === 'granted' ? (
              <span className="text-green-600 font-medium flex items-center">
                <Navigation className="h-3.5 w-3.5 sm:h-4 sm:w-4 mr-1.5 sm:mr-2 flex-shrink-0" />
                <span className="break-words">📍 Location enabled - showing agents near you</span>
              </span>
            ) : (
              <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between w-full gap-2">
                <span className="text-orange-600 font-medium flex items-center">
                  <Navigation className="h-3.5 w-3.5 sm:h-4 sm:w-4 mr-1.5 sm:mr-2 flex-shrink-0" />
                  <span className="break-words">⚠️ Location disabled - distance filter unavailable</span>
                </span>
                <button
                  onClick={getUserLocation}
                  className="text-xs sm:text-sm text-blue-600 hover:text-blue-800 underline whitespace-nowrap"
                >
                  Enable location
                </button>
              </div>
            )}
          </div>
        </div>

        {/* Results Summary */}
        <div className="mb-4 sm:mb-6">
          <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-2">
            <p className="text-sm sm:text-base text-gray-600">
              Found {sortedAgents.length} agent{sortedAgents.length !== 1 ? 's' : ''} 
              {userLocation && ` within ${filterRadius}km`}
            </p>
            <div className="flex items-center text-xs sm:text-sm text-gray-500">
              <Users className="h-3.5 w-3.5 sm:h-4 sm:w-4 mr-1 flex-shrink-0" />
              {sortedAgents.filter(a => a.isActive).length} online
            </div>
          </div>
        </div>

        {/* Agent Display */}
        {viewMode === 'list' ? (
          /* Agent Grid */
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6">
          {sortedAgents.map((agent) => (
            <div
              key={agent.id}
              className={`bg-white rounded-xl sm:rounded-2xl shadow-sm border border-gray-200 p-4 sm:p-6 hover:shadow-md transition-shadow duration-200 cursor-pointer ${
                selectedAgent?.id === agent.id ? 'ring-2 ring-gray-500' : ''
              }`}
              onClick={() => setSelectedAgent(selectedAgent?.id === agent.id ? null : agent)}
            >
              {/* Agent Header */}
              <div className="flex items-start justify-between mb-3 sm:mb-4 gap-2">
                <div className="flex-1 min-w-0">
                  <h3 className="text-sm sm:text-base font-semibold text-gray-900 mb-1 break-words">
                    {agent.businessName}
                  </h3>
                  <div className="flex items-start text-xs sm:text-sm text-gray-600 mb-2">
                    <MapPin className="h-3.5 w-3.5 sm:h-4 sm:w-4 mr-1 flex-shrink-0 mt-0.5" />
                    <span className="break-words">{agent.location.address}</span>
                  </div>
                  {userLocation && (
                    <div className="flex items-center text-xs sm:text-sm text-gray-500">
                      <Navigation className="h-3.5 w-3.5 sm:h-4 sm:w-4 mr-1 flex-shrink-0" />
                      <span className="whitespace-nowrap">{formatDistance(agent)} away</span>
                    </div>
                  )}
                </div>
                <div className={`px-2 py-1 rounded-full text-xs font-medium whitespace-nowrap flex-shrink-0 ${
                  agent.isActive 
                    ? 'bg-green-100 text-green-800' 
                    : 'bg-gray-100 text-gray-600'
                }`}>
                  {agent.isActive ? 'Online' : 'Offline'}
                </div>
              </div>

              {/* Agent Stats */}
              <div className="grid grid-cols-2 gap-3 sm:gap-4 mb-3 sm:mb-4">
                <div className="min-w-0">
                  <p className="text-xs text-gray-500 mb-1">Cash Available</p>
                  <p className="text-sm sm:text-base font-mono font-semibold text-gray-900 break-all">
                    {formatBalance(agent.cashBalance)} UGX
                  </p>
                </div>
                <div className="min-w-0">
                  <p className="text-xs text-gray-500 mb-1">Commission</p>
                  <p className="text-sm sm:text-base font-mono font-semibold text-gray-900">
                    {(agent.commissionRate < 1 ? agent.commissionRate * 100 : agent.commissionRate).toFixed(1)}%
                  </p>
                </div>
              </div>

              {/* Rating */}
              {agent.rating && agent.reviewCount ? (
                <div className="mb-3 sm:mb-4">
                  <div className="flex items-center mb-2">
                    <div className="flex items-center flex-shrink-0">
                      {[1, 2, 3, 4, 5].map((star) => (
                        <Star
                          key={star}
                          className={`h-3.5 w-3.5 sm:h-4 sm:w-4 ${
                            star <= Math.round(agent.rating!) ? 'text-yellow-400 fill-current' : 'text-gray-300'
                          }`}
                        />
                      ))}
                    </div>
                    <span className="ml-2 text-xs sm:text-sm text-gray-600 whitespace-nowrap">
                      {agent.rating.toFixed(1)} ({agent.reviewCount} reviews)
                    </span>
                  </div>
                  
                  {/* Recent Reviews */}
                  {agent.reviews && agent.reviews.length > 0 && selectedAgent?.id === agent.id && (
                    <div className="space-y-2 mt-2 sm:mt-3 max-h-48 overflow-y-auto">
                      <p className="text-xs font-medium text-gray-700 mb-2">Recent Reviews</p>
                      {agent.reviews.map((review) => (
                        <div key={review.id} className="bg-gray-50 rounded-lg p-2.5 sm:p-3 text-xs">
                          <div className="flex items-center justify-between mb-1 gap-2">
                            <span className="font-medium text-gray-900 truncate">{review.userName}</span>
                            <div className="flex items-center flex-shrink-0">
                              {[1, 2, 3, 4, 5].map((star) => (
                                <Star
                                  key={star}
                                  className={`h-2.5 w-2.5 sm:h-3 sm:w-3 ${
                                    star <= review.rating ? 'text-yellow-400 fill-current' : 'text-gray-300'
                                  }`}
                                />
                              ))}
                            </div>
                          </div>
                          <p className="text-gray-600 break-words">{review.comment}</p>
                          <p className="text-gray-400 mt-1 text-xs">
                            {new Date(review.createdAt).toLocaleDateString()}
                          </p>
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              ) : (
                <div className="flex items-center mb-3 sm:mb-4 text-xs sm:text-sm text-gray-500">
                  <span>No reviews yet</span>
                </div>
              )}

              {/* Services */}
              <div className="flex flex-wrap gap-1.5 sm:gap-2 mb-3 sm:mb-4">
                <span className="px-2 py-1 bg-blue-100 text-blue-800 text-xs rounded-full whitespace-nowrap">
                  Cash Deposit
                </span>
                <span className="px-2 py-1 bg-green-100 text-green-800 text-xs rounded-full whitespace-nowrap">
                  Withdrawal
                </span>
                <span className="px-2 py-1 bg-orange-100 text-orange-800 text-xs rounded-full whitespace-nowrap">
                  Bitcoin Exchange
                </span>
              </div>

              {/* Action Buttons */}
              <div className="flex gap-2">
                <button className="flex-1 bg-gray-900 text-white px-3 sm:px-4 py-2 rounded-lg text-xs sm:text-sm font-medium hover:bg-gray-800 transition-colors duration-200">
                  Contact Agent
                </button>
                <button className="px-3 sm:px-4 py-2 border border-gray-300 text-gray-700 rounded-lg text-sm font-medium hover:bg-gray-50 transition-colors duration-200">
                  <Phone className="h-3.5 w-3.5 sm:h-4 sm:w-4" />
                </button>
              </div>

              {/* Expanded Details */}
              {selectedAgent?.id === agent.id && (
                <div className="mt-3 sm:mt-4 pt-3 sm:pt-4 border-t border-gray-200">
                  <div className="space-y-2.5 sm:space-y-3">
                    <div>
                      <p className="text-xs text-gray-500 mb-1">Operating Hours</p>
                      <div className="flex items-center text-xs sm:text-sm text-gray-700">
                        <Clock className="h-3.5 w-3.5 sm:h-4 sm:w-4 mr-1.5 sm:mr-2 flex-shrink-0" />
                        <span className="break-words">Mon-Sat: 8:00 AM - 8:00 PM</span>
                      </div>
                    </div>
                    <div>
                      <p className="text-xs text-gray-500 mb-1">Services Available</p>
                      <ul className="text-xs sm:text-sm text-gray-700 space-y-1">
                        <li className="break-words">• Cash deposits and withdrawals</li>
                        <li className="break-words">• Bitcoin buying and selling</li>
                        <li className="break-words">• Money transfers</li>
                        <li className="break-words">• Account verification</li>
                      </ul>
                    </div>
                    <div>
                      <p className="text-xs text-gray-500 mb-1">Contact Information</p>
                      <p className="text-xs sm:text-sm text-gray-700 break-words">+256 700 123 456</p>
                      <p className="text-xs sm:text-sm text-gray-700 break-all">agent@afritokeni.com</p>
                    </div>
                  </div>
                </div>
              )}
            </div>
          ))}
          </div>
        ) : (
          /* Map View */
          <div className="bg-white rounded-xl sm:rounded-2xl shadow-sm border border-gray-200 p-4 sm:p-6">
            <div className="h-[400px] sm:h-[500px] lg:h-[600px] rounded-lg overflow-hidden">
              <MapContainer
                center={userLocation ? [userLocation.lat, userLocation.lng] : [0, 20]}
                zoom={userLocation ? 13 : 3}
                style={{ height: '100%', width: '100%' }}
                className="rounded-lg"
              >
                <TileLayer
                  attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
                  url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
                />
                
                {/* User Location Marker */}
                {userLocation && (
                  <Marker
                    position={[userLocation.lat, userLocation.lng]}
                    icon={createUserIcon()}
                  >
                    <Popup>
                      <div className="text-center">
                        <div className="font-semibold text-blue-600">Your Location</div>
                        <div className="text-sm text-gray-600">
                          {locationPermission === 'granted' ? 'Current location' : 'Default location (Kampala)'}
                        </div>
                      </div>
                    </Popup>
                  </Marker>
                )}

                {/* Agent Markers */}
                {sortedAgents.map((agent) => (
                  <Marker
                    key={agent.id}
                    position={[agent.location.coordinates.lat, agent.location.coordinates.lng]}
                    icon={createAgentIcon(agent.isActive)}
                    eventHandlers={{
                      click: () => setSelectedAgent(selectedAgent?.id === agent.id ? null : agent)
                    }}
                  >
                    <Popup>
                      <div className="min-w-48">
                        <div className="font-semibold text-gray-900 mb-2">
                          {agent.businessName}
                        </div>
                        <div className="text-sm text-gray-600 mb-2">
                          {agent.location.address}
                        </div>
                        <div className="flex items-center justify-between mb-3">
                          <span className={`px-2 py-1 rounded-full text-xs ${
                            agent.isActive ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-600'
                          }`}>
                            {agent.isActive ? 'Online' : 'Offline'}
                          </span>
                          {userLocation && (
                            <span className="text-sm text-gray-500">
                              {formatDistance(agent)} away
                            </span>
                          )}
                        </div>
                        <div className="grid grid-cols-2 gap-2 text-xs mb-3">
                          <div>
                            <span className="text-gray-500">Cash:</span>
                            <div className="font-mono font-semibold">{formatBalance(agent.cashBalance)} UGX</div>
                          </div>
                          <div>
                            <span className="text-gray-500">Fee:</span>
                            <div className="font-mono font-semibold">{agent.commissionRate > 0 ? agent.commissionRate : (agent.commissionRate * 100).toFixed(1)}%</div>
                          </div>
                        </div>
                        <button
                          onClick={() => setSelectedAgent(agent)}
                          className="w-full bg-gray-900 text-white px-3 py-2 rounded-lg text-sm font-medium hover:bg-gray-800 transition-colors duration-200"
                        >
                          Contact Agent
                        </button>
                      </div>
                    </Popup>
                  </Marker>
                ))}
              </MapContainer>
            </div>
            
            {/* Selected Agent Details */}
            {selectedAgent && viewMode === 'map' && (
              <div className="mt-4 sm:mt-6 bg-gray-50 rounded-lg p-3 sm:p-4">
                <h3 className="text-sm sm:text-base font-semibold text-gray-900 mb-2 sm:mb-3 break-words">{selectedAgent.businessName}</h3>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-3 sm:gap-4 text-xs sm:text-sm">
                  <div className="min-w-0">
                    <span className="text-gray-500">Address:</span>
                    <p className="text-gray-900 break-words">{selectedAgent.location.address}</p>
                  </div>
                  <div className="min-w-0">
                    <span className="text-gray-500">Cash Available:</span>
                    <p className="font-mono text-gray-900 break-all">{formatBalance(selectedAgent.cashBalance)} UGX</p>
                  </div>
                  <div className="min-w-0">
                    <span className="text-gray-500">Commission:</span>
                    <p className="font-mono text-gray-900">{selectedAgent.commissionRate < 1 ? (selectedAgent.commissionRate * 100).toFixed(1) : selectedAgent.commissionRate}%</p>
                  </div>
                </div>
                <div className="flex flex-col sm:flex-row gap-2 mt-3 sm:mt-4">
                  <button className="bg-gray-900 text-white px-4 py-2 rounded-lg text-xs sm:text-sm font-medium hover:bg-gray-800 transition-colors duration-200">
                    Contact Agent
                  </button>
                  <button 
                    onClick={() => setSelectedAgent(null)}
                    className="px-4 py-2 border border-gray-300 text-gray-700 rounded-lg text-xs sm:text-sm font-medium hover:bg-gray-50 transition-colors duration-200"
                  >
                    Close
                  </button>
                </div>
              </div>
            )}
          </div>
        )}

        {/* Empty State */}
        {sortedAgents.length === 0 && (
          <div className="text-center py-8 sm:py-12">
            <MapPin className="h-10 w-10 sm:h-12 sm:w-12 text-gray-400 mx-auto mb-3 sm:mb-4" />
            <h3 className="text-base sm:text-lg font-medium text-gray-900 mb-2">No agents found</h3>
            <p className="text-sm sm:text-base text-gray-600 mb-4 px-4">
              Try adjusting your search criteria or increasing the distance radius.
            </p>
            <button
              onClick={() => {
                setSearchQuery('');
                setFilterRadius(50);
                setShowOnlineOnly(false);
              }}
              className="bg-gray-900 text-white px-4 sm:px-6 py-2 rounded-lg text-sm sm:text-base font-medium hover:bg-gray-800 transition-colors duration-200"
            >
              Clear Filters
            </button>
          </div>
        )}

    </div>
  );
};

export default AgentMapPage;
