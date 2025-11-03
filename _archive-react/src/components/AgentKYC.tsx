import React, { useState } from 'react';
import { Upload, CheckCircle, Building, ArrowRight, ArrowLeft } from 'lucide-react';
import { AgentKYCData, LocationSuggestion } from '../types/auth';
import LocationSearch from './LocationSearch';

interface AgentKYCProps {
  onSubmit: (data: AgentKYCData) => void;
  isLoading?: boolean;
}

type Step = 1 | 2;

const AgentKYC: React.FC<AgentKYCProps> = ({ onSubmit, isLoading = false }) => {
  const [currentStep, setCurrentStep] = useState<Step>(1);
  const [formData, setFormData] = useState<AgentKYCData>({
    // Personal Details
    firstName: '',
    lastName: '',
    phoneNumber: '',
    
    // Operating Details
    location: {
      country: '',
      state: '',
      city: '',
      address: ''
    },
    locationDescription: '',
    operatingHours: '',
    operatingDays: '',
    
    // Optional fields
    documentType: 'national_id',
    documentNumber: '',
    businessLicense: ''
  });
  const [selectedLocation, setSelectedLocation] = useState<LocationSuggestion | null>(null);
  const [errors, setErrors] = useState<Record<string, string>>({});

  const documentTypes = [
    { value: 'national_id', label: 'National ID Card' },
    { value: 'passport', label: 'International Passport' },
    { value: 'drivers_license', label: 'Driver\'s License' }
  ];

  const operatingDaysOptions = [
    'Monday to Friday',
    'Monday to Saturday', 
    'Monday to Sunday',
    'Tuesday to Saturday',
    'Custom Schedule'
  ];

  const operatingHoursOptions = [
    '8:00 AM - 5:00 PM',
    '9:00 AM - 6:00 PM',
    '8:00 AM - 8:00 PM',
    '24 Hours',
    'Custom Hours'
  ];

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
    if (errors[name]) {
      setErrors(prev => ({ ...prev, [name]: '' }));
    }
  };

  const handleAddressChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { value } = e.target;
    setFormData(prev => ({
      ...prev,
      location: { ...prev.location, address: value }
    }));
    if (errors.address) {
      setErrors(prev => ({ ...prev, address: '' }));
    }
  };

  const handleDocumentFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      setFormData(prev => ({ ...prev, documentFile: file }));
      if (errors.documentFile) {
        setErrors(prev => ({ ...prev, documentFile: '' }));
      }
    }
  };

  const handleBusinessLicenseFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      setFormData(prev => ({ ...prev, businessLicenseFile: file }));
      if (errors.businessLicenseFile) {
        setErrors(prev => ({ ...prev, businessLicenseFile: '' }));
      }
    }
  };

  const handleLocationSelect = (location: LocationSuggestion | null) => {
    setSelectedLocation(location);
    if (location) {
      setFormData(prev => ({
        ...prev,
        location: {
          ...prev.location,
          country: location.location.country || '',
          state: location.location.state || '',
          city: location.location.city || '',
          address: location.location.address || '',
          coordinates: {
            lat: location.location.coordinates.lat,
            lng: location.location.coordinates.lng
          }
        }
      }));
      if (errors.location) {
        setErrors(prev => ({ ...prev, location: '' }));
      }
    }
  };

  const validateStep1 = (): boolean => {
    const newErrors: Record<string, string> = {};

    // Personal Details validation
    if (!formData.firstName.trim()) {
      newErrors.firstName = 'First name is required';
    }

    if (!formData.lastName.trim()) {
      newErrors.lastName = 'Last name is required';
    }

    if (!formData.phoneNumber.trim()) {
      newErrors.phoneNumber = 'Phone number is required';
    } else if (!/^\+\d{1,4}\d{6,14}$/.test(formData.phoneNumber)) {
      newErrors.phoneNumber = 'Please enter a valid international phone number (e.g., +1234567890)';
    }

    // Operating Details validation
    if (!selectedLocation) {
      newErrors.location = 'Please select your location';
    }

    if (!formData.location.address.trim()) {
      newErrors.address = 'Street address is required';
    }

    if (!formData.locationDescription.trim()) {
      newErrors.locationDescription = 'Location description is required';
    }

    if (!formData.operatingHours) {
      newErrors.operatingHours = 'Operating hours are required';
    }

    if (!formData.operatingDays) {
      newErrors.operatingDays = 'Operating days are required';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const validateStep2 = (): boolean => {
    // Step 2 fields are optional, so no validation needed
    return true;
  };

  const handleNextStep = () => {
    if (currentStep === 1 && validateStep1()) {
      setCurrentStep(2);
    }
  };

  const handlePreviousStep = () => {
    if (currentStep === 2) {
      setCurrentStep(1);
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (currentStep === 1) {
      handleNextStep();
    } else if (currentStep === 2 && validateStep2()) {
      onSubmit(formData);
    }
  };

  const renderStep1 = () => (
    <div className="space-y-4 sm:space-y-6">
      {/* Personal Details Section */}
      <div className="border-b border-gray-200 pb-4 sm:pb-6">
        <h3 className="text-base sm:text-lg font-medium text-gray-900 mb-3 sm:mb-4">Personal Details</h3>
        
        <div className="grid grid-cols-1 md:grid-cols-2 gap-3 sm:gap-4">
          {/* First Name */}
          <div>
            <label htmlFor="firstName" className="block text-sm font-medium text-gray-700 mb-1 sm:mb-2">
              First Name
            </label>
            <input
              type="text"
              id="firstName"
              name="firstName"
              value={formData.firstName}
              onChange={handleInputChange}
              placeholder="Enter your first name"
              className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-neutral-500 text-sm sm:text-base ${
                errors.firstName ? 'border-red-500' : 'border-gray-300'
              }`}
            />
            {errors.firstName && (
              <p className="mt-1 text-xs sm:text-sm text-red-600">{errors.firstName}</p>
            )}
          </div>

          {/* Last Name */}
          <div>
            <label htmlFor="lastName" className="block text-sm font-medium text-gray-700 mb-1 sm:mb-2">
              Last Name
            </label>
            <input
              type="text"
              id="lastName"
              name="lastName"
              value={formData.lastName}
              onChange={handleInputChange}
              placeholder="Enter your last name"
              className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-neutral-500 text-sm sm:text-base ${
                errors.lastName ? 'border-red-500' : 'border-gray-300'
              }`}
            />
            {errors.lastName && (
              <p className="mt-1 text-xs sm:text-sm text-red-600">{errors.lastName}</p>
            )}
          </div>
        </div>

        {/* Phone Number */}
        <div className="mt-3 sm:mt-4">
          <label htmlFor="phoneNumber" className="block text-sm font-medium text-gray-700 mb-1 sm:mb-2">
            Phone Number
          </label>
          <input
            type="tel"
            id="phoneNumber"
            name="phoneNumber"
            value={formData.phoneNumber}
            onChange={handleInputChange}
            placeholder="+256701234567 or 0701234567"
            className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-neutral-500 text-sm sm:text-base ${
              errors.phoneNumber ? 'border-red-500' : 'border-gray-300'
            }`}
          />
          {errors.phoneNumber && (
            <p className="mt-1 text-xs sm:text-sm text-red-600">{errors.phoneNumber}</p>
          )}
        </div>
      </div>

      {/* Operating Details Section */}
      <div className="space-y-3 sm:space-y-4">
        <h3 className="text-base sm:text-lg font-medium text-gray-900">Operating Details</h3>
        
        {/* Location Search */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1 sm:mb-2">
            City/Town
          </label>
          <LocationSearch
            value={selectedLocation}
            onChange={handleLocationSelect}
            placeholder="Search for your city or town..."
            required
          />
          {errors.location && (
            <p className="mt-1 text-xs sm:text-sm text-red-600">{errors.location}</p>
          )}
        </div>

        {/* Street Address */}
        <div>
          <label htmlFor="address" className="block text-sm font-medium text-gray-700 mb-1 sm:mb-2">
            Street Address
          </label>
          <input
            type="text"
            id="address"
            name="address"
            value={formData.location.address}
            onChange={handleAddressChange}
            placeholder="Enter your street address"
            className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-neutral-500 text-sm sm:text-base ${
              errors.address ? 'border-red-500' : 'border-gray-300'
            }`}
          />
          {errors.address && (
            <p className="mt-1 text-xs sm:text-sm text-red-600">{errors.address}</p>
          )}
        </div>

        {/* Operating Hours and Days */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-3 sm:gap-4">
          <div>
            <label htmlFor="operatingDays" className="block text-sm font-medium text-gray-700 mb-1 sm:mb-2">
              Operating Days
            </label>
            <select
              id="operatingDays"
              name="operatingDays"
              value={formData.operatingDays}
              onChange={handleInputChange}
              className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-neutral-500 text-sm sm:text-base ${
                errors.operatingDays ? 'border-red-500' : 'border-gray-300'
              }`}
            >
              <option value="">Select operating days</option>
              {operatingDaysOptions.map(option => (
                <option key={option} value={option}>{option}</option>
              ))}
            </select>
            {errors.operatingDays && (
              <p className="mt-1 text-xs sm:text-sm text-red-600">{errors.operatingDays}</p>
            )}
          </div>

          <div>
            <label htmlFor="operatingHours" className="block text-sm font-medium text-gray-700 mb-1 sm:mb-2">
              Operating Hours
            </label>
            <select
              id="operatingHours"
              name="operatingHours"
              value={formData.operatingHours}
              onChange={handleInputChange}
              className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-neutral-500 text-sm sm:text-base ${
                errors.operatingHours ? 'border-red-500' : 'border-gray-300'
              }`}
            >
              <option value="">Select operating hours</option>
              {operatingHoursOptions.map(option => (
                <option key={option} value={option}>{option}</option>
              ))}
            </select>
            {errors.operatingHours && (
              <p className="mt-1 text-xs sm:text-sm text-red-600">{errors.operatingHours}</p>
            )}
          </div>
        </div>

        {/* Location Description */}
        <div>
          <label htmlFor="locationDescription" className="block text-sm font-medium text-gray-700 mb-1 sm:mb-2">
            Location Description
          </label>
          <textarea
            id="locationDescription"
            name="locationDescription"
            value={formData.locationDescription}
            onChange={handleInputChange}
            rows={3}
            placeholder="Describe your location (e.g., 'Near the market', 'Next to the bank', etc.)"
            className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-neutral-500 text-sm sm:text-base ${
              errors.locationDescription ? 'border-red-500' : 'border-gray-300'
            }`}
          />
          {errors.locationDescription && (
            <p className="mt-1 text-xs sm:text-sm text-red-600">{errors.locationDescription}</p>
          )}
        </div>
      </div>
    </div>
  );

  const renderStep2 = () => (
    <div className="space-y-6">
      {/* Personal Identification Section */}
      <div className="border-b border-gray-200 pb-6">
        <h3 className="text-lg font-medium text-gray-900 mb-2">Personal Identification</h3>
        <p className="text-sm text-gray-600 mb-4">(Optional - You can complete verification later)</p>
        
        {/* Document Type */}
        <div className="mb-4">
          <label htmlFor="documentType" className="block text-sm font-medium text-gray-700 mb-2">
            Document Type (Optional)
          </label>
          <select
            id="documentType"
            name="documentType"
            value={formData.documentType || 'national_id'}
            onChange={handleInputChange}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-neutral-500"
          >
            {documentTypes.map(type => (
              <option key={type.value} value={type.value}>{type.label}</option>
            ))}
          </select>
        </div>

        {/* Document Number */}
        <div className="mb-3 sm:mb-4">
          <label htmlFor="documentNumber" className="block text-sm font-medium text-gray-700 mb-1 sm:mb-2">
            Document Number (Optional)
          </label>
          <input
            type="text"
            id="documentNumber"
            name="documentNumber"
            value={formData.documentNumber || ''}
            onChange={handleInputChange}
            placeholder="Enter your document number (optional)"
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-neutral-500 text-sm sm:text-base"
          />
        </div>

        {/* Document Upload */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1 sm:mb-2">
            Document Image (Optional)
          </label>
          <div className="border-2 border-dashed border-gray-300 rounded-md p-3 sm:p-4 text-center">
            <input
              type="file"
              id="documentFile"
              accept="image/*"
              onChange={handleDocumentFileChange}
              className="hidden"
            />
            <label htmlFor="documentFile" className="cursor-pointer">
              <Upload className="w-6 h-6 sm:w-8 sm:h-8 text-gray-400 mx-auto mb-2" />
              <p className="text-xs sm:text-sm text-gray-600">
                {formData.documentFile ? formData.documentFile.name : 'Click to upload document image (optional)'}
              </p>
              <p className="text-xs text-gray-500 mt-1">PNG, JPG up to 10MB</p>
            </label>
          </div>
        </div>
      </div>

      {/* Business License Section */}
      <div className="space-y-3 sm:space-y-4">
        <h3 className="text-base sm:text-lg font-medium text-gray-900">Business License</h3>
        <p className="text-xs sm:text-sm text-gray-600">(Optional - You can complete verification later)</p>
        
        {/* Business License Number */}
        <div>
          <label htmlFor="businessLicense" className="block text-sm font-medium text-gray-700 mb-1 sm:mb-2">
            Business License Number (Optional)
          </label>
          <input
            type="text"
            id="businessLicense"
            name="businessLicense"
            value={formData.businessLicense || ''}
            onChange={handleInputChange}
            placeholder="Enter business license number (optional)"
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-neutral-500 text-sm sm:text-base"
          />
        </div>

        {/* Business License Upload */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1 sm:mb-2">
            Business License Document (Optional)
          </label>
          <div className="border-2 border-dashed border-gray-300 rounded-md p-3 sm:p-4 text-center">
            <input
              type="file"
              id="businessLicenseFile"
              accept="image/*,application/pdf"
              onChange={handleBusinessLicenseFileChange}
              className="hidden"
            />
            <label htmlFor="businessLicenseFile" className="cursor-pointer">
              <Upload className="w-6 h-6 sm:w-8 sm:h-8 text-gray-400 mx-auto mb-2" />
              <p className="text-xs sm:text-sm text-gray-600">
                {formData.businessLicenseFile ? formData.businessLicenseFile.name : 'Click to upload business license (optional)'}
              </p>
              <p className="text-xs text-gray-500 mt-1">PNG, JPG, PDF up to 10MB</p>
            </label>
          </div>
        </div>
      </div>
    </div>
  );

  return (
    <div className="w-full max-w-sm sm:max-w-lg lg:max-w-2xl mx-auto bg-white rounded-lg shadow-md p-4 sm:p-6">
      <div className="text-center mb-4 sm:mb-6">
        <div className="w-12 h-12 sm:w-16 sm:h-16 bg-neutral-100 rounded-full flex items-center justify-center mx-auto mb-3 sm:mb-4">
          <Building className="w-6 h-6 sm:w-8 sm:h-8 text-neutral-600" />
        </div>
        <h2 className="text-xl sm:text-2xl font-bold text-gray-900">Agent Verification</h2>
        <p className="text-sm sm:text-base text-gray-600 mt-1 sm:mt-2">
          Complete your agent verification to start serving customers
        </p>
      </div>

      {/* Step Indicator */}
      <div className="flex items-center justify-center mb-6 sm:mb-8">
        <div className="flex items-center space-x-3 sm:space-x-4">
          <div className={`flex items-center justify-center w-7 h-7 sm:w-8 sm:h-8 rounded-full text-xs sm:text-sm font-medium ${
            currentStep >= 1 ? 'bg-neutral-900 text-white' : 'bg-gray-300 text-gray-600'
          }`}>
            1
          </div>
          <div className="w-12 sm:w-16 h-0.5 bg-gray-300"></div>
          <div className={`flex items-center justify-center w-7 h-7 sm:w-8 sm:h-8 rounded-full text-xs sm:text-sm font-medium ${
            currentStep >= 2 ? 'bg-neutral-900 text-white' : 'bg-gray-300 text-gray-600'
          }`}>
            2
          </div>
        </div>
      </div>

      {/* Step Titles */}
      <div className="text-center mb-4 sm:mb-6">
        <h3 className="text-base sm:text-lg font-semibold text-gray-900">
          {currentStep === 1 ? 'Personal & Operating Details' : 'Identification & Business License'}
        </h3>
        <p className="text-xs sm:text-sm text-gray-600 mt-1">
          {currentStep === 1 
            ? 'Provide your personal information and operating details' 
            : 'Upload identification and business documents (optional)'
          }
        </p>
      </div>

      <form onSubmit={handleSubmit} className="space-y-4 sm:space-y-6">
        {currentStep === 1 ? renderStep1() : renderStep2()}

        {/* Navigation Buttons */}
        <div className="flex justify-between pt-4 sm:pt-6">
          {currentStep === 2 && (
            <button
              type="button"
              onClick={handlePreviousStep}
              className="flex items-center px-3 py-2 sm:px-4 sm:py-2 text-neutral-700 bg-neutral-100 rounded-md hover:bg-neutral-200 focus:outline-none focus:ring-2 focus:ring-neutral-500 text-sm sm:text-base"
            >
              <ArrowLeft className="w-4 h-4 mr-1 sm:mr-2" />
              Previous
            </button>
          )}
          
          <button
            type="submit"
            disabled={isLoading}
            className={`flex items-center justify-center px-4 py-2 sm:px-6 sm:py-2 bg-neutral-900 text-white rounded-md hover:bg-neutral-800 focus:outline-none focus:ring-2 focus:ring-neutral-500 disabled:opacity-50 disabled:cursor-not-allowed text-sm sm:text-base ${
              currentStep === 1 ? 'ml-auto' : ''
            }`}
          >
            {isLoading ? (
              <>
                <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
                Submitting...
              </>
            ) : currentStep === 1 ? (
              <>
                Next
                <ArrowRight className="w-4 h-4 ml-1 sm:ml-2" />
              </>
            ) : (
              <>
                <CheckCircle className="w-4 h-4 mr-1 sm:mr-2" />
                Complete Agent Verification
              </>
            )}
          </button>
        </div>
      </form>

      <div className="mt-4 sm:mt-6 text-center">
        <p className="text-xs text-gray-500">
          Your information is encrypted and secure. We verify all agents to ensure customer safety.
        </p>
      </div>
    </div>
  );
};

export default AgentKYC;
