import React, { useEffect, useState } from 'react';
import { MapPin, Star, List, MapIcon as Map, X, Navigation, Phone, Clock } from 'lucide-react';
import { MapContainer, TileLayer, Marker, useMap } from 'react-leaflet';
import L from 'leaflet';
import 'leaflet/dist/leaflet.css';
import { formatCurrencyAmount, type AfricanCurrency } from '../../types/currency';

// Fix for default Leaflet icon paths in Vite/Webpack
delete (L.Icon.Default.prototype as any)._getIconUrl;
L.Icon.Default.mergeOptions({
  iconRetinaUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.7.1/images/marker-icon-2x.png',
  iconUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.7.1/images/marker-icon.png',
  shadowUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.7.1/images/marker-shadow.png',
});
import { AgentSelectionStepProps } from '../../types/depositTypes';
import { Agent } from '../../services/agentService';

// No need for leaflet popup CSS since we're using custom React popup

const AgentSelectionStep: React.FC<AgentSelectionStepProps> = ({
  agents,
  selectedAgent,
  amount,
  selectedCurrency,
  userLocation,
  viewMode,
  isLoadingAgents,
  isCreating,
  onAgentSelect,
  onViewModeChange,
  onCreateDepositRequest,
}) => {
  // Ensure unique agents to prevent duplicates
  const uniqueAgents = agents.filter((agent, index, self) => 
    index === self.findIndex(a => a.id === agent.id)
  );
  
  // Debug log to track agent duplication and map rendering
  console.log('AgentSelectionStep rendered with agents:', agents.length, 'unique:', uniqueAgents.length);
  if (agents.length !== uniqueAgents.length) {
    console.warn('Duplicate agents detected!', agents.map(a => a.id));
  }
  
  // Map rendering debug
  useEffect(() => {
    console.log('🗺️ View mode changed to:', viewMode);
    console.log('🗺️ User location:', userLocation);
    
    if (viewMode === 'map') {
      // Give the map container time to render then check if Leaflet is working
      setTimeout(() => {
        const mapContainers = document.querySelectorAll('.leaflet-container');
        console.log('🗺️ Found Leaflet containers:', mapContainers.length);
        if (mapContainers.length === 0) {
          console.error('🗺️ No Leaflet containers found - map may not be rendering');
        } else {
          console.log('🗺️ Map container found, checking tiles...');
          const tiles = document.querySelectorAll('.leaflet-tile');
          console.log('🗺️ Found tiles:', tiles.length);
        }
      }, 1000);
    }
  }, [viewMode, userLocation]);

  // State for custom popup that bypasses leaflet z-index issues
  const [popupAgent, setPopupAgent] = useState<Agent | null>(null);
  const [popupPosition, setPopupPosition] = useState<{ x: number; y: number }>({ x: 0, y: 0 });

  // Handle marker clicks to show custom popup
  const handleMarkerClick = (agent: Agent, event: L.LeafletMouseEvent) => {
    // Get the marker's screen position
    const marker = event.target;
    const map = marker._map;
    const point = map.latLngToContainerPoint(marker.getLatLng());
    
    // Get the map container's position
    const mapContainer = map.getContainer();
    const mapRect = mapContainer.getBoundingClientRect();
    
    // Calculate popup position relative to viewport
    setPopupPosition({
      x: mapRect.left + point.x,
      y: mapRect.top + point.y - 10 // Offset to appear above marker
    });
    setPopupAgent(agent);
    onAgentSelect(agent);
  };

  // Close popup
  const closePopup = () => {
    setPopupAgent(null);
  };

  // Map helper functions
  const createBlinkingUserIcon = () => {
    return L.divIcon({
      html: `
        <div class="relative">
          <div class="absolute inset-0 bg-blue-500 rounded-full animate-ping opacity-75"></div>
          <div className="relative flex items-center justify-center w-10 h-10 bg-blue-600 rounded-full text-white">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
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
        <div className="relative flex items-center justify-center w-8 h-8 ${bgColor} rounded-full text-white border-2 border-white shadow-lg">
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


  const calculateDistance = (lat1: number, lon1: number, lat2: number, lon2: number): string => {
    const R = 6371;
    const dLat = (lat2 - lat1) * Math.PI / 180;
    const dLon = (lon2 - lon1) * Math.PI / 180;
    const a = 
      Math.sin(dLat/2) * Math.sin(dLat/2) +
      Math.cos(lat1 * Math.PI / 180) * Math.cos(lat2 * Math.PI / 180) * 
      Math.sin(dLon/2) * Math.sin(dLon/2);
    const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1-a));
    const distance = R * c;
    return distance < 1 ? `${(distance * 1000).toFixed(0)} meters` : `${distance.toFixed(1)} km`;
  };

  const formatDistance = (agent: Agent): string => {
    if (!userLocation) return '';
    const distance = parseFloat(calculateDistance(
      userLocation[0],
      userLocation[1],
      agent.location.coordinates.lat,
      agent.location.coordinates.lng
    ).replace(' km', '').replace(' meters', ''));
    
    if (calculateDistance(userLocation[0], userLocation[1], agent.location.coordinates.lat, agent.location.coordinates.lng).includes('meters')) {
      return `${Math.round(distance)}m`;
    }
    return `${distance.toFixed(1)}km`;
  };

  const formatBalance = (amount: number): string => {
    if (amount >= 1000000) {
      return `${(amount / 1000000).toFixed(1)}M`;
    } else if (amount >= 1000) {
      return `${(amount / 1000).toFixed(0)}K`;
    }
    return amount.toString();
  };

  const CenterMap = ({ center }: { center: [number, number] }) => {
    const map = useMap();
    useEffect(() => {
      map.setView(center, 13);
    }, [center, map]);
    return null;
  };

  const renderAgentDetails = (agent: typeof agents[0]) => (
    <div className="min-w-48">
      {/* Agent Header */}
      <div className="flex items-start justify-between mb-3 sm:mb-4">
        <div className="flex-1 min-w-0">
          <h3 className="font-semibold text-gray-900 mb-1 text-xs sm:text-sm break-words">
            {agent.businessName}
          </h3>
          <div className="flex items-center text-xs sm:text-sm text-gray-600 mb-1.5 sm:mb-2">
            <MapPin className="h-3 w-3 sm:h-4 sm:w-4 mr-1 flex-shrink-0" />
            <span className="break-words">{agent.location.address}</span>
          </div>
          {userLocation && (
            <div className="flex items-center text-xs sm:text-sm text-gray-500">
              <Navigation className="h-3 w-3 sm:h-4 sm:w-4 mr-1 flex-shrink-0" />
              {formatDistance(agent)} away
            </div>
          )}
        </div>
        <div className={`px-1.5 sm:px-2 py-0.5 sm:py-1 rounded-full text-xs font-medium flex-shrink-0 ${
          agent.status === 'available'
            ? 'bg-green-100 text-green-800' 
            : agent.status === 'busy'
            ? 'bg-yellow-100 text-yellow-800'
            : 'bg-gray-100 text-gray-600'
        }`}>
          {agent.status === 'available' ? 'Online' : agent.status === 'busy' ? 'Busy' : 'Offline'}
        </div>
      </div>

      {/* Agent Stats */}
      <div className="grid grid-cols-2 gap-2 sm:gap-3 md:gap-4 mb-3 sm:mb-4">
        <div>
          <p className="text-xs text-gray-500 mb-0.5 sm:mb-1">Digital Balance</p>
          <p className="font-mono font-semibold text-gray-900 text-xs sm:text-sm break-words">
            {formatBalance(agent.digitalBalance || 0)} {selectedCurrency || 'UGX'}
          </p>
        </div>
        <div>
          <p className="text-xs text-gray-500 mb-0.5 sm:mb-1">Cash Available</p>
          <p className="font-mono font-semibold text-gray-900 text-xs sm:text-sm break-words">
            {formatBalance(agent.cashBalance || 0)} {selectedCurrency || 'UGX'}
          </p>
        </div>
      </div>

      {/* Rating (Mock) */}
      <div className="flex items-center mb-3 sm:mb-4">
        <div className="flex items-center">
          {[1, 2, 3, 4, 5].map((star) => (
            <Star
              key={star}
              className={`h-3 w-3 sm:h-4 sm:w-4 flex-shrink-0 ${
                star <= 4 ? 'text-yellow-400 fill-current' : 'text-gray-300'
              }`}
            />
          ))}
        </div>
        <span className="ml-1.5 sm:ml-2 text-xs sm:text-sm text-gray-600">4.0 (23 reviews)</span>
      </div>

      {/* Services */}
      <div className="flex flex-wrap gap-1.5 sm:gap-2 mb-3 sm:mb-4">
        <span className="px-1.5 sm:px-2 py-0.5 sm:py-1 bg-blue-100 text-blue-800 text-xs rounded-full">
          Cash Deposit
        </span>
        <span className="px-1.5 sm:px-2 py-0.5 sm:py-1 bg-green-100 text-green-800 text-xs rounded-full">
          Withdrawal
        </span>
        <span className="px-1.5 sm:px-2 py-0.5 sm:py-1 bg-orange-100 text-orange-800 text-xs rounded-full">
          Bitcoin Exchange
        </span>
      </div>

      {/* Action Buttons */}
      <div className="flex gap-2">
        <button
          onClick={(e) => {
            e.stopPropagation();
            onCreateDepositRequest(agent);
          }}
          disabled={isCreating}
          className="flex-1 bg-gray-900 text-white px-3 sm:px-4 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-medium hover:bg-gray-800 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors duration-200"
        >
          {isCreating ? 'Creating...' : 'Select Agent'}
        </button>
        <button className="px-3 sm:px-4 py-1.5 sm:py-2 border border-gray-300 text-gray-700 rounded-lg text-xs sm:text-sm font-medium hover:bg-gray-50 transition-colors duration-200">
          <Phone className="h-3.5 w-3.5 sm:h-4 sm:w-4 flex-shrink-0" />
        </button>
      </div>
    </div>
  );

  return (
    <div className="max-w-4xl mx-auto">
      <div className="bg-white rounded-lg sm:rounded-xl shadow-sm border border-gray-200 p-3 sm:p-4 md:p-6 lg:p-8">
        <div className="text-center mb-4 sm:mb-5 md:mb-6 lg:mb-8">
          <h2 className="text-lg sm:text-xl md:text-2xl lg:text-3xl font-bold text-gray-900 mb-1.5 sm:mb-2">Select Agent</h2>
          <p className="text-xs sm:text-sm md:text-base text-gray-600 break-words">
            Choose an agent to deposit {formatCurrencyAmount(parseFloat(amount), selectedCurrency as AfricanCurrency)}
          </p>
        </div>

        {/* View Toggle */}
        <div className="flex justify-center mb-3 sm:mb-4 md:mb-5 lg:mb-6">
          <div className="bg-gray-100 p-0.5 sm:p-1 rounded-lg">
            <button
              type="button"
              onClick={() => onViewModeChange('list')}
              className={`px-2.5 sm:px-3 md:px-4 py-1 sm:py-1.5 md:py-2 text-[10px] sm:text-xs md:text-sm font-medium rounded-md transition-all ${
                viewMode === 'list'
                  ? 'bg-white text-gray-900 shadow-sm'
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              <List className="w-3 h-3 sm:w-3.5 sm:h-3.5 md:w-4 md:h-4 inline mr-1 sm:mr-1.5 md:mr-2 flex-shrink-0" />
              List View
            </button>
            <button
              type="button"
              onClick={() => onViewModeChange('map')}
              className={`px-2.5 sm:px-3 md:px-4 py-1 sm:py-1.5 md:py-2 text-[10px] sm:text-xs md:text-sm font-medium rounded-md transition-all ${
                viewMode === 'map'
                  ? 'bg-white text-gray-900 shadow-sm'
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              <Map className="w-3 h-3 sm:w-3.5 sm:h-3.5 md:w-4 md:h-4 inline mr-1 sm:mr-1.5 md:mr-2 flex-shrink-0" />
              Map View
            </button>
          </div>
        </div>

        {isLoadingAgents ? (
          <div className="text-center py-4 sm:py-6 md:py-8">
            <div className="inline-flex items-center space-x-1.5 sm:space-x-2 text-gray-600 text-[10px] sm:text-xs md:text-sm">
              <div className="animate-spin rounded-full h-3.5 w-3.5 sm:h-4 sm:w-4 border-b-2 border-gray-900"></div>
              <span>Finding nearby agents...</span>
            </div>
          </div>
        ) : viewMode === 'list' ? (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3 sm:gap-4 md:gap-5 lg:gap-6">
            {uniqueAgents.map((agent) => (
              <div
                key={agent.id}
                className={`bg-white rounded-xl shadow-sm border border-gray-200 p-4 sm:p-5 md:p-6 hover:shadow-md transition-shadow duration-200 cursor-pointer ${
                  selectedAgent?.id === agent.id ? 'ring-2 ring-blue-500' : ''
                }`}
                onClick={() => {
                  if (selectedAgent?.id === agent.id) {
                    // Deselect if clicking the same agent
                    onAgentSelect(agent);
                  } else {
                    // Select the new agent
                    onAgentSelect(agent);
                  }
                }}
              >
                {/* Agent Header */}
                <div className="flex items-start justify-between mb-3 sm:mb-4">
                  <div className="flex-1 min-w-0">
                    <h3 className="font-semibold text-gray-900 mb-1 text-sm sm:text-base break-words">
                      {agent.businessName}
                    </h3>
                    <div className="flex items-center text-xs sm:text-sm text-gray-600 mb-1.5 sm:mb-2">
                      <MapPin className="h-3.5 w-3.5 sm:h-4 sm:w-4 mr-1 flex-shrink-0" />
                      <span className="break-words">{agent.location.address}</span>
                    </div>
                    {userLocation && (
                      <div className="flex items-center text-xs sm:text-sm text-gray-500">
                        <Navigation className="h-3.5 w-3.5 sm:h-4 sm:w-4 mr-1 flex-shrink-0" />
                        {formatDistance(agent)} away
                      </div>
                    )}
                  </div>
                  <div className={`px-2 py-1 rounded-full text-xs font-medium flex-shrink-0 ${
                    agent.status === 'available'
                      ? 'bg-green-100 text-green-800' 
                      : agent.status === 'busy'
                      ? 'bg-yellow-100 text-yellow-800'
                      : 'bg-gray-100 text-gray-600'
                  }`}>
                    {agent.status === 'available' ? 'Online' : agent.status === 'busy' ? 'Busy' : 'Offline'}
                  </div>
                </div>

                {/* Agent Stats */}
                <div className="grid grid-cols-2 gap-3 sm:gap-4 mb-3 sm:mb-4">
                  <div>
                    <p className="text-xs text-gray-500 mb-0.5 sm:mb-1">Digital Balance</p>
                    <p className="font-mono font-semibold text-gray-900 text-xs sm:text-sm break-words">
                      {formatBalance(agent.digitalBalance || 0)} {selectedCurrency || 'UGX'}
                    </p>
                  </div>
                  <div>
                    <p className="text-xs text-gray-500 mb-0.5 sm:mb-1">Cash Available</p>
                    <p className="font-mono font-semibold text-gray-900 text-xs sm:text-sm break-words">
                      {formatBalance(agent.cashBalance || 0)} {selectedCurrency || 'UGX'}
                    </p>
                  </div>
                </div>

                {/* Rating (Mock) */}
                <div className="flex items-center mb-3 sm:mb-4">
                  <div className="flex items-center">
                    {[1, 2, 3, 4, 5].map((star) => (
                      <Star
                        key={star}
                        className={`h-3.5 w-3.5 sm:h-4 sm:w-4 flex-shrink-0 ${
                          star <= 4 ? 'text-yellow-400 fill-current' : 'text-gray-300'
                        }`}
                      />
                    ))}
                  </div>
                  <span className="ml-2 text-xs sm:text-sm text-gray-600">4.0 (23 reviews)</span>
                </div>

                {/* Services */}
                <div className="flex flex-wrap gap-1.5 sm:gap-2 mb-3 sm:mb-4">
                  <span className="px-1.5 sm:px-2 py-0.5 sm:py-1 bg-blue-100 text-blue-800 text-xs rounded-full">
                    Cash Deposit
                  </span>
                  <span className="px-1.5 sm:px-2 py-0.5 sm:py-1 bg-green-100 text-green-800 text-xs rounded-full">
                    Withdrawal
                  </span>
                  <span className="px-1.5 sm:px-2 py-0.5 sm:py-1 bg-orange-100 text-orange-800 text-xs rounded-full">
                    Bitcoin Exchange
                  </span>
                </div>

                {/* Action Buttons */}
                <div className="flex gap-2">
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      console.log('Select Agent button clicked', agent);
                      onCreateDepositRequest(agent);
                    }}
                    disabled={isCreating}
                    className="flex-1 bg-gray-900 text-white px-3 sm:px-4 py-1.5 sm:py-2 rounded-lg text-xs sm:text-sm font-medium hover:bg-gray-800 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors duration-200"
                  >
                    {isCreating ? 'Creating...' : 'Select Agent'}
                  </button>
                  <button className="px-2.5 sm:px-3 md:px-4 py-1.5 sm:py-2 border border-gray-300 text-gray-700 rounded-lg text-xs sm:text-sm font-medium hover:bg-gray-50 transition-colors duration-200">
                    <Phone className="h-3 w-3 sm:h-3.5 sm:w-3.5 md:h-4 md:w-4 flex-shrink-0" />
                  </button>
                </div>

                {/* Expanded Details */}
                {selectedAgent?.id === agent.id && (
                  <div className="mt-3 sm:mt-4 pt-3 sm:pt-4 border-t border-gray-200">
                    <div className="space-y-2 sm:space-y-3">
                      <div>
                        <p className="text-xs text-gray-500 mb-1">Operating Hours</p>
                        <div className="flex items-center text-xs sm:text-sm text-gray-700">
                          <Clock className="h-3 w-3 sm:h-3.5 sm:w-3.5 md:h-4 md:w-4 mr-1 sm:mr-1.5 md:mr-2 flex-shrink-0" />
                          Mon-Sat: 8:00 AM - 8:00 PM
                        </div>
                      </div>
                      <div>
                        <p className="text-xs text-gray-500 mb-1">Services Available</p>
                        <ul className="text-xs sm:text-sm text-gray-700 space-y-1">
                          <li>• Cash deposits and withdrawals</li>
                          <li>• Bitcoin buying and selling</li>
                          <li>• Money transfers</li>
                          <li>• Account verification</li>
                        </ul>
                      </div>
                      <div>
                        <p className="text-xs text-gray-500 mb-1">Available Balance</p>
                        <p className="text-xs sm:text-sm text-gray-700 break-words">
                          Digital: {agent.digitalBalance?.toLocaleString() || 'N/A'} {selectedCurrency || 'UGX'}
                        </p>
                        <p className="text-xs sm:text-sm text-gray-700 break-words">
                          Cash: {agent.cashBalance?.toLocaleString() || 'N/A'} {selectedCurrency || 'UGX'}
                        </p>
                      </div>
                    </div>
                  </div>
                )}
              </div>
            ))}
          </div>
        ) : (
          <div className="h-[250px] sm:h-[300px] md:h-[400px] lg:h-[500px] xl:h-[600px] w-full relative z-0">
            <MapContainer
              center={userLocation || [0.3476, 32.5825]} // Default to Kampala if no user location
              zoom={13}
              style={{ 
                height: '100%', 
                width: '100%', 
                minHeight: '300px', 
                position: 'relative', 
                zIndex: 1,
                borderRadius: '0.5rem'
              }}
              className="leaflet-map-container"
            >
              <TileLayer
                attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
                url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
              />
              
              {userLocation && <CenterMap center={userLocation} />}
              
              {userLocation && (
                <Marker 
                  position={userLocation}
                  icon={createBlinkingUserIcon()}
                />
              )}
              
              {uniqueAgents.map((agent) => (
                <Marker
                  key={agent.id}
                  position={[agent.location.coordinates.lat, agent.location.coordinates.lng]}
                  icon={createAgentIcon(agent.status === 'available')}
                  eventHandlers={{
                    click: (event) => {
                      handleMarkerClick(agent, event);
                    },
                  }}
                />
              ))}
            </MapContainer>
          </div>
        )}

        {isCreating && (
          <div className="text-center mt-3 sm:mt-4 md:mt-5 lg:mt-6">
            <div className="inline-flex items-center space-x-1.5 sm:space-x-2 text-gray-600 text-[10px] sm:text-xs md:text-sm">
              <div className="animate-spin rounded-full h-3.5 w-3.5 sm:h-4 sm:w-4 border-b-2 border-gray-900"></div>
              <span>Creating deposit request...</span>
            </div>
          </div>
        )}
      </div>

      {/* Custom popup that renders outside map to avoid z-index issues */}
      {popupAgent && (
        <>
          {/* Backdrop to close popup */}
          <div
            className="fixed inset-0 z-[9998]"
            onClick={closePopup}
            style={{ backgroundColor: 'transparent' }}
          />
          
          {/* Popup content */}
          <div
            className="fixed z-[9999] bg-white rounded-lg shadow-2xl border border-gray-200"
            style={{
              left: `${popupPosition.x}px`,
              top: `${popupPosition.y}px`,
              transform: 'translate(-50%, -100%)',
              marginTop: '-10px',
              maxWidth: '260px',
              minWidth: '220px',
            }}
          >
            {/* Arrow pointing down */}
            <div 
              className="absolute -bottom-2 left-1/2 transform -translate-x-1/2"
              style={{
                width: 0,
                height: 0,
                borderLeft: '8px solid transparent',
                borderRight: '8px solid transparent',
                borderTop: '8px solid white',
              }}
            />
            
            {/* Close button */}
            <button
              onClick={closePopup}
              className="absolute top-1.5 sm:top-2 md:top-3 right-1.5 sm:right-2 md:right-3 z-10 p-0.5 sm:p-1 md:p-1.5 rounded-full hover:bg-gray-100 transition-colors"
            >
              <X size={12} className="sm:w-3.5 sm:h-3.5 md:w-4 md:h-4" />
            </button>

            {/* Agent details content */}
            <div className="p-2.5 sm:p-3 md:p-4">
              {renderAgentDetails(popupAgent)}
            </div>
          </div>
        </>
      )}
    </div>
  );
};

export default AgentSelectionStep;