import { useEffect, useState } from 'react';
import { useStore } from './store';
import type { LibraryTab } from './store';
import { apiGetWindowLabel } from './lib/api';
import { TitleBar } from './components/shared/TitleBar';
import { ProjectsView } from './components/projects/ProjectsView';
import { ProjectDetailView } from './components/projects/ProjectDetailView';
import { TemplatesView } from './components/templates/TemplatesView';
import { SharedSectionsView } from './components/templates/SharedSectionsView';
import { SharedSprintsView } from './components/templates/SharedSprintsView';
import { TemplateEditorView } from './components/templates/TemplateEditorView';
import { ActiveMode } from './components/active/ActiveMode';
import logoBar from './assets/logo-bar.png';

const libraryTabs: { key: LibraryTab; label: string }[] = [
  { key: 'templates', label: 'Templates' },
  { key: 'shared-sprints', label: 'Shared Sprints' },
  { key: 'shared-sections', label: 'Shared Sections' },
];

function App() {
  const view = useStore((s) => s.view);
  const libraryTab = useStore((s) => s.libraryTab);
  const setView = useStore((s) => s.setView);
  const setLibraryTab = useStore((s) => s.setLibraryTab);
  const editingProjectId = useStore((s) => s.editingProjectId);
  const selectedTemplateId = useStore((s) => s.selectedTemplateId);
  const fetchTemplates = useStore((s) => s.fetchTemplates);
  const fetchProjects = useStore((s) => s.fetchProjects);
  const fetchProjectDetail = useStore((s) => s.fetchProjectDetail);
  const error = useStore((s) => s.error);
  const clearError = useStore((s) => s.clearError);
  const [windowLabel, setWindowLabel] = useState('management');

  useEffect(() => {
    apiGetWindowLabel().then((label) => {
      console.log('[App] window label:', label);
      setWindowLabel(label);
    }).catch((e) => {
      console.error('[App] failed to get window label:', e);
      setWindowLabel('management');
    });
  }, []);

  useEffect(() => {
    fetchTemplates();
    fetchProjects();

    // Poll every 3 seconds so changes made by MCP agents appear live in the UI.
    // Also refreshes the selected project detail so the main window sees live updates.
    const interval = setInterval(() => {
      fetchProjects();
      const pid = useStore.getState().selectedProjectId;
      if (pid) {
        fetchProjectDetail(pid, true); // silent = no loading spinner
      }
    }, 3000);

    return () => clearInterval(interval);
  }, [fetchTemplates, fetchProjects, fetchProjectDetail]);

  if (windowLabel === 'active') {
    return <ActiveMode />;
  }

  const showProjectDetail = editingProjectId !== null;
  const showTemplateEditor = view === 'template-editor' && selectedTemplateId !== null;
  const isInLibrary = view === 'library';

  const renderLibraryContent = () => {
    switch (libraryTab) {
      case 'shared-sections':
        return <SharedSectionsView />;
      case 'shared-sprints':
        return <SharedSprintsView />;
      default:
        return <TemplatesView />;
    }
  };

  return (
    <div className="h-screen flex flex-col bg-gray-50">
      <TitleBar />
      <header className="bg-white border-b px-6 py-3 flex items-center justify-between shrink-0" style={{ appRegion: 'drag' } as React.CSSProperties}>
        <div className="flex items-center gap-4">
          <img src={logoBar} alt="DevStar" className="h-7" />
          <nav className="flex gap-1 bg-gray-100 rounded-lg p-0.5">
            <button
              onClick={() => {
                setView('projects');
                useStore.getState().setEditingProjectId(null);
                useStore.getState().setSelectedTemplateId(null);
              }}
              className={`px-3 py-1.5 text-sm rounded-md transition-colors ${view === 'projects' && !showProjectDetail
                  ? 'bg-white text-gray-800 shadow-sm'
                  : 'text-gray-500 hover:text-gray-700'
                }`}
            >
              Projects
            </button>
            <button
              onClick={() => {
                setView('library');
                useStore.getState().setEditingProjectId(null);
                useStore.getState().setSelectedTemplateId(null);
              }}
              className={`px-3 py-1.5 text-sm rounded-md transition-colors ${isInLibrary && !showTemplateEditor
                  ? 'bg-white text-gray-800 shadow-sm'
                  : 'text-gray-500 hover:text-gray-700'
                }`}
            >
              Library
            </button>
          </nav>
        </div>
      </header>

      {isInLibrary && !showTemplateEditor && (
        <div className="bg-white border-b px-6 py-2 shrink-0">
          <div className="flex gap-1 bg-gray-100 rounded-lg p-0.5 w-fit">
            {libraryTabs.map((tab) => (
              <button
                key={tab.key}
                onClick={() => setLibraryTab(tab.key)}
                className={`px-3 py-1.5 text-sm rounded-md transition-colors ${libraryTab === tab.key
                    ? 'bg-white text-gray-800 shadow-sm'
                    : 'text-gray-500 hover:text-gray-700'
                  }`}
              >
                {tab.label}
              </button>
            ))}
          </div>
        </div>
      )}

      {error && (
        <div className="mx-6 mt-3 p-3 bg-red-50 border border-red-200 rounded-md flex items-center justify-between shrink-0">
          <span className="text-sm text-red-700">{error}</span>
          <button onClick={clearError} className="text-red-500 hover:text-red-700">&times;</button>
        </div>
      )}

      <main className="flex-1 overflow-y-auto p-6">
        {showProjectDetail ? (
          <ProjectDetailView />
        ) : showTemplateEditor ? (
          <TemplateEditorView />
        ) : isInLibrary ? (
          renderLibraryContent()
        ) : (
          <ProjectsView />
        )}
      </main>
    </div>
  );
}

export default App;
