import React, { useState } from 'react';
import { Upload, Shield, CheckCircle } from 'lucide-react';
import { UserKYCData } from '../types/auth';

interface UserKYCProps {
  onSubmit: (data: UserKYCData) => void;
  isLoading?: boolean;
}

const UserKYC: React.FC<UserKYCProps> = ({ onSubmit, isLoading = false }) => {
  const [formData, setFormData] = useState<UserKYCData>({
    firstName: '',
    lastName: '',
    phoneNumber: '',
    documentType: 'national_id',
    documentNumber: '',
  });
  const [errors, setErrors] = useState<Record<string, string>>({});

  const documentTypes = [
    { value: "national_id", label: "National ID Card" },
    { value: "passport", label: "International Passport"},
    { value: "drivers_license", label: "Driver's License"}
  ];

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
    if (errors[name]) {
      setErrors(prev => ({ ...prev, [name]: '' }));
    }
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      setFormData(prev => ({ ...prev, documentFile: file }));
      if (errors.documentFile) {
        setErrors(prev => ({ ...prev, documentFile: '' }));
      }
    }
  };

  const validateForm = (): boolean => {
    const newErrors: Record<string, string> = {};

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

    // Document fields are now optional - no validation required

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (validateForm()) {
      onSubmit(formData);
    }
  };

  return (
    <div className="w-full max-w-md mx-auto bg-white rounded-lg shadow-md p-4 sm:p-6">
      <div className="text-center mb-4 sm:mb-6">
        <div className="w-12 h-12 sm:w-16 sm:h-16 bg-blue-100 rounded-full flex items-center justify-center mx-auto mb-3 sm:mb-4">
          <Shield className="w-6 h-6 sm:w-8 sm:h-8 text-blue-600" />
        </div>
        <h2 className="text-xl sm:text-2xl font-bold text-gray-900">Complete Your Verification</h2>
        <p className="text-sm sm:text-base text-gray-600 mt-2">
          Verify your identity to start using AfriTokeni securely
        </p>
      </div>

      <form onSubmit={handleSubmit} className="space-y-4 sm:space-y-6">
        {/* Personal Information */}
        <div className="space-y-3 sm:space-y-4">
          <h3 className="text-base sm:text-lg font-medium text-gray-900">Personal Information</h3>
          
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
              className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm sm:text-base ${
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
              className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm sm:text-base ${
                errors.lastName ? 'border-red-500' : 'border-gray-300'
              }`}
            />
            {errors.lastName && (
              <p className="mt-1 text-xs sm:text-sm text-red-600">{errors.lastName}</p>
            )}
          </div>

          {/* Phone Number */}
          <div>
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
              className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm sm:text-base ${
                errors.phoneNumber ? 'border-red-500' : 'border-gray-300'
              }`}
            />
            {errors.phoneNumber && (
              <p className="mt-1 text-xs sm:text-sm text-red-600">{errors.phoneNumber}</p>
            )}
          </div>
        </div>

        {/* Document Type */}
        <div>
          <label htmlFor="documentType" className="block text-sm font-medium text-gray-700 mb-1 sm:mb-2">
            Document Type (Optional)
          </label>
          <select
            id="documentType"
            name="documentType"
            value={formData.documentType}
            onChange={handleInputChange}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm sm:text-base"
          >
            {documentTypes.map(type => (
              <option key={type.value} value={type.value}>{type.label}</option>
            ))}
          </select>
        </div>

        {/* Document Number */}
        <div>
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
            className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm sm:text-base ${
              errors.documentNumber ? 'border-red-500' : 'border-gray-300'
            }`}
          />
          {errors.documentNumber && (
            <p className="mt-1 text-xs sm:text-sm text-red-600">{errors.documentNumber}</p>
          )}
        </div>

        {/* Document Upload */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1 sm:mb-2">
            Document Image (Optional)
          </label>
          <div className={`border-2 border-dashed rounded-md p-3 sm:p-4 text-center ${
            errors.documentFile ? 'border-red-500' : 'border-gray-300'
          }`}>
            <input
              type="file"
              id="documentFile"
              accept="image/*"
              onChange={handleFileChange}
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
          {errors.documentFile && (
            <p className="mt-1 text-xs sm:text-sm text-red-600">{errors.documentFile}</p>
          )}
        </div>

        {/* Submit Button */}
        <button
          type="submit"
          disabled={isLoading}
          className="w-full flex items-center justify-center px-4 py-3 bg-neutral-900 text-white rounded-lg font-semibold hover:bg-neutral-800 focus:outline-none focus:ring-2 focus:ring-neutral-900 disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200 text-sm sm:text-base"
        >
          {isLoading ? (
            <>
              <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
              Submitting...
            </>
          ) : (
            <>
              <CheckCircle className="w-4 h-4 mr-2" />
              Complete Verification
            </>
          )}
        </button>
      </form>

      <div className="mt-4 sm:mt-6 text-center">
        <p className="text-xs text-gray-500">
          Your information is encrypted and secure. We use bank-level security to protect your data.
        </p>
      </div>
    </div>
  );
};

export default UserKYC;
