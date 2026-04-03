import { useEffect, useState } from 'react';
import { useStore } from '../../store';
import {
  apiToggleMode,
  apiCompleteSprint,
  apiAddProjectSprint,
  apiAddSharedSprintToProject,
  apiAddProjectSection,
  apiCreateSharedSection,
  apiAddSharedSectionItem,
  apiAddProjectItem,
  apiDeleteProjectItem,
  apiDeleteProjectSection,
  apiToggleProjectItem,
} from '../../lib/api';
import { CollapsibleSection } from '../shared/CollapsibleSection';

type SectionAddMode = 'custom' | 'shared';
type SprintAddMode = 'custom' | 'shared';

export function ProjectDetailView() {
  const selectedProjectId = useStore((s) => s.selectedProjectId);
  const projectSprints = useStore((s) => s.projectSprints);
  const projects = useStore((s) => s.projects);
  const templates = useStore((s) => s.templates);
  const sharedSections = useStore((s) => s.sharedSections);
  const sharedSprints = useStore((s) => s.sharedSprints);
  const loading = useStore((s) => s.loading);
  const error = useStore((s) => s.error);
  const clearError = useStore((s) => s.clearError);
  const fetchProjectDetail = useStore((s) => s.fetchProjectDetail);
  const setEditingProjectId = useStore((s) => s.setEditingProjectId);

  const [sectionAddMode, setSectionAddMode] = useState<SectionAddMode>('custom');
  const [addingSectionSprintId, setAddingSectionSprintId] = useState<number | null>(null);
  const [selectedSectionId, setSelectedSectionId] = useState(0);
  const [sectionLinked, setSectionLinked] = useState(false);
  const [customSectionName, setCustomSectionName] = useState('');
  const [customSectionItems, setCustomSectionItems] = useState<string[]>([]);
  const [newItemInput, setNewItemInput] = useState('');

  const [sprintAddMode, setSprintAddMode] = useState<SprintAddMode>('custom');
  const [showAddSprint, setShowAddSprint] = useState(false);
  const [selectedSharedSprintId, setSelectedSharedSprintId] = useState(0);
  const [sprintLinked, setSprintLinked] = useState(false);
  const [customSprintName, setCustomSprintName] = useState('');
  const [customSprintDesc, setCustomSprintDesc] = useState('');

  const project = projects.find((p) => p.id === selectedProjectId);
  const template = templates.find((t) => t.id === (project?.template_id ?? 0));
  const sprints = selectedProjectId ? projectSprints.get(selectedProjectId) : undefined;

  useEffect(() => {
    if (selectedProjectId) fetchProjectDetail(selectedProjectId);
  }, [selectedProjectId]);

  if (!project || !sprints) return null;

  const totalChecked = sprints.reduce(
    (sum, s) => sum + s.sections.reduce((s2, sec) => s2 + sec.items.filter((i) => i.checked).length, 0),
    0,
  );
  const totalItems = sprints.reduce(
    (sum, s) => sum + s.sections.reduce((s2, sec) => s2 + sec.items.length, 0),
    0,
  );

  const refresh = () => fetchProjectDetail(selectedProjectId!, true);

  const handleAddSection = async (sprintId: number) => {
    if (sectionAddMode === 'custom') {
      if (!customSectionName.trim()) return;
      const section = await apiCreateSharedSection({ name: customSectionName.trim(), description: '' });
      for (const item of customSectionItems) {
        if (item.trim()) {
          await apiAddSharedSectionItem({ section_id: section.id, title: item.trim() });
        }
      }
      await apiAddProjectSection({ sprint_id: sprintId, name: section.name });
    } else {
      if (!selectedSectionId) return;
      await apiAddProjectSection({
        sprint_id: sprintId,
        name: '',
        linked_from_section_id: sectionLinked ? selectedSectionId : undefined,
      });
    }
    refresh();
    setAddingSectionSprintId(null);
    setSelectedSectionId(0);
    setCustomSectionName('');
    setCustomSectionItems([]);
  };

  const handleAddSprint = async () => {
    if (sprintAddMode === 'custom') {
      if (!customSprintName.trim()) return;
      await apiAddProjectSprint(selectedProjectId!, customSprintName.trim(), customSprintDesc.trim());
    } else {
      if (!selectedSharedSprintId) return;
      await apiAddSharedSprintToProject(selectedProjectId!, selectedSharedSprintId, sprintLinked);
    }
    refresh();
    setShowAddSprint(false);
    setCustomSprintName('');
    setCustomSprintDesc('');
    setSelectedSharedSprintId(0);
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
                    {sprint.sprint.status === 'active' && (
                      <span className="text-xs px-2 py-0.5 rounded-full bg-indigo-100 text-indigo-700 font-medium">
                        Current Sprint
                      </span>
                    )}
                  </div>
                  <div className="flex gap-2">
                    {sprint.sprint.status === 'active' && (
                      <button
                        onClick={() => apiCompleteSprint(sprint.sprint.id, selectedProjectId!).then(refresh).catch(() => {})}
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
                      onToggleItem={(itemId) => {
                        apiToggleProjectItem(itemId).then(refresh).catch(() => {});
                      }}
                      onAddItem={(input) => {
                        apiAddProjectItem(input).then(refresh).catch(() => {});
                      }}
                      onDeleteItem={(itemId) => {
                        apiDeleteProjectItem(itemId).then(refresh).catch(() => {});
                      }}
                      onDeleteSection={(sectionId) => {
                        apiDeleteProjectSection(sectionId).then(refresh).catch(() => {});
                      }}
                    />
                  ))}
                </div>

                {addingSectionSprintId === sprint.sprint.id && (
                  <div className="mt-3 border rounded-lg p-3 space-y-2">
                    <div className="flex gap-1 bg-gray-100 rounded p-0.5 w-fit">
                      <button
                        onClick={() => setSectionAddMode('custom')}
                        className={`text-xs px-2 py-1 rounded ${sectionAddMode === 'custom' ? 'bg-white shadow-sm' : 'text-gray-500'}`}
                      >
                        Custom
                      </button>
                      <button
                        onClick={() => setSectionAddMode('shared')}
                        className={`text-xs px-2 py-1 rounded ${sectionAddMode === 'shared' ? 'bg-white shadow-sm' : 'text-gray-500'}`}
                      >
                        From Shared
                      </button>
                    </div>

                    {sectionAddMode === 'custom' ? (
                      <div className="space-y-2">
                        <input
                          value={customSectionName}
                          onChange={(e) => setCustomSectionName(e.target.value)}
                          placeholder="Section name"
                          className="w-full text-sm border rounded px-2 py-1"
                          autoFocus
                        />
                        <div className="space-y-1">
                          {customSectionItems.map((item, i) => (
                            <div key={i} className="flex items-center gap-2 px-2 py-1 bg-gray-50 rounded">
                              <span className="text-xs flex-1">{item}</span>
                              <button
                                onClick={() => setCustomSectionItems((prev) => prev.filter((_, idx) => idx !== i))}
                                className="text-gray-400 hover:text-red-500 text-xs"
                              >
                                &times;
                              </button>
                            </div>
                          ))}
                          <div className="flex gap-2">
                            <input
                              value={newItemInput}
                              onChange={(e) => setNewItemInput(e.target.value)}
                              onKeyDown={(e) => {
                                if (e.key === 'Enter' && newItemInput.trim()) {
                                  setCustomSectionItems((prev) => [...prev, newItemInput.trim()]);
                                  setNewItemInput('');
                                }
                              }}
                              placeholder="Item title..."
                              className="flex-1 text-xs border rounded px-2 py-1"
                            />
                            <button
                              onClick={() => {
                                if (newItemInput.trim()) {
                                  setCustomSectionItems((prev) => [...prev, newItemInput.trim()]);
                                  setNewItemInput('');
                                }
                              }}
                              className="text-xs px-2 py-1 border rounded hover:bg-gray-50"
                            >
                              +
                            </button>
                          </div>
                        </div>
                      </div>
                    ) : (
                      <div className="flex gap-2 flex-wrap">
                        <select
                          value={selectedSectionId}
                          onChange={(e) => setSelectedSectionId(Number(e.target.value))}
                          className="flex-1 text-sm border rounded px-2 py-1"
                        >
                          <option value={0}>Select section</option>
                          {sharedSections.map((s) => (
                            <option key={s.id} value={s.id}>{s.name}</option>
                          ))}
                        </select>
                        <label className="flex items-center gap-1 text-xs">
                          <input
                            type="checkbox"
                            checked={sectionLinked}
                            onChange={(e) => setSectionLinked(e.target.checked)}
                          />
                          Link
                        </label>
                      </div>
                    )}

                    <div className="flex gap-2">
                      <button
                        onClick={() => handleAddSection(sprint.sprint.id)}
                        className="text-xs px-3 py-1.5 bg-indigo-600 text-white rounded hover:bg-indigo-700"
                      >
                        Add Section
                      </button>
                      <button
                        onClick={() => {
                          setAddingSectionSprintId(null);
                          setCustomSectionName('');
                          setCustomSectionItems([]);
                          setSelectedSectionId(0);
                        }}
                        className="text-xs px-3 py-1.5 border rounded hover:bg-gray-50"
                      >
                        Cancel
                      </button>
                    </div>
                  </div>
                )}
              </div>
            );
          })}

          {showAddSprint ? (
            <div className="border rounded-xl p-4 space-y-2">
              <div className="flex gap-1 bg-gray-100 rounded p-0.5 w-fit">
                <button
                  onClick={() => setSprintAddMode('custom')}
                  className={`text-xs px-2 py-1 rounded ${sprintAddMode === 'custom' ? 'bg-white shadow-sm' : 'text-gray-500'}`}
                >
                  Custom
                </button>
                <button
                  onClick={() => setSprintAddMode('shared')}
                  className={`text-xs px-2 py-1 rounded ${sprintAddMode === 'shared' ? 'bg-white shadow-sm' : 'text-gray-500'}`}
                >
                  From Shared
                </button>
              </div>

              {sprintAddMode === 'custom' ? (
                <>
                  <input
                    value={customSprintName}
                    onChange={(e) => setCustomSprintName(e.target.value)}
                    placeholder="Sprint name"
                    className="w-full text-sm border rounded px-3 py-2"
                    autoFocus
                  />
                  <input
                    value={customSprintDesc}
                    onChange={(e) => setCustomSprintDesc(e.target.value)}
                    placeholder="Description (optional)"
                    className="w-full text-sm border rounded px-3 py-2"
                  />
                </>
              ) : (
                <div className="flex gap-2">
                  <select
                    value={selectedSharedSprintId}
                    onChange={(e) => setSelectedSharedSprintId(Number(e.target.value))}
                    className="flex-1 text-sm border rounded px-2 py-2"
                  >
                    <option value={0}>Select shared sprint</option>
                    {sharedSprints.map((s) => (
                      <option key={s.id} value={s.id}>{s.name}</option>
                    ))}
                  </select>
                  <label className="flex items-center gap-1 text-xs">
                    <input
                      type="checkbox"
                      checked={sprintLinked}
                      onChange={(e) => setSprintLinked(e.target.checked)}
                    />
                    Link
                  </label>
                </div>
              )}

              <div className="flex gap-2">
                <button
                  onClick={handleAddSprint}
                  className="text-sm px-3 py-1.5 bg-indigo-600 text-white rounded hover:bg-indigo-700"
                >
                  Add Sprint
                </button>
                <button
                  onClick={() => {
                    setShowAddSprint(false);
                    setCustomSprintName('');
                    setCustomSprintDesc('');
                    setSelectedSharedSprintId(0);
                  }}
                  className="text-sm px-3 py-1.5 border rounded hover:bg-gray-50"
                >
                  Cancel
                </button>
              </div>
            </div>
          ) : (
            <button
              onClick={() => setShowAddSprint(true)}
              className="w-full py-3 text-sm text-indigo-600 border-2 border-dashed border-indigo-200 rounded-xl hover:bg-indigo-50 transition-colors"
            >
              + Add Sprint
            </button>
          )}
        </div>
      )}
    </div>
  );
}
