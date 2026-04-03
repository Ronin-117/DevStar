interface MiniSearchInputProps {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
}

export function MiniSearchInput({ value, onChange, placeholder = 'Search...' }: MiniSearchInputProps) {
  if (!value) return null;
  return (
    <div className="relative">
      <input
        type="text"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        className="w-full pl-2 pr-6 py-1 text-xs border rounded focus:outline-none focus:ring-1 focus:ring-indigo-500"
        autoFocus
      />
      {value && (
        <button
          onClick={() => onChange('')}
          className="absolute right-2 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600 text-xs"
        >
          &times;
        </button>
      )}
    </div>
  );
}
