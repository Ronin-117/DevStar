import { useEffect, useState } from 'react';
import { useStore } from '../../store';
import { apiCreateTemplate, apiAddTemplateSprint, apiGetSharedSprintWithSections } from '../../lib/api';
import { Modal } from '../shared/Modal';

type SprintAddMode = 'custom' | 'shared';
type SectionAddMode = 'custom' | 'shared';

export function TemplatesView() {
  const templates = useStore((s) => s.templates);
  const templateSprints = useStore((s) => s.templateSprints);
  const sharedSections = useStore((s) => s.sharedSections);
  const sharedSprints = useStore((s) => s.sharedSprints);
  const fetchTemplates = useStore((s) => s.fetchTemplates);
  const fetchSharedSections = useStore((s) => s.fetchSharedSections);
  const fetchSharedSprints = useStore((s) => s.fetchSharedSprints);
  const fetchTemplateSprints = useStore((s) => s.fetchTemplateSprints);
  const deleteTemplate = useStore((s) => s.deleteTemplate);
  const addTemplateSprint = useStore((s) => s.addTemplateSprint);
  const deleteTemplateSprint = useStore((s) => s.deleteTemplateSprint);
  const addTemplateSprintSection = useStore((s) => s.addTemplateSprintSection);
  const deleteTemplateSprintSection = useStore((s) => s.deleteTemplateSprintSection);

  const [expandedTemplateId, setExpandedTemplateId] = useState<number | null>(null);
  const [expandedSprintId, setExpandedSprintId] = useState<number | null>(null);
  const [showCreate, setShowCreate] = useState(false);
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [color, setColor] = useState('#6366f1');

  const [sprintAddMode, setSprintAddMode] = useState<SprintAddMode>('custom');
  const [newSprintName, setNewSprintName] = useState('');
  const [newSprintDesc, setNewSprintDesc] = useState('');
  const [selectedSharedSprintId, setSelectedSharedSprintId] = useState(0);
  const [sprintLinkMode, setSprintLinkMode] = useState(true);
  const [addingSprintToTemplate, setAddingSprintToTemplate] = useState<number | null>(null);

  const [sectionAddMode, setSectionAddMode] = useState<SectionAddMode>('shared');
  const [addingSectionToSprint, setAddingSectionToSprint] = useState<number | null>(null);
  const [selectedSectionId, setSelectedSectionId] = useState(0);
  const [sectionLinkMode, setSectionLinkMode] = useState(true);
  const [customSectionName, setCustomSectionName] = useState('');
  const [customSectionItems, setCustomSectionItems] = useState<string[]>([]);
  const [newItemInput, setNewItemInput] = useState('');

  useEffect(() => {
    fetchTemplates();
    fetchSharedSections();
    fetchSharedSprints();
  }, []);

  const handleCreate = (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) return;
    apiCreateTemplate({ name: name.trim(), description: description.trim() || undefined, color })
      .then(() => fetchTemplates());
    setShowCreate(false);
    setName('');
    setDescription('');
  };

  const handleExpandTemplate = async (id: number) => {
    if (expandedTemplateId === id) {
      setExpandedTemplateId(null);
      return;
    }
    setExpandedTemplateId(id);
    await fetchTemplateSprints(id);
  };

  const handleAddSprint = async (templateId: number) => {
    if (sprintAddMode === 'custom') {
      if (!newSprintName.trim()) return;
      await addTemplateSprint(templateId, newSprintName.trim(), newSprintDesc.trim());
    } else {
      if (!selectedSharedSprintId) return;
      const ss = sharedSprints.find((s) => s.id === selectedSharedSprintId);
      if (ss) {
        const detail = await apiGetSharedSprintWithSections(selectedSharedSprintId);
        const sprint = await apiAddTemplateSprint(templateId, ss.name, ss.description);
        for (const dss of detail.sections) {
          await addTemplateSprintSection(sprint.id, dss.section_id, sprintLinkMode, templateId);
        }
      }
    }
    setNewSprintName('');
    setNewSprintDesc('');
    setSelectedSharedSprintId(0);
    setAddingSprintToTemplate(null);
  };

  const handleAddSection = async (sprintId: number, templateId: number) => {
    if (sectionAddMode === 'shared') {
      if (!selectedSectionId) return;
      await addTemplateSprintSection(sprintId, selectedSectionId, sectionLinkMode, templateId);
    } else {
      if (!customSectionName.trim()) return;
      const { apiCreateSharedSection, apiAddSharedSectionItem } = await import('../../lib/api');
      const section = await apiCreateSharedSection({ name: customSectionName.trim(), description: '' });
      for (const item of customSectionItems) {
        if (item.trim()) {
          await apiAddSharedSectionItem({ section_id: section.id, title: item.trim() });
        }
      }
      await addTemplateSprintSection(sprintId, section.id, false, templateId);
      fetchSharedSections();
    }
    setAddingSectionToSprint(null);
    setSelectedSectionId(0);
    setCustomSectionName('');
    setCustomSectionItems([]);
  };

  const sectionMap = new Map(sharedSections.map((s) => [s.id, s]));

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-xl font-semibold">Templates</h2>
        <button
          onClick={() => setShowCreate(true)}
          className="px-4 py-2 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700"
        >
          + New Template
        </button>
      </div>

      {templates.length === 0 ? (
        <div className="text-center py-16">
          <p className="text-gray-400 text-lg mb-4">No templates yet</p>
          <button
            onClick={() => setShowCreate(true)}
            className="px-4 py-2 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700"
          >
            Create your first template
          </button>
        </div>
      ) : (
        <div className="space-y-3">
          {templates.map((template) => {
            const sprints = templateSprints.get(template.id) ?? [];
            const isExpanded = expandedTemplateId === template.id;
            return (
              <div key={template.id} className="bg-white border rounded-xl overflow-hidden">
                <button
                  onClick={() => handleExpandTemplate(template.id)}
                  className="w-full flex items-center justify-between px-4 py-3 hover:bg-gray-50"
                >
                  <div className="flex items-center gap-3">
                    <svg
                      className={`w-4 h-4 text-gray-400 transition-transform ${isExpanded ? 'rotate-90' : ''}`}
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke="currentColor"
                    >
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                    </svg>
                    <span className="w-3 h-3 rounded-full" style={{ backgroundColor: template.color }} />
                    <div className="text-left">
                      <h3 className="font-medium text-sm">{template.name}</h3>
                      {template.description && (
                        <p className="text-xs text-gray-500">{template.description}</p>
                      )}
                    </div>
                  </div>
                  <div className="flex items-center gap-3">
                    <span className="text-xs text-gray-400">{sprints.length} sprints</span>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        if (confirm(`Delete "${template.name}"?`)) deleteTemplate(template.id);
                      }}
                      className="text-gray-400 hover:text-red-500 text-sm"
                    >
                      &times;
                    </button>
                  </div>
                </button>

                {isExpanded && (
                  <div className="border-t px-4 py-3 space-y-3">
                    {sprints.map((sprint) => {
                      const sprintExpanded = expandedSprintId === sprint.sprint.id;
                      return (
                        <div key={sprint.sprint.id} className="border rounded-lg overflow-hidden">
                          <button
                            onClick={() => setExpandedSprintId(sprintExpanded ? null : sprint.sprint.id)}
                            className="w-full flex items-center justify-between px-3 py-2 hover:bg-gray-50"
                          >
                            <div className="flex items-center gap-2">
                              <svg
                                className={`w-3 h-3 text-gray-400 transition-transform ${sprintExpanded ? 'rotate-90' : ''}`}
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                              >
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                              </svg>
                              <span className="text-sm font-medium">
                                Sprint {sprint.sprint.sort_order + 1}: {sprint.sprint.name}
                              </span>
                            </div>
                            <div className="flex items-center gap-2">
                              <span className="text-xs text-gray-400">{sprint.sections.length} sections</span>
                              <button
                                onClick={(e) => {
                                  e.stopPropagation();
                                  if (confirm(`Delete "${sprint.sprint.name}"?`)) {
                                    deleteTemplateSprint(sprint.sprint.id, template.id);
                                  }
                                }}
                                className="text-gray-400 hover:text-red-500 text-xs"
                              >
                                &times;
                              </button>
                            </div>
                          </button>

                          {sprintExpanded && (
                            <div className="border-t px-3 py-2 space-y-2">
                              {sprint.sections.map((ss) => {
                                const section = sectionMap.get(ss.section_id);
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
                                      onClick={() => deleteTemplateSprintSection(ss.id, template.id)}
                                      className="text-gray-400 hover:text-red-500 text-xs"
                                    >
                                      &times;
                                    </button>
                                  </div>
                                );
                              })}

                              {addingSectionToSprint === sprint.sprint.id ? (
                                <div className="space-y-2">
                                  <div className="flex gap-1 bg-gray-100 rounded p-0.5 w-fit">
                                    <button
                                      onClick={() => setSectionAddMode('shared')}
                                      className={`text-xs px-2 py-1 rounded ${sectionAddMode === 'shared' ? 'bg-white shadow-sm' : 'text-gray-500'}`}
                                    >
                                      From Shared
                                    </button>
                                    <button
                                      onClick={() => setSectionAddMode('custom')}
                                      className={`text-xs px-2 py-1 rounded ${sectionAddMode === 'custom' ? 'bg-white shadow-sm' : 'text-gray-500'}`}
                                    >
                                      Custom
                                    </button>
                                  </div>

                                  {sectionAddMode === 'shared' ? (
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
                                          checked={sectionLinkMode}
                                          onChange={(e) => setSectionLinkMode(e.target.checked)}
                                        />
                                        Link
                                      </label>
                                      <button
                                        onClick={() => handleAddSection(sprint.sprint.id, template.id)}
                                        className="text-xs px-2 py-1 bg-indigo-600 text-white rounded hover:bg-indigo-700"
                                      >
                                        Add
                                      </button>
                                      <button
                                        onClick={() => { setAddingSectionToSprint(null); setSelectedSectionId(0); }}
                                        className="text-xs px-2 py-1 border rounded hover:bg-gray-50"
                                      >
                                        Cancel
                                      </button>
                                    </div>
                                  ) : (
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
                                      <div className="flex gap-2">
                                        <button
                                          onClick={() => handleAddSection(sprint.sprint.id, template.id)}
                                          className="text-xs px-2 py-1 bg-indigo-600 text-white rounded hover:bg-indigo-700"
                                        >
                                          Create Section
                                        </button>
                                        <button
                                          onClick={() => { setAddingSectionToSprint(null); setCustomSectionName(''); setCustomSectionItems([]); }}
                                          className="text-xs px-2 py-1 border rounded hover:bg-gray-50"
                                        >
                                          Cancel
                                        </button>
                                      </div>
                                    </div>
                                  )}
                                </div>
                              ) : (
                                <button
                                  onClick={() => setAddingSectionToSprint(sprint.sprint.id)}
                                  className="w-full text-left text-xs text-indigo-600 py-1.5 hover:bg-gray-50 rounded border border-dashed"
                                >
                                  + Add section
                                </button>
                              )}
                            </div>
                          )}
                        </div>
                      );
                    })}

                    {addingSprintToTemplate === template.id ? (
                      <div className="space-y-2">
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
                              value={newSprintName}
                              onChange={(e) => setNewSprintName(e.target.value)}
                              placeholder="Sprint name"
                              className="w-full text-sm border rounded px-3 py-2 focus:outline-none focus:ring-1 focus:ring-indigo-500"
                              autoFocus
                            />
                            <input
                              value={newSprintDesc}
                              onChange={(e) => setNewSprintDesc(e.target.value)}
                              placeholder="Description (optional)"
                              className="w-full text-sm border rounded px-3 py-2 focus:outline-none focus:ring-1 focus:ring-indigo-500"
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
                                checked={sprintLinkMode}
                                onChange={(e) => setSprintLinkMode(e.target.checked)}
                              />
                              Link
                            </label>
                          </div>
                        )}

                        <div className="flex gap-2">
                          <button
                            onClick={() => handleAddSprint(template.id)}
                            className="text-sm px-3 py-1.5 bg-indigo-600 text-white rounded hover:bg-indigo-700"
                          >
                            Add Sprint
                          </button>
                          <button
                            onClick={() => { setAddingSprintToTemplate(null); setNewSprintName(''); setNewSprintDesc(''); setSelectedSharedSprintId(0); }}
                            className="text-sm px-3 py-1.5 border rounded hover:bg-gray-50"
                          >
                            Cancel
                          </button>
                        </div>
                      </div>
                    ) : (
                      <button
                        onClick={() => setAddingSprintToTemplate(template.id)}
                        className="w-full py-3 text-sm text-indigo-600 border-2 border-dashed border-indigo-200 rounded-lg hover:bg-indigo-50 transition-colors"
                      >
                        + Add Sprint
                      </button>
                    )}
                  </div>
                )}
              </div>
            );
          })}
        </div>
      )}

      <Modal open={showCreate} onClose={() => setShowCreate(false)} title="New Template">
        <form onSubmit={handleCreate} className="space-y-4">
          <div>
            <label className="block text-sm font-medium mb-1">Name</label>
            <input
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full border rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
              placeholder="Template name"
              required
              autoFocus
            />
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">Description</label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              className="w-full border rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
              placeholder="Optional"
              rows={2}
            />
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">Color</label>
            <input
              type="color"
              value={color}
              onChange={(e) => setColor(e.target.value)}
              className="w-10 h-8 rounded cursor-pointer"
            />
          </div>
          <div className="flex gap-2 justify-end pt-2">
            <button type="button" onClick={() => setShowCreate(false)} className="px-3 py-1.5 text-sm border rounded-md hover:bg-gray-50">
              Cancel
            </button>
            <button type="submit" className="px-3 py-1.5 text-sm bg-indigo-600 text-white rounded-md hover:bg-indigo-700">
              Create
            </button>
          </div>
        </form>
      </Modal>
    </div>
  );
}
