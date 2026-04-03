import { cn } from '../../lib/utils';

interface ProgressBarProps {
  checked: number;
  total: number;
  size?: 'sm' | 'md' | 'lg';
  color?: string;
}

export function ProgressBar({ checked, total, size = 'md', color = 'bg-indigo-600' }: ProgressBarProps) {
  const pct = total > 0 ? Math.round((checked / total) * 100) : 0;
  const height = size === 'sm' ? 'h-1' : size === 'md' ? 'h-2' : 'h-3';

  return (
    <div className="w-full">
      <div className={cn('w-full bg-gray-200 rounded-full overflow-hidden', height)}>
        <div
          className={cn('rounded-full transition-all duration-300', color, height)}
          style={{ width: `${pct}%` }}
        />
      </div>
      <span className="text-xs text-gray-500 mt-0.5">{checked}/{total} ({pct}%)</span>
    </div>
  );
}
