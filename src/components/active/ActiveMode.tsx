import { useEffect, useState, useCallback, useRef } from 'react';
import { apiToggleMode, apiListProjectSectionsWithItems, apiUpdateProjectItem, apiListProjects, apiResizeActiveWindow } from '../../lib/api';
import { Checkbox } from '../shared/Checkbox';

interface FlatItem {
  id: number;
  title: string;
  checked: boolean;
  sectionName: string;
}

const ACTIVE_PROJECT_KEY = 'pt_active_project_id';

export function ActiveMode() {
  const [projectId, setProjectId] = useState<number | null>(null);
  const [projectName, setProjectName] = useState('');
  const [items, setItems] = useState<FlatItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [collapsed, setCollapsed] = useState(false);
  const scrollRef = useRef<HTMLDivElement>(null);

  const loadProject = useCallback(async (isPoll = false) => {
    const stored = localStorage.getItem(ACTIVE_PROJECT_KEY);
    if (!stored) {
      setProjectId(null);
      if (!isPoll) setLoading(false);
      return;
    }
    const id = Number(stored);
    setProjectId(id);
    // Only show loading on initial load, not during polling
    if (!isPoll) setLoading(true);
    try {
      const [sections, projects] = await Promise.all([
        apiListProjectSectionsWithItems(id),
        apiListProjects(),
      ]);
      const project = projects.find((p) => p.id === id);
      if (project) setProjectName(project.name);

      const flat: FlatItem[] = [];
      for (const section of sections) {
        for (const item of section.items) {
          if (!item.checked) {
            flat.push({
              id: item.id,
              title: item.title,
              checked: item.checked,
              sectionName: section.section.name,
            });
          }
        }
      }
      setItems(flat);
    } catch {
      // ignore
    } finally {
      if (!isPoll) setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadProject();
    const interval = setInterval(() => loadProject(true), 2000);
    return () => clearInterval(interval);
  }, [loadProject]);

  const handleToggle = async (itemId: number) => {
    try {
      await apiUpdateProjectItem({ id: itemId, checked: true });
      setItems((prev) => prev.filter((i) => i.id !== itemId));
    } catch {
      loadProject(true);
    }
  };

  const handleBack = () => {
    apiToggleMode('management');
  };

  const handleCollapse = async () => {
    await apiResizeActiveWindow(48, 48);
    setCollapsed(true);
  };

  const handleExpand = async () => {
    setCollapsed(false);
    await apiResizeActiveWindow(340, 500);
  };

  // Collapsed state: small floating button
  if (collapsed) {
    return (
      <>
        <style>{`html, body { background: #4f46e5 !important; }`}</style>
        <div
          className="w-full h-full flex items-center justify-center"
          data-tauri-drag-region
          style={{ background: '#4f46e5', cursor: 'grab', overflow: 'hidden' }}
        >
          <button
            onClick={handleExpand}
            className="w-9 h-9 rounded-full bg-indigo-600 hover:bg-indigo-700 text-white flex items-center justify-center shadow-lg transition-transform hover:scale-105"
            title="ProjectTracker Live"
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4" />
            </svg>
          </button>
        </div>
      </>
    );
  }

  // Expanded state: full checklist panel
  return (
    <div className="w-full h-full rounded-xl overflow-hidden shadow-xl border border-gray-200 bg-white flex flex-col">
      <header
        className="px-3 py-2 bg-indigo-50 flex items-center justify-between shrink-0"
        style={{ appRegion: 'drag', cursor: 'grab' } as React.CSSProperties}
      >
        <div className="flex items-center gap-2">
          <button
            onClick={handleCollapse}
            className="text-gray-500 hover:text-gray-700 p-1 rounded hover:bg-indigo-100"
            style={{ appRegion: 'no-drag' } as React.CSSProperties}
            title="Minimize"
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={2}>
              <path strokeLinecap="round" strokeLinejoin="round" d="M20 12H4" />
            </svg>
          </button>
          <div>
            <h2 className="text-sm font-semibold leading-tight">{projectName || 'No project'}</h2>
            <p className="text-xs text-gray-500">{items.length} remaining</p>
          </div>
        </div>
        <div className="flex items-center gap-1">
          <button
            onClick={handleBack}
            className="text-xs text-gray-500 hover:text-gray-700 px-2 py-1 rounded hover:bg-indigo-100"
            style={{ appRegion: 'no-drag' } as React.CSSProperties}
            title="Back to Projects"
          >
            Back
          </button>
        </div>
      </header>

      <div ref={scrollRef} className="flex-1 overflow-auto">
        {loading ? (
          <div className="text-center py-8 text-xs text-gray-400">Loading...</div>
        ) : !projectId ? (
          <div className="text-center py-8">
            <p className="text-sm text-gray-400 mb-2">No active project</p>
            <p className="text-xs text-gray-300">Open a project in management mode first</p>
          </div>
        ) : items.length === 0 ? (
          <div className="text-center py-8">
            <p className="text-sm text-gray-400">All done!</p>
          </div>
        ) : (
          <div className="divide-y divide-gray-50">
            {items.map((item) => (
              <div
                key={item.id}
                className="flex items-center gap-3 px-3 py-2.5 hover:bg-gray-50 transition-colors"
              >
                <Checkbox checked={false} onChange={() => handleToggle(item.id)} />
                <div className="flex-1 min-w-0" style={{ appRegion: 'no-drag' } as React.CSSProperties}>
                  <p className="text-sm truncate">{item.title}</p>
                  <p className="text-xs text-gray-400">{item.sectionName}</p>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
