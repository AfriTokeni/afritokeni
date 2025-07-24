import { ChangeEvent } from "react";

export interface InputFieldProps {
  value: string;
  onChange: (event: ChangeEvent<HTMLInputElement>) => void;
  placeholder?: string;
  disabled?: boolean;
  className?: string;
  type?: string;
  label?: string;
  required?: boolean;
  maxLength?: number;
}

/**
 * Reusable input field component with built-in styling
 */
export function InputField({
  value,
  onChange,
  placeholder,
  disabled = false,
  className = "",
  type = "text",
  label,
  required = false,
  maxLength,
}: InputFieldProps) {
  return (
    <div className="flex flex-col gap-1">
      {label && (
        <label className="text-sm font-medium text-gray-700">{label}</label>
      )}
      <input
        type={type}
        value={value}
        onChange={onChange}
        placeholder={placeholder}
        disabled={disabled}
        required={required}
        maxLength={maxLength}
        className={`rounded-lg border border-gray-300 bg-white px-4 py-2 text-base text-gray-900 transition-colors duration-200 focus:border-blue-500 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50 ${className}`.trim()}
      />
    </div>
  );
}
