import React from 'react';

interface AfriTokeniLogoProps {
  size?: 'sm' | 'md' | 'lg';
  className?: string;
}

const AfriTokeniLogo: React.FC<AfriTokeniLogoProps> = () => {

  return (
      <img src="/afriTokeni.svg" alt="AfriTokeni Logo" className={`h-5 w-auto`} />
  );
};

export default AfriTokeniLogo;
