import React from 'react';
import { ProgressIndicatorProps } from '../../types/depositTypes';

const ProgressIndicator: React.FC<ProgressIndicatorProps> = ({ currentStep, steps }) => {
  const getCurrentStepIndex = () => {
    return steps.findIndex(step => step.key === currentStep);
  };

  const getStepClassName = (stepIndex: number) => {
    const currentIndex = getCurrentStepIndex();
    
    if (stepIndex < currentIndex) {
      return 'bg-green-500 text-white'; // Completed
    } else if (stepIndex === currentIndex) {
      return 'bg-gray-900 text-white'; // Current
    } else {
      return 'bg-gray-200 text-gray-500'; // Not started
    }
  };

  return (
    <div className="max-w-4xl mx-auto mb-6 sm:mb-7 md:mb-8 px-3 sm:px-4">
      <div className="flex items-center justify-center space-x-4 sm:space-x-6 md:space-x-8 overflow-x-auto scrollbar-hide">
        {steps.map((step, index) => (
          <div key={step.key} className="flex items-center flex-shrink-0">
            <div className={`w-6 h-6 sm:w-7 sm:h-7 md:w-8 md:h-8 rounded-full flex items-center justify-center text-xs sm:text-sm font-medium ${getStepClassName(index)}`}>
              {step.number}
            </div>
            <span className="ml-1.5 sm:ml-2 text-xs sm:text-sm font-medium text-gray-700 whitespace-nowrap">{step.label}</span>
          </div>
        ))}
      </div>
    </div>
  );
};

export default ProgressIndicator;