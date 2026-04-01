import { cn } from '../../lib/utils';

interface CheckboxProps {
  checked: boolean;
  onChange: () => void;
  className?: string;
}

export function Checkbox({ checked, onChange, className }: CheckboxProps) {
  return (
    <button
      onClick={onChange}
      className={cn(
        'w-4 h-4 rounded border flex items-center justify-center shrink-0 transition-colors',
        checked
          ? 'bg-indigo-600 border-indigo-600'
          : 'border-gray-300 hover:border-indigo-400',
        className,
      )}
    >
      {checked && (
        <svg className="w-3 h-3 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={3}>
          <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
        </svg>
      )}
    </button>
  );
}
