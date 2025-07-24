import { ReactNode } from "react";

export interface ButtonProps {
  onClick?: () => void;
  disabled?: boolean;
  className?: string;
  children: ReactNode;
  type?: "button" | "submit" | "reset";
  variant?: "primary" | "secondary" | "outline";
  fullWidth?: boolean;
}

/**
 * Reusable button component with built-in styling
 */
export function Button({
  onClick,
  disabled = false,
  className = "",
  children,
  type = "button",
  variant = "primary",
  fullWidth = false,
}: ButtonProps) {
  const getVariantStyles = () => {
    switch (variant) {
      case "secondary":
        return "bg-gray-500 hover:bg-gray-600 text-white";
      case "outline":
        return "bg-white border-2 border-blue-500 text-blue-500 hover:bg-blue-50";
      case "primary":
      default:
        return "bg-blue-500 hover:bg-blue-600 text-white";
    }
  };

  return (
    <button
      onClick={onClick}
      disabled={disabled}
      type={type}
      className={`rounded-lg px-4 py-2 text-base font-medium transition-colors duration-200 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50 ${
        fullWidth ? "w-full" : ""
      } ${getVariantStyles()} ${className}`.trim()}
    >
      {children}
    </button>
  );
}
