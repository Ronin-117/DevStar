import { useEffect, useState } from 'react';
import { useStore } from './store';
import { apiToggleMode, apiGetWindowLabel } from './lib/api';
import { TitleBar } from './components/shared/TitleBar';
import { ProjectsView } from './components/projects/ProjectsView';
import { ProjectDetailView } from './components/projects/ProjectDetailView';
import { TemplatesView } from './components/templates/TemplatesView';
import { TemplateEditorView } from './components/templates/TemplateEditorView';
import { ActiveMode } from './components/active/ActiveMode';

function App() {
  const view = useStore((s) => s.view);
  const setView = useStore((s) => s.setView);
  const editingProjectId = useStore((s) => s.editingProjectId);
  const selectedTemplateId = useStore((s) => s.selectedTemplateId);
  const fetchTemplates = useStore((s) => s.fetchTemplates);
  const fetchProjects = useStore((s) => s.fetchProjects);
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
  }, [fetchTemplates, fetchProjects]);

  if (windowLabel === 'active') {
    return <ActiveMode />;
  }

  const showProjectDetail = editingProjectId !== null;
  const showTemplateEditor = view === 'template-editor' && selectedTemplateId !== null;

  return (
    <div className="h-screen flex flex-col bg-gray-50">
      <TitleBar />
      <header className="bg-white border-b px-6 py-3 flex items-center justify-between shrink-0" style={{ appRegion: 'drag' } as React.CSSProperties}>
        <div className="flex items-center gap-4">
          <h1 className="text-lg font-bold text-gray-800">ProjectTracker</h1>
          <nav className="flex gap-1 bg-gray-100 rounded-lg p-0.5">
            <button
              onClick={() => {
                setView('projects');
                useStore.getState().setEditingProjectId(null);
                useStore.getState().setSelectedTemplateId(null);
              }}
              className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
                view === 'projects' && !showProjectDetail
                  ? 'bg-white text-gray-800 shadow-sm'
                  : 'text-gray-500 hover:text-gray-700'
              }`}
            >
              Projects
            </button>
            <button
              onClick={() => {
                setView('templates');
                useStore.getState().setEditingProjectId(null);
                useStore.getState().setSelectedTemplateId(null);
              }}
              className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
                view === 'templates' && !showTemplateEditor
                  ? 'bg-white text-gray-800 shadow-sm'
                  : 'text-gray-500 hover:text-gray-700'
              }`}
            >
              Templates
            </button>
          </nav>
        </div>
        <button
          onClick={() => apiToggleMode('active')}
          className="px-3 py-1.5 text-sm bg-indigo-600 text-white rounded-md hover:bg-indigo-700 transition-colors"
          style={{ appRegion: 'no-drag' } as React.CSSProperties}
        >
          Live Mode
        </button>
      </header>

      {error && (
        <div className="mx-6 mt-3 p-3 bg-red-50 border border-red-200 rounded-md flex items-center justify-between shrink-0">
          <span className="text-sm text-red-700">{error}</span>
          <button onClick={clearError} className="text-red-500 hover:text-red-700">×</button>
        </div>
      )}

      <main className="flex-1 overflow-y-auto p-6">
        {showProjectDetail ? (
          <ProjectDetailView />
        ) : showTemplateEditor ? (
          <TemplateEditorView />
        ) : view === 'templates' ? (
          <TemplatesView />
        ) : (
          <ProjectsView />
        )}
      </main>
    </div>
  );
}

export default App;
