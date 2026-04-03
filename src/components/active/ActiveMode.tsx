import { useEffect, useState } from 'react';
import {
  apiToggleMode,
  apiListProjectSprints,
  apiToggleProjectItem,
  apiAddProjectItem,
  apiDeleteProjectItem,
  apiDeleteProjectSection,
  apiSetActiveWindowCompact,
  apiSetActiveWindowFull,
} from '../../lib/api';
import { CollapsibleSection } from '../shared/CollapsibleSection';
import type { ProjectSprintWithSections } from '../../lib/types';

export function ActiveMode() {
  const [sprints, setSprints] = useState<ProjectSprintWithSections[]>([]);
  const [loading, setLoading] = useState(true);
  const [projectId, setProjectId] = useState<number | null>(null);
  const [minimized, setMinimized] = useState(false);

  useEffect(() => {
    const stored = localStorage.getItem('pt_active_project_id');
    if (stored) {
      const id = Number(stored);
      setProjectId(id);
      apiListProjectSprints(id)
        .then((data) => {
          setSprints(data);
          setLoading(false);
        })
        .catch(() => setLoading(false));
    } else {
      setLoading(false);
    }
  }, []);

  const refresh = () => {
    if (projectId) {
      apiListProjectSprints(projectId)
        .then((data) => setSprints(data))
        .catch(() => {});
    }
  };

  const handleMinimize = () => {
    setMinimized(true);
    apiSetActiveWindowCompact().catch(() => {});
  };

  const handleRestore = () => {
    setMinimized(false);
    apiSetActiveWindowFull().catch(() => {});
  };

  const activeSprint = sprints.find((s) => s.sprint.status === 'active');

  if (loading) {
    return (
      <div className="w-full h-full flex items-center justify-center bg-transparent" style={{ overflow: 'hidden' }}>
        <p className="text-sm text-gray-400">Loading...</p>
      </div>
    );
  }

  if (!projectId || !activeSprint) {
    return (
      <div className="w-full h-full flex items-center justify-center bg-transparent" style={{ overflow: 'hidden' }}>
        <div className="text-center">
          <p className="text-sm text-gray-400">{projectId ? 'No active sprint' : 'No project selected'}</p>
        </div>
      </div>
    );
  }

  const totalItems = activeSprint.sections.reduce((sum, s) => sum + s.items.length, 0);
  const checkedItems = activeSprint.sections.reduce(
    (sum, s) => sum + s.items.filter((i) => i.checked).length,
    0,
  );

  if (minimized) {
    return (
      <div
        className="w-full h-full flex items-center justify-center"
        style={{
          background: '#4f46e5',
          overflow: 'hidden',
        }}
      >
        <button
          onClick={handleRestore}
          className="w-10 h-10 rounded-full bg-indigo-600 text-white shadow-lg hover:bg-indigo-500 transition-colors flex items-center justify-center"
          style={{
            ['appRegion' as string]: 'no-drag',
          }}
          title="Open Live Mode"
        >
          <svg className="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2} strokeLinecap="round" strokeLinejoin="round">
            <path d="M9 11l3 3L22 4" />
            <path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11" />
          </svg>
        </button>
      </div>
    );
  }

  return (
    <div className="w-full h-full flex flex-col bg-gray-50">
      <header
        className="px-3 py-2 bg-indigo-600 flex items-center justify-between shrink-0"
        style={{ ['appRegion' as string]: 'drag', cursor: 'grab' }}
      >
        <div>
          <h2 className="text-sm font-bold text-white">
            Sprint {activeSprint.sprint.sort_order + 1}: {activeSprint.sprint.name}
          </h2>
          <p className="text-xs text-indigo-200">{checkedItems}/{totalItems} complete</p>
        </div>
        <div className="flex items-center gap-1" style={{ ['appRegion' as string]: 'no-drag' }}>
          <button
            onClick={handleMinimize}
            className="w-6 h-6 flex items-center justify-center rounded text-indigo-200 hover:text-white hover:bg-indigo-700"
            title="Minimize"
          >
            <svg className="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2} strokeLinecap="round" strokeLinejoin="round">
              <path d="M9 11l3 3L22 4" />
              <path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11" />
            </svg>
          </button>
          <button
            onClick={() => apiToggleMode('management')}
            className="px-2 py-1 text-xs text-indigo-200 hover:text-white hover:bg-indigo-700 rounded"
          >
            Back
          </button>
        </div>
      </header>

      <div className="flex-1 overflow-auto p-2 space-y-2">
        {activeSprint.sections.map((section) => (
          <div key={section.section.id} className="border rounded-lg overflow-hidden bg-white">
            <CollapsibleSection
              section={section}
              projectId={projectId}
              onToggleItem={(itemId) => {
                apiToggleProjectItem(itemId).then(() => refresh()).catch(() => {});
              }}
              onAddItem={(input) => {
                apiAddProjectItem(input).then(() => refresh()).catch(() => {});
              }}
              onDeleteItem={(itemId) => {
                apiDeleteProjectItem(itemId).then(() => refresh()).catch(() => {});
              }}
              onDeleteSection={(sectionId) => {
                apiDeleteProjectSection(sectionId).then(() => refresh()).catch(() => {});
              }}
            />
          </div>
        ))}
      </div>
    </div>
  );
}
