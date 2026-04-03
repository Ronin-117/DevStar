import { useEffect, useState } from 'react';
import { useStore } from '../../store';
import type { SharedSprintWithSections } from '../../lib/types';

export function TemplateEditorView() {
  const selectedTemplateId = useStore((s) => s.selectedTemplateId);
  const templates = useStore((s) => s.templates);
  const templateSprints = useStore((s) => s.templateSprints);
  const sharedSections = useStore((s) => s.sharedSections);
  const sharedSprints = useStore((s) => s.sharedSprints);
  const fetchTemplateSprints = useStore((s) => s.fetchTemplateSprints);
  const fetchSharedSections = useStore((s) => s.fetchSharedSections);
  const fetchSharedSprints = useStore((s) => s.fetchSharedSprints);
  const addTemplateSprint = useStore((s) => s.addTemplateSprint);
  const deleteTemplateSprint = useStore((s) => s.deleteTemplateSprint);
  const addTemplateSprintSection = useStore((s) => s.addTemplateSprintSection);
  const deleteTemplateSprintSection = useStore((s) => s.deleteTemplateSprintSection);
  const setView = useStore((s) => s.setView);
  const setEditingProjectId = useStore((s) => s.setEditingProjectId);

  const [showAddSprint, setShowAddSprint] = useState(false);
  const [sprintName, setSprintName] = useState('');
  const [sprintDesc, setSprintDesc] = useState('');
  const [showAddSection, setShowAddSection] = useState<number | null>(null);
  const [sectionSource, setSectionSource] = useState<'shared' | 'shared-sprint'>('shared');
  const [selectedSectionId, setSelectedSectionId] = useState(0);
  const [selectedSprintId, setSelectedSprintId] = useState(0);
  const [isLinked, setIsLinked] = useState(true);
  const [sharedSprintDetail, setSharedSprintDetail] = useState<SharedSprintWithSections | null>(null);

  const template = templates.find((t) => t.id === selectedTemplateId);
  const sprints = selectedTemplateId ? templateSprints.get(selectedTemplateId) : undefined;

  useEffect(() => {
    if (selectedTemplateId) {
      fetchTemplateSprints(selectedTemplateId);
      fetchSharedSections();
      fetchSharedSprints();
    }
  }, [selectedTemplateId]);

  if (!template || !sprints || selectedTemplateId === null) return null;

  const handleAddSprint = () => {
    if (!sprintName.trim()) return;
    addTemplateSprint(selectedTemplateId, sprintName.trim(), sprintDesc.trim());
    setSprintName('');
    setSprintDesc('');
    setShowAddSprint(false);
  };

  const handleAddSection = async (sprintId: number) => {
    if (sectionSource === 'shared' && selectedSectionId) {
      addTemplateSprintSection(sprintId, selectedSectionId, isLinked, selectedTemplateId);
    } else if (sectionSource === 'shared-sprint' && selectedSprintId) {
      if (!sharedSprintDetail) {
        const { apiGetSharedSprintWithSections } = await import('../../lib/api');
        const detail = await apiGetSharedSprintWithSections(selectedSprintId);
        detail.sections.forEach((ss) => {
          addTemplateSprintSection(sprintId, ss.section_id, ss.is_linked, selectedTemplateId);
        });
      }
    }
    setShowAddSection(null);
    setSelectedSectionId(0);
    setSelectedSprintId(0);
    setSharedSprintDetail(null);
  };

  const handleSelectSprint = async (sprintId: number) => {
    setSelectedSprintId(sprintId);
    if (sprintId) {
      const { apiGetSharedSprintWithSections } = await import('../../lib/api');
      const detail = await apiGetSharedSprintWithSections(sprintId);
      setSharedSprintDetail(detail);
    } else {
      setSharedSprintDetail(null);
    }
  };

  return (
    <div>
      <div className="flex items-center gap-3 mb-6">
        <button
          onClick={() => {
            setView('library');
            setEditingProjectId(null);
          }}
          className="text-gray-500 hover:text-gray-700"
        >
          <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
          </svg>
        </button>
        <div>
          <h2 className="text-xl font-semibold">{template.name}</h2>
          <p className="text-sm text-gray-500">{template.description}</p>
        </div>
      </div>

      <div className="space-y-6">
        {sprints.map((sprint) => (
          <div key={sprint.sprint.id} className="bg-white border rounded-xl p-4">
            <div className="flex items-center justify-between mb-3">
              <h3 className="font-semibold">
                Sprint {sprint.sprint.sort_order + 1}: {sprint.sprint.name}
              </h3>
              <div className="flex gap-2">
                <button
                  onClick={() => setShowAddSection(sprint.sprint.id)}
                  className="text-xs px-2 py-1 border rounded hover:bg-gray-50"
                >
                  + Section
                </button>
                <button
                  onClick={() => deleteTemplateSprint(sprint.sprint.id, selectedTemplateId)}
                  className="text-xs px-2 py-1 text-red-600 border border-red-200 rounded hover:bg-red-50"
                >
                  Delete
                </button>
              </div>
            </div>

            {sprint.sections.length > 0 && (
              <div className="space-y-2 mb-3">
                {sprint.sections.map((ss) => {
                  const section = sharedSections.find((s) => s.id === ss.section_id);
                  return (
                    <div
                      key={ss.id}
                      className="flex items-center justify-between px-3 py-2 bg-gray-50 rounded"
                    >
                      <div className="flex items-center gap-2">
                        {section && (
                          <span className="w-2 h-2 rounded-full" style={{ backgroundColor: section.color }} />
                        )}
                        <span className="text-sm">{section?.name ?? 'Unknown'}</span>
                        {ss.is_linked && (
                          <span className="text-xs px-1.5 py-0.5 rounded bg-blue-50 text-blue-600">linked</span>
                        )}
                      </div>
                      <button
                        onClick={() => deleteTemplateSprintSection(ss.id, selectedTemplateId)}
                        className="text-gray-400 hover:text-red-500 text-xs"
                      >
                        &times;
                      </button>
                    </div>
                  );
                })}
              </div>
            )}

            {showAddSection === sprint.sprint.id && (
              <div className="border-t pt-3 space-y-3">
                <div className="flex gap-2">
                  <button
                    onClick={() => setSectionSource('shared')}
                    className={`text-xs px-2 py-1 rounded ${sectionSource === 'shared' ? 'bg-indigo-100 text-indigo-700' : 'border'}`}
                  >
                    Shared Section
                  </button>
                  <button
                    onClick={() => setSectionSource('shared-sprint')}
                    className={`text-xs px-2 py-1 rounded ${sectionSource === 'shared-sprint' ? 'bg-indigo-100 text-indigo-700' : 'border'}`}
                  >
                    Shared Sprint
                  </button>
                </div>

                {sectionSource === 'shared' && (
                  <select
                    value={selectedSectionId}
                    onChange={(e) => setSelectedSectionId(Number(e.target.value))}
                    className="w-full text-sm border rounded px-2 py-1"
                  >
                    <option value={0}>Select section</option>
                    {sharedSections.map((s) => (
                      <option key={s.id} value={s.id}>{s.name}</option>
                    ))}
                  </select>
                )}

                {sectionSource === 'shared-sprint' && (
                  <div className="space-y-2">
                    <select
                      value={selectedSprintId}
                      onChange={(e) => handleSelectSprint(Number(e.target.value))}
                      className="w-full text-sm border rounded px-2 py-1"
                    >
                      <option value={0}>Select sprint</option>
                      {sharedSprints.map((s) => (
                        <option key={s.id} value={s.id}>{s.name}</option>
                      ))}
                    </select>
                    {sharedSprintDetail && (
                      <div className="text-xs text-gray-500">
                        {sharedSprintDetail.sections.length} sections will be added
                      </div>
                    )}
                  </div>
                )}

                <label className="flex items-center gap-2 text-sm">
                  <input
                    type="checkbox"
                    checked={isLinked}
                    onChange={(e) => setIsLinked(e.target.checked)}
                  />
                  Link (auto-updates with source)
                </label>

                <div className="flex gap-2">
                  <button
                    onClick={() => handleAddSection(sprint.sprint.id)}
                    className="text-xs px-3 py-1.5 bg-indigo-600 text-white rounded hover:bg-indigo-700"
                  >
                    Add
                  </button>
                  <button
                    onClick={() => {
                      setShowAddSection(null);
                      setSelectedSectionId(0);
                      setSelectedSprintId(0);
                      setSharedSprintDetail(null);
                    }}
                    className="text-xs px-3 py-1.5 border rounded hover:bg-gray-50"
                  >
                    Cancel
                  </button>
                </div>
              </div>
            )}
          </div>
        ))}

        {showAddSprint ? (
          <div className="bg-white border rounded-xl p-4 space-y-3">
            <input
              value={sprintName}
              onChange={(e) => setSprintName(e.target.value)}
              placeholder="Sprint name"
              className="w-full text-sm border rounded px-3 py-2 focus:outline-none focus:ring-1 focus:ring-indigo-500"
              autoFocus
            />
            <input
              value={sprintDesc}
              onChange={(e) => setSprintDesc(e.target.value)}
              placeholder="Description (optional)"
              className="w-full text-sm border rounded px-3 py-2 focus:outline-none focus:ring-1 focus:ring-indigo-500"
            />
            <div className="flex gap-2">
              <button
                onClick={handleAddSprint}
                className="text-sm px-3 py-1.5 bg-indigo-600 text-white rounded hover:bg-indigo-700"
              >
                Add Sprint
              </button>
              <button
                onClick={() => { setShowAddSprint(false); setSprintName(''); setSprintDesc(''); }}
                className="text-sm px-3 py-1.5 border rounded hover:bg-gray-50"
              >
                Cancel
              </button>
            </div>
          </div>
        ) : (
          <button
            onClick={() => setShowAddSprint(true)}
            className="w-full py-4 text-sm text-indigo-600 border-2 border-dashed border-indigo-200 rounded-xl hover:bg-indigo-50 transition-colors"
          >
            + Add Sprint
          </button>
        )}
      </div>
    </div>
  );
}
