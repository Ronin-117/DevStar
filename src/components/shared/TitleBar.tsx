import { apiCloseWindow, apiMinimizeWindow, apiToggleMaximizeWindow } from '../../lib/api';

const drag = { appRegion: 'drag' } as React.CSSProperties;
const noDrag = { appRegion: 'no-drag' } as React.CSSProperties;

export function TitleBar() {
  return (
    <div
      className="h-8 bg-white border-b border-gray-200 flex items-center justify-between shrink-0 select-none"
      style={drag}
    >
      <div className="flex items-center gap-2 px-3">
        <div className="w-3 h-3 rounded-full bg-indigo-600" />
        <span className="text-xs font-medium text-gray-600">ProjectTracker</span>
      </div>
      <div className="flex items-center h-full">
        <button
          onClick={apiMinimizeWindow}
          className="h-full w-12 flex items-center justify-center text-gray-500 hover:bg-gray-100 hover:text-gray-700 transition-colors"
          style={noDrag}
          title="Minimize"
        >
          <svg className="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2.5}>
            <path strokeLinecap="round" strokeLinejoin="round" d="M5 12h14" />
          </svg>
        </button>
        <button
          onClick={apiToggleMaximizeWindow}
          className="h-full w-12 flex items-center justify-center text-gray-500 hover:bg-gray-100 hover:text-gray-700 transition-colors"
          style={noDrag}
          title="Maximize"
        >
          <svg className="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
            <rect x="5" y="5" width="14" height="14" rx="1" />
          </svg>
        </button>
        <button
          onClick={apiCloseWindow}
          className="h-full w-12 flex items-center justify-center text-gray-500 hover:bg-red-500 hover:text-white transition-colors"
          style={noDrag}
          title="Close"
        >
          <svg className="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2.5}>
            <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
    </div>
  );
}
