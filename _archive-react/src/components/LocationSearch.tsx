import React, { useState, useRef, useEffect } from 'react';
import { Search, MapPin, X } from 'lucide-react';
import { LocationSuggestion } from '../types/auth';

interface LocationSearchProps {
  value?: LocationSuggestion | null;
  onChange: (location: LocationSuggestion | null) => void;
  placeholder?: string;
  required?: boolean;
}

const LocationSearch: React.FC<LocationSearchProps> = ({
  value,
  onChange,
  placeholder = "Search for your location...",
  required = false
}) => {
  const [query, setQuery] = useState(value?.display_name || '');
  const [suggestions, setSuggestions] = useState<LocationSuggestion[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [isOpen, setIsOpen] = useState(false);
  const searchTimeout = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  const containerRef = useRef<HTMLDivElement>(null);

  // Location data based on existing agents in agents.json
  const mockLocations: LocationSuggestion[] = [
    {
      place_id: "1",
      display_name: "Kampala Central, Central Division, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Kampala Road, Central Division, Kampala",
        coordinates: {
          lat: 0.3476,
          lng: 32.5825
        }
      }
    },
    {
      place_id: "2",
      display_name: "Rubaga, Mengo, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Buganda Road, Mengo, Kampala",
        coordinates: {
          lat: 0.3136,
          lng: 32.5811
        }
      }
    },
    {
      place_id: "3",
      display_name: "Lugogo, Nakawa Division, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Jinja Road, Nakawa Division, Kampala",
        coordinates: {
          lat: 0.3563,
          lng: 32.6378
        }
      }
    },
    {
      place_id: "4",
      display_name: "Quality Mall Area, Wakiso, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Entebbe Road, Wakiso, Kampala",
        coordinates: {
          lat: 0.2906,
          lng: 32.5739
        }
      }
    },
    {
      place_id: "5",
      display_name: "Mulago, Kawempe Division, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Bombo Road, Kawempe Division, Kampala",
        coordinates: {
          lat: 0.3319,
          lng: 32.5729
        }
      }
    },
    {
      place_id: "6",
      display_name: "Ggaba, Makindye Division, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Masaka Road, Makindye Division, Kampala",
        coordinates: {
          lat: 0.3157,
          lng: 32.5656
        }
      }
    },
    {
      place_id: "7",
      display_name: "Makerere University Area, Kasangati, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Gayaza Road, Kasangati, Kampala",
        coordinates: {
          lat: 0.3341,
          lng: 32.6189
        }
      }
    },
    {
      place_id: "8",
      display_name: "Kabalagala, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Ggaba Road, Kabalagala, Kampala",
        coordinates: {
          lat: 0.2742,
          lng: 32.6014
        }
      }
    },
    {
      place_id: "9",
      display_name: "Makerere Hill, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Hoima Road, Makerere, Kampala",
        coordinates: {
          lat: 0.3298,
          lng: 32.5456
        }
      }
    },
    {
      place_id: "10",
      display_name: "Kamwokya, Central Division, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Kamwokya, Central Division, Kampala",
        coordinates: {
          lat: 0.3789,
          lng: 32.6156
        }
      }
    },
    {
      place_id: "11",
      display_name: "Muyenga, Makindye Division, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Muyenga, Makindye Division, Kampala",
        coordinates: {
          lat: 0.3023,
          lng: 32.5698
        }
      }
    },
    {
      place_id: "12",
      display_name: "Wandegeya, Central Division, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Wandegeya, Central Division, Kampala",
        coordinates: {
          lat: 0.3645,
          lng: 32.5923
        }
      }
    },
    {
      place_id: "13",
      display_name: "Ntinda, Nakawa Division, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Ntinda, Nakawa Division, Kampala",
        coordinates: {
          lat: 0.2987,
          lng: 32.6234
        }
      }
    },
    {
      place_id: "14",
      display_name: "Old Kampala, Central Division, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Old Kampala, Central Division, Kampala",
        coordinates: {
          lat: 0.3512,
          lng: 32.5634
        }
      }
    },
    {
      place_id: "15",
      display_name: "Kyanja, Nakawa Division, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Kyanja, Nakawa Division, Kampala",
        coordinates: {
          lat: 0.3167,
          lng: 32.6445
        }
      }
    },
    {
      place_id: "16",
      display_name: "Kansanga, Makindye Division, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Kansanga, Makindye Division, Kampala",
        coordinates: {
          lat: 0.2834,
          lng: 32.5512
        }
      }
    },
    {
      place_id: "17",
      display_name: "Bwaise, Kawempe Division, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Bwaise, Kawempe Division, Kampala",
        coordinates: {
          lat: 0.3423,
          lng: 32.5512
        }
      }
    },
    {
      place_id: "18",
      display_name: "Kisaasi, Nakawa Division, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Kisaasi, Nakawa Division, Kampala",
        coordinates: {
          lat: 0.3689,
          lng: 32.6523
        }
      }
    },
    {
      place_id: "19",
      display_name: "Nsambya, Makindye Division, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Nsambya, Makindye Division, Kampala",
        coordinates: {
          lat: 0.2945,
          lng: 32.5823
        }
      }
    },
    {
      place_id: "20",
      display_name: "Bukoto, Central Division, Kampala",
      location: {
        country: "Uganda",
        state: "Central",
        city: "Kampala",
        address: "Bukoto, Central Division, Kampala",
        coordinates: {
          lat: 0.3834,
          lng: 32.5834
        }
      }
    }
  ];

  const searchLocations = (searchQuery: string) => {
    setIsLoading(true);
    
    // Simulate API delay
    setTimeout(() => {
      if (searchQuery.trim()) {
        const filtered = mockLocations.filter(location =>
          location.display_name.toLowerCase().includes(searchQuery.toLowerCase()) ||
          location.location.city?.toLowerCase().includes(searchQuery.toLowerCase()) ||
          location.location.state?.toLowerCase().includes(searchQuery.toLowerCase())
        );
        setSuggestions(filtered);
      } else {
        setSuggestions(mockLocations);
      }
      setIsLoading(false);
    }, 300);
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newQuery = e.target.value;
    setQuery(newQuery);
    setIsOpen(true);

    if (searchTimeout.current) {
      clearTimeout(searchTimeout.current);
    }

    searchTimeout.current = setTimeout(() => {
      searchLocations(newQuery);
    }, 300);
  };

  const handleSuggestionClick = (suggestion: LocationSuggestion) => {
    setQuery(suggestion.display_name);
    onChange(suggestion);
    setIsOpen(false);
  };

  const handleClear = () => {
    setQuery('');
    onChange(null);
    setIsOpen(false);
  };

  const handleFocus = () => {
    setIsOpen(true);
    if (suggestions.length === 0) {
      searchLocations(query);
    }
  };

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (containerRef.current && !containerRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  return (
    <div className="relative" ref={containerRef}>
      <div className="relative">
        <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
          <Search className="h-5 w-5 text-neutral-400" />
        </div>
        <input
          type="text"
          value={query}
          onChange={handleInputChange}
          onFocus={handleFocus}
          placeholder={placeholder}
          required={required}
          className="w-full pl-10 pr-10 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-neutral-500 focus:border-neutral-500 bg-white placeholder-neutral-500"
        />
        {query && (
          <div className="absolute inset-y-0 right-0 pr-3 flex items-center">
            <button
              type="button"
              onClick={handleClear}
              className="text-neutral-400 hover:text-neutral-600"
            >
              <X className="h-5 w-5" />
            </button>
          </div>
        )}
      </div>

      {isOpen && (
        <div className="absolute z-10 mt-1 w-full bg-white shadow-lg max-h-60 rounded-md py-1 text-base ring-1 ring-black ring-opacity-5 overflow-auto">
          {isLoading ? (
            <div className="px-4 py-2 text-sm text-neutral-500">
              Searching locations...
            </div>
          ) : suggestions.length > 0 ? (
            suggestions.map((suggestion) => (
              <button
                key={suggestion.place_id}
                type="button"
                onClick={() => handleSuggestionClick(suggestion)}
                className="w-full text-left px-4 py-2 text-sm text-neutral-700 hover:bg-neutral-50 focus:outline-none focus:bg-neutral-50 flex items-center"
              >
                <MapPin className="h-4 w-4 text-neutral-400 mr-3 flex-shrink-0" />
                <div className="truncate">
                  <div className="font-medium">{suggestion.display_name}</div>
                  <div className="text-xs text-neutral-500 truncate">
                    {suggestion.location.state}, {suggestion.location.country}
                  </div>
                </div>
              </button>
            ))
          ) : query.trim() ? (
            <div className="px-4 py-2 text-sm text-neutral-500">
              No locations found for &quot;{query}&quot;
            </div>
          ) : (
            <div className="px-4 py-2 text-sm text-neutral-500">
              Start typing to search for locations
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default LocationSearch;
