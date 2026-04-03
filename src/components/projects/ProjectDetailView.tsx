import { useEffect, useState } from 'react';
import { useStore } from '../../store';
import { apiToggleMode } from '../../lib/api';
import { CollapsibleSection } from '../shared/CollapsibleSection';

export function ProjectDetailView() {
  const selectedProjectId = useStore((s) => s.selectedProjectId);
  const projectSprints = useStore((s) => s.projectSprints);
  const projects = useStore((s) => s.projects);
  const templates = useStore((s) => s.templates);
  const loading = useStore((s) => s.loading);
  const error = useStore((s) => s.error);
  const clearError = useStore((s) => s.clearError);
  const fetchProjectDetail = useStore((s) => s.fetchProjectDetail);
  const setEditingProjectId = useStore((s) => s.setEditingProjectId);
  const setSprintStatus = useStore((s) => s.setSprintStatus);

  const [addingSectionSprintId, setAddingSectionSprintId] = useState<number | null>(null);
  const [newSectionName, setNewSectionName] = useState('');

  const project = projects.find((p) => p.id === selectedProjectId);
  const template = templates.find((t) => t.id === (project?.template_id ?? 0));
  const sprints = selectedProjectId ? projectSprints.get(selectedProjectId) : undefined;

  useEffect(() => {
    if (selectedProjectId) fetchProjectDetail(selectedProjectId);
  }, [selectedProjectId, fetchProjectDetail]);

  if (!project || !sprints) return null;

  const totalChecked = sprints.reduce(
    (sum, s) => sum + s.sections.reduce((s2, sec) => s2 + sec.items.filter((i) => i.checked).length, 0),
    0,
  );
  const totalItems = sprints.reduce(
    (sum, s) => sum + s.sections.reduce((s2, sec) => s2 + sec.items.length, 0),
    0,
  );

  const handleAddSection = (sprintId: number) => {
    if (!newSectionName.trim()) return;
    useStore.getState().addProjectSection({ sprint_id: sprintId, name: newSectionName.trim() }, selectedProjectId!);
    setNewSectionName('');
    setAddingSectionSprintId(null);
  };

  return (
    <div>
      <div className="flex items-center gap-3 mb-4">
        <button
          onClick={() => {
            setEditingProjectId(null);
            useStore.getState().setSelectedProjectId(null);
          }}
          className="text-gray-500 hover:text-gray-700"
        >
          <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
          </svg>
        </button>
        <div className="flex-1">
          <h2 className="text-xl font-semibold">{project.name}</h2>
          {template && <span className="text-xs" style={{ color: template.color }}>{template.name}</span>}
        </div>
        <button
          onClick={() => apiToggleMode('active')}
          className="px-3 py-1.5 text-sm bg-green-600 text-white rounded-md hover:bg-green-700 transition-colors"
        >
          Live Mode
        </button>
      </div>

      <div className="mb-6 bg-white rounded-xl p-4 border">
        <div className="flex items-center justify-between mb-2">
          <span className="text-sm font-medium">Overall Progress</span>
          <span className="text-sm text-gray-500">{totalChecked}/{totalItems}</span>
        </div>
        <div className="w-full bg-gray-200 rounded-full h-3 overflow-hidden">
          <div
            className="h-3 rounded-full bg-indigo-600 transition-all duration-500"
            style={{ width: totalItems > 0 ? `${(totalChecked / totalItems) * 100}%` : '0%' }}
          />
        </div>
      </div>

      {error && (
        <div className="mb-4 p-3 bg-red-50 border border-red-200 rounded-md flex items-center justify-between">
          <span className="text-sm text-red-700">{error}</span>
          <button onClick={clearError} className="text-red-500 hover:text-red-700">&times;</button>
        </div>
      )}

      {loading ? (
        <div className="text-center py-8 text-gray-400">Loading...</div>
      ) : (
        <div className="space-y-6">
          {sprints.map((sprint) => {
            const sprintChecked = sprint.sections.reduce((sum, s) => sum + s.items.filter((i) => i.checked).length, 0);
            const sprintTotal = sprint.sections.reduce((sum, s) => sum + s.items.length, 0);
            const statusColor = sprint.sprint.status === 'active'
              ? 'bg-blue-100 text-blue-700'
              : sprint.sprint.status === 'done'
              ? 'bg-green-100 text-green-700'
              : 'bg-gray-100 text-gray-600';

            return (
              <div key={sprint.sprint.id}>
                <div className="flex items-center justify-between mb-3">
                  <div className="flex items-center gap-3">
                    <h3 className="font-semibold">
                      Sprint {sprint.sprint.sort_order + 1}: {sprint.sprint.name}
                    </h3>
                    <span className={`text-xs px-2 py-0.5 rounded-full ${statusColor}`}>
                      {sprint.sprint.status}
                    </span>
                  </div>
                  <div className="flex gap-2">
                    {sprint.sprint.status === 'pending' && (
                      <button
                        onClick={() => setSprintStatus(sprint.sprint.id, 'active', selectedProjectId!)}
                        className="text-xs px-2 py-1 bg-blue-600 text-white rounded hover:bg-blue-700"
                      >
                        Start
                      </button>
                    )}
                    {sprint.sprint.status === 'active' && (
                      <button
                        onClick={() => setSprintStatus(sprint.sprint.id, 'done', selectedProjectId!)}
                        className="text-xs px-2 py-1 bg-green-600 text-white rounded hover:bg-green-700"
                      >
                        Complete
                      </button>
                    )}
                    <button
                      onClick={() => setAddingSectionSprintId(sprint.sprint.id)}
                      className="text-xs px-2 py-1 border rounded hover:bg-gray-50"
                    >
                      + Section
                    </button>
                  </div>
                </div>

                <div className="space-y-3">
                  {sprint.sections.map((section) => (
                    <CollapsibleSection
                      key={section.section.id}
                      section={section}
                      projectId={selectedProjectId!}
                    />
                  ))}
                </div>

                {sprintChecked}/{sprintTotal}

                {addingSectionSprintId === sprint.sprint.id && (
                  <div className="mt-3 flex gap-2">
                    <input
                      value={newSectionName}
                      onChange={(e) => setNewSectionName(e.target.value)}
                      onKeyDown={(e) => e.key === 'Enter' && handleAddSection(sprint.sprint.id)}
                      placeholder="Section name..."
                      className="flex-1 text-sm border rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-indigo-500"
                      autoFocus
                    />
                    <button
                      onClick={() => handleAddSection(sprint.sprint.id)}
                      className="px-3 py-2 text-sm bg-indigo-600 text-white rounded-md hover:bg-indigo-700"
                    >
                      Add
                    </button>
                    <button
                      onClick={() => { setAddingSectionSprintId(null); setNewSectionName(''); }}
                      className="px-3 py-2 text-sm border rounded-md hover:bg-gray-50"
                    >
                      Cancel
                    </button>
                  </div>
                )}
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
