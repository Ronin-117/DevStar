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
  apiCheckAndAdvanceSprint,
} from '../../lib/api';
import { CollapsibleSection } from '../shared/CollapsibleSection';
import type { ProjectSprintWithSections } from '../../lib/types';
import appIcon from '../../assets/app-icon.png';

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

    // Poll every 2 seconds to stay in sync with management window
    const interval = setInterval(() => {
      const currentId = localStorage.getItem('pt_active_project_id');
      if (currentId) {
        const id = Number(currentId);
        if (id !== projectId) {
          setProjectId(id);
        }
        apiListProjectSprints(id)
          .then((data) => setSprints(data))
          .catch(() => { });
      }
    }, 2000);

    return () => clearInterval(interval);
  }, [projectId]);

  const refresh = () => {
    if (projectId) {
      apiListProjectSprints(projectId)
        .then((data) => setSprints(data))
        .catch(() => { });
    }
  };

  const handleMinimize = () => {
    setMinimized(true);
    apiSetActiveWindowCompact().catch(() => { });
  };

  const handleRestore = () => {
    setMinimized(false);
    apiSetActiveWindowFull().catch(() => { });
  };

  const activeSprint = sprints.find((s) => s.sprint.status === 'active');

  if (loading) {
    return (
      <div className="w-full h-full flex items-center justify-center bg-white" style={{ overflow: 'hidden' }}>
        <p className="text-sm text-gray-400">Loading...</p>
      </div>
    );
  }

  if (!projectId || !activeSprint) {
    return (
      <div className="w-full h-full flex items-center justify-center bg-white" style={{ overflow: 'hidden' }}>
        <div className="text-center">
          <p className="text-sm text-gray-400">{projectId ? 'No active sprint' : 'No project selected'}</p>
          <button
            onClick={() => apiToggleMode('management')}
            className="mt-3 px-3 py-1.5 text-xs text-indigo-200 hover:text-white bg-indigo-600 rounded-md hover:bg-indigo-700 transition-colors"
          >
            Back to Management
          </button>
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
          background: 'transparent',
          overflow: 'hidden',
        }}
      >
        <button
          onClick={handleRestore}
          className="w-10 h-10 rounded-full bg-white/90 backdrop-blur-sm shadow-xl hover:bg-white transition-colors flex items-center justify-center overflow-hidden"
          style={{
            ['appRegion' as string]: 'no-drag',
          }}
          title="Open Live Mode"
        >
          <img src={appIcon} alt="" className="w-6 h-6" />
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
            className="w-6 h-6 flex items-center justify-center rounded bg-white/20 hover:bg-white/30 overflow-hidden"
            title="Minimize"
          >
            <img src={appIcon} alt="" className="w-4 h-4" />
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
                apiToggleProjectItem(itemId)
                  .then(() => apiCheckAndAdvanceSprint(projectId))
                  .then(() => refresh())
                  .catch(() => { });
              }}
              onAddItem={(input) => {
                apiAddProjectItem(input).then(() => refresh()).catch(() => { });
              }}
              onDeleteItem={(itemId) => {
                apiDeleteProjectItem(itemId).then(() => refresh()).catch(() => { });
              }}
              onDeleteSection={(sectionId) => {
                apiDeleteProjectSection(sectionId).then(() => refresh()).catch(() => { });
              }}
            />
          </div>
        ))}
      </div>
    </div>
  );
}
