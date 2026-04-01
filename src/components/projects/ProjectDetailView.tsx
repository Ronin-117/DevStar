import { useEffect, useState, useRef } from 'react';
import { useStore } from '../../store';
import { apiToggleMode } from '../../lib/api';
import { CollapsibleSection } from '../shared/CollapsibleSection';

export function ProjectDetailView() {
  const selectedProjectId = useStore((s) => s.selectedProjectId);
  const projectSections = useStore((s) => s.projectSections);
  const projectProgress = useStore((s) => s.projectProgress);
  const projects = useStore((s) => s.projects);
  const templates = useStore((s) => s.templates);
  const loading = useStore((s) => s.loading);
  const error = useStore((s) => s.error);
  const clearError = useStore((s) => s.clearError);
  const fetchProjectDetail = useStore((s) => s.fetchProjectDetail);
  const setEditingProjectId = useStore((s) => s.setEditingProjectId);
  const addProjectSection = useStore((s) => s.addProjectSection);

  const [addingSection, setAddingSection] = useState(false);
  const [newSectionName, setNewSectionName] = useState('');
  const [initialLoad, setInitialLoad] = useState(true);
  const scrollRef = useRef<HTMLDivElement>(null);

  const project = projects.find((p) => p.id === selectedProjectId);
  const template = templates.find((t) => t.id === (project?.template_id ?? 0));

  useEffect(() => {
    if (selectedProjectId) {
      fetchProjectDetail(selectedProjectId);
      setInitialLoad(true);
    }
  }, [selectedProjectId, fetchProjectDetail]);

  // Restore collapsed states after data refresh
  useEffect(() => {
    if (projectSections.length > 0) {
      if (initialLoad) {
        setInitialLoad(false);
      }
    }
  }, [projectSections, initialLoad]);

  if (!project) return null;

  const handleAddSection = () => {
    if (!newSectionName.trim()) return;
    addProjectSection(newSectionName.trim());
    setNewSectionName('');
    setAddingSection(false);
  };

  return (
    <div ref={scrollRef}>
      <div className="flex items-center gap-3 mb-2">
        <button
          onClick={() => setEditingProjectId(null)}
          className="text-gray-500 hover:text-gray-700"
        >
          <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
          </svg>
        </button>
        <div>
          <h2 className="text-xl font-semibold">{project.name}</h2>
          {template && (
            <span className="text-xs text-gray-500" style={{ color: template.color }}>
              {template.name}
            </span>
          )}
        </div>
      </div>

      {projectProgress && (
        <div className="mb-6 bg-white rounded-xl p-4 border">
          <div className="flex items-center justify-between mb-2">
            <span className="text-sm font-medium">Overall Progress</span>
            <span className="text-sm text-gray-500">
              {projectProgress[0]}/{projectProgress[1]}
            </span>
          </div>
          <div className="w-full bg-gray-200 rounded-full h-3 overflow-hidden">
            <div
              className="h-3 rounded-full bg-indigo-600 transition-all duration-500"
              style={{
                width: projectProgress[1] > 0 ? `${(projectProgress[0] / projectProgress[1]) * 100}%` : '0%',
              }}
            />
          </div>
        </div>
      )}

      <div className="flex items-center justify-between mb-4">
        <h3 className="text-sm font-medium text-gray-500">Checklist</h3>
        <button
          onClick={() => apiToggleMode('active')}
          className="px-3 py-1.5 text-xs bg-indigo-600 text-white rounded-md hover:bg-indigo-700"
        >
          Live Mode
        </button>
      </div>

      {error && (
        <div className="mb-4 p-3 bg-red-50 border border-red-200 rounded-md flex items-center justify-between">
          <span className="text-sm text-red-700">{error}</span>
          <button onClick={clearError} className="text-red-500 hover:text-red-700">×</button>
        </div>
      )}

      {initialLoad && loading ? (
        <div className="text-center py-8 text-gray-400">Loading...</div>
      ) : (
        <div className="space-y-3">
          {projectSections.map((section) => (
            <CollapsibleSection key={section.section.id} section={section} />
          ))}
        </div>
      )}

      {addingSection ? (
        <div className="mt-4 flex gap-2">
          <input
            value={newSectionName}
            onChange={(e) => setNewSectionName(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && handleAddSection()}
            placeholder="Section name..."
            className="flex-1 text-sm border rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-indigo-500"
            autoFocus
          />
          <button
            onClick={handleAddSection}
            className="px-3 py-2 text-sm bg-indigo-600 text-white rounded-md hover:bg-indigo-700"
          >
            Add
          </button>
          <button
            onClick={() => {
              setAddingSection(false);
              setNewSectionName('');
            }}
            className="px-3 py-2 text-sm border rounded-md hover:bg-gray-50"
          >
            Cancel
          </button>
        </div>
      ) : (
        <button
          onClick={() => setAddingSection(true)}
          className="w-full mt-4 py-3 text-sm text-indigo-600 border-2 border-dashed border-indigo-200 rounded-lg hover:bg-indigo-50 transition-colors"
        >
          + Add custom section
        </button>
      )}
    </div>
  );
}
