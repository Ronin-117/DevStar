import { useEffect, useState } from 'react';
import { useStore } from '../../store';
import { apiListTemplateSectionsWithItems, apiAddTemplateSection, apiAddTemplateItem, apiUpdateTemplateItem, apiDeleteTemplateItem, apiDeleteTemplateSection, apiListTemplates } from '../../lib/api';
import type { Template, SectionWithItems } from '../../lib/types';

export function TemplateEditorView() {
  const selectedTemplateId = useStore((s) => s.selectedTemplateId);
  const setView = useStore((s) => s.setView);
  const fetchTemplates = useStore((s) => s.fetchTemplates);

  const [sections, setSections] = useState<SectionWithItems[]>([]);
  const [selectedSectionId, setSelectedSectionId] = useState<number | null>(null);
  const [editingItem, setEditingItem] = useState<number | null>(null);
  const [editingItemTitle, setEditingItemTitle] = useState('');
  const [editingItemDesc, setEditingItemDesc] = useState('');
  const [newItemTitle, setNewItemTitle] = useState('');
  const [newSectionName, setNewSectionName] = useState('');
  const [showAddSection, setShowAddSection] = useState(false);
  const [showInherit, setShowInherit] = useState(false);
  const [inheritSource, setInheritSource] = useState<number | null>(null);
  const [inheritSections, setInheritSections] = useState<SectionWithItems[]>([]);
  const [allTemplates, setAllTemplates] = useState<Template[]>([]);
  const [templateName, setTemplateName] = useState('');
  const [templateDesc, setTemplateDesc] = useState('');
  const [editingTemplateName, setEditingTemplateName] = useState(false);

  const currentTemplate = allTemplates.find((t) => t.id === selectedTemplateId);
  const selectedSection = sections.find((s) => s.section.id === selectedSectionId);

  useEffect(() => {
    apiListTemplates().then(setAllTemplates).catch(console.error);
  }, []);

  useEffect(() => {
    if (selectedTemplateId) {
      apiListTemplateSectionsWithItems(selectedTemplateId)
        .then(setSections)
        .catch(console.error);
      const t = allTemplates.find((t) => t.id === selectedTemplateId);
      if (t) {
        setTemplateName(t.name);
        setTemplateDesc(t.description);
      }
    }
  }, [selectedTemplateId]);

  const refreshSections = () => {
    if (selectedTemplateId) {
      apiListTemplateSectionsWithItems(selectedTemplateId)
        .then(setSections)
        .catch(console.error);
    }
  };

  const handleAddItem = async () => {
    if (!newItemTitle.trim() || !selectedSectionId) return;
    await apiAddTemplateItem({ section_id: selectedSectionId, title: newItemTitle.trim() });
    setNewItemTitle('');
    refreshSections();
  };

  const handleSaveItem = async (itemId: number) => {
    await apiUpdateTemplateItem(itemId, editingItemTitle, editingItemDesc);
    setEditingItem(null);
    refreshSections();
  };

  const handleDeleteItem = async (itemId: number) => {
    await apiDeleteTemplateItem(itemId);
    if (editingItem === itemId) setEditingItem(null);
    refreshSections();
  };

  const handleAddSection = async () => {
    if (!newSectionName.trim() || !selectedTemplateId) return;
    await apiAddTemplateSection({ template_id: selectedTemplateId, name: newSectionName.trim() });
    setNewSectionName('');
    setShowAddSection(false);
    refreshSections();
  };

  const handleInherit = async () => {
    if (!inheritSource || !selectedTemplateId) return;
    const sourceSections = await apiListTemplateSectionsWithItems(inheritSource);
    setInheritSections(sourceSections);
  };

  const handleCopySection = async (section: SectionWithItems) => {
    if (!selectedTemplateId) return;
    const newSection = await apiAddTemplateSection({
      template_id: selectedTemplateId,
      name: section.section.name,
      description: section.section.description,
    });
    for (const item of section.items) {
      await apiAddTemplateItem({ section_id: newSection.id, title: item.title, description: item.description });
    }
    setShowInherit(false);
    setInheritSections([]);
    setInheritSource(null);
    refreshSections();
  };

  const handleLinkSection = async (section: SectionWithItems) => {
    if (!selectedTemplateId) return;
    const sourceSectionId = section.section.linked_from_section_id ?? section.section.id;
    await apiAddTemplateSection({
      template_id: selectedTemplateId,
      name: section.section.name,
      description: section.section.description,
      linked_from_section_id: sourceSectionId,
    });
    setShowInherit(false);
    setInheritSections([]);
    setInheritSource(null);
    refreshSections();
  };

  const handleDeleteSection = async (sectionId: number) => {
    await apiDeleteTemplateSection(sectionId);
    if (selectedSectionId === sectionId) setSelectedSectionId(null);
    refreshSections();
  };

  const handleSaveTemplateName = async () => {
    if (!selectedTemplateId) return;
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('update_template', { id: selectedTemplateId, name: templateName, description: templateDesc });
      setEditingTemplateName(false);
      fetchTemplates();
    } catch (e) {
      console.error(e);
    }
  };

  if (!currentTemplate) return null;

  return (
    <div className="flex h-[calc(100vh-80px)]">
      {/* Left sidebar: sections */}
      <div className="w-64 border-r bg-gray-50 flex flex-col">
        <div className="p-3 border-b">
          <button
            onClick={() => setView('templates')}
            className="text-sm text-gray-500 hover:text-gray-700 mb-2 flex items-center gap-1"
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
            </svg>
            Back
          </button>
          {editingTemplateName ? (
            <div className="space-y-2">
              <input
                value={templateName}
                onChange={(e) => setTemplateName(e.target.value)}
                className="w-full text-sm font-medium border rounded px-2 py-1"
                autoFocus
              />
              <input
                value={templateDesc}
                onChange={(e) => setTemplateDesc(e.target.value)}
                className="w-full text-xs border rounded px-2 py-1"
                placeholder="Description"
              />
              <button onClick={handleSaveTemplateName} className="text-xs text-indigo-600">Save</button>
            </div>
          ) : (
            <div onClick={() => setEditingTemplateName(true)} className="cursor-pointer">
              <h3 className="font-medium text-sm">{templateName}</h3>
              <p className="text-xs text-gray-500 mt-0.5">{templateDesc}</p>
            </div>
          )}
        </div>

        <div className="flex-1 overflow-y-auto p-2 space-y-1">
          {sections.map((s) => (
            <div
              key={s.section.id}
              className={`flex items-center gap-2 px-3 py-2 rounded-md cursor-pointer text-sm group ${
                selectedSectionId === s.section.id
                  ? 'bg-indigo-50 text-indigo-700'
                  : 'hover:bg-gray-100'
              }`}
              onClick={() => setSelectedSectionId(s.section.id)}
            >
              <span className="flex-1 truncate">{s.section.name}</span>
              <span className="text-xs text-gray-400">{s.items.length}</span>
              {s.section.linked_from_section_id && (
                <span className="text-xs px-1 py-0.5 rounded bg-blue-50 text-blue-600">🔗</span>
              )}
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  handleDeleteSection(s.section.id);
                }}
                className="opacity-0 group-hover:opacity-100 text-gray-400 hover:text-red-500 text-xs"
              >
                ×
              </button>
            </div>
          ))}
        </div>

        <div className="p-2 border-t space-y-1">
          {showAddSection ? (
            <div className="space-y-1">
              <input
                value={newSectionName}
                onChange={(e) => setNewSectionName(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleAddSection()}
                placeholder="Section name..."
                className="w-full text-xs border rounded px-2 py-1"
                autoFocus
              />
              <div className="flex gap-1">
                <button onClick={handleAddSection} className="text-xs text-indigo-600">Add</button>
                <button onClick={() => setShowAddSection(false)} className="text-xs text-gray-500">Cancel</button>
              </div>
            </div>
          ) : (
            <button
              onClick={() => setShowAddSection(true)}
              className="w-full text-left text-xs text-indigo-600 px-2 py-1.5 hover:bg-gray-100 rounded"
            >
              + Add section
            </button>
          )}
          <button
            onClick={() => setShowInherit(!showInherit)}
            className="w-full text-left text-xs text-gray-600 px-2 py-1.5 hover:bg-gray-100 rounded"
          >
            + Add from template...
          </button>
        </div>
      </div>

      {/* Main area: items */}
      <div className="flex-1 overflow-y-auto p-6">
        {showInherit && (
          <div className="mb-6 p-4 border rounded-lg bg-gray-50">
            <h3 className="text-sm font-medium mb-3">Add section from another template</h3>
            <div className="flex gap-2 mb-3">
              <select
                value={inheritSource ?? ''}
                onChange={(e) => setInheritSource(Number(e.target.value))}
                className="text-sm border rounded-md px-3 py-1.5"
              >
                <option value="">Select template</option>
                {allTemplates
                  .filter((t) => t.id !== selectedTemplateId)
                  .map((t) => (
                    <option key={t.id} value={t.id}>{t.name}</option>
                  ))}
              </select>
              <button
                onClick={handleInherit}
                className="text-sm px-3 py-1.5 bg-indigo-600 text-white rounded-md hover:bg-indigo-700"
              >
                Load
              </button>
              <button
                onClick={() => {
                  setShowInherit(false);
                  setInheritSections([]);
                  setInheritSource(null);
                }}
                className="text-sm px-3 py-1.5 border rounded-md hover:bg-gray-50"
              >
                Cancel
              </button>
            </div>
            {inheritSections.length > 0 && (
              <div className="space-y-2">
                {inheritSections.map((s) => (
                  <div
                    key={s.section.id}
                    className="flex items-center justify-between px-3 py-2 bg-white border rounded-md"
                  >
                    <div>
                      <span className="text-sm font-medium">{s.section.name}</span>
                      <span className="text-xs text-gray-500 ml-2">{s.items.length} items</span>
                    </div>
                    <div className="flex gap-2">
                      <button
                        onClick={() => handleCopySection(s)}
                        className="text-xs px-2 py-1 bg-indigo-600 text-white rounded hover:bg-indigo-700"
                        title="Copy (independent)"
                      >
                        Copy
                      </button>
                      <button
                        onClick={() => handleLinkSection(s)}
                        className="text-xs px-2 py-1 bg-emerald-600 text-white rounded hover:bg-emerald-700"
                        title="Link (updates with source)"
                      >
                        Link
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}

        {selectedSection ? (
          <div>
            <h2 className="text-lg font-semibold mb-1">{selectedSection.section.name}</h2>
            {selectedSection.section.description && (
              <p className="text-sm text-gray-500 mb-2">{selectedSection.section.description}</p>
            )}
            {selectedSection.section.linked_from_section_id && (
              <span className="text-xs px-2 py-0.5 rounded-full bg-emerald-50 text-emerald-600 mb-4 inline-block border border-emerald-200">
                🔗 Linked — changes to source will reflect here
              </span>
            )}

            <div className="space-y-1 mt-4">
              {selectedSection.items.map((item) => (
                <div
                  key={item.id}
                  className="flex items-start gap-3 p-3 rounded-lg hover:bg-gray-50 group border"
                >
                  {editingItem === item.id ? (
                    <div className="flex-1 space-y-2">
                      <input
                        value={editingItemTitle}
                        onChange={(e) => setEditingItemTitle(e.target.value)}
                        className="w-full text-sm border rounded px-2 py-1"
                        autoFocus
                      />
                      <input
                        value={editingItemDesc}
                        onChange={(e) => setEditingItemDesc(e.target.value)}
                        className="w-full text-xs border rounded px-2 py-1 text-gray-500"
                        placeholder="Description (optional)"
                      />
                      <div className="flex gap-2">
                        <button
                          onClick={() => handleSaveItem(item.id)}
                          className="text-xs px-2 py-1 bg-indigo-600 text-white rounded"
                        >
                          Save
                        </button>
                        <button
                          onClick={() => setEditingItem(null)}
                          className="text-xs px-2 py-1 border rounded"
                        >
                          Cancel
                        </button>
                      </div>
                    </div>
                  ) : (
                    <>
                      <div className="flex-1">
                        <p className="text-sm">{item.title}</p>
                        {item.description && (
                          <p className="text-xs text-gray-500 mt-0.5">{item.description}</p>
                        )}
                      </div>
                      <div className="flex gap-1 opacity-0 group-hover:opacity-100">
                        <button
                          onClick={() => {
                            setEditingItem(item.id);
                            setEditingItemTitle(item.title);
                            setEditingItemDesc(item.description);
                          }}
                          className="text-xs text-gray-400 hover:text-gray-600 px-1"
                        >
                          edit
                        </button>
                        <button
                          onClick={() => handleDeleteItem(item.id)}
                          className="text-xs text-gray-400 hover:text-red-500 px-1"
                        >
                          ×
                        </button>
                      </div>
                    </>
                  )}
                </div>
              ))}
            </div>

            <div className="mt-4 flex gap-2">
              <input
                value={newItemTitle}
                onChange={(e) => setNewItemTitle(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleAddItem()}
                placeholder="New item..."
                className="flex-1 text-sm border rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
              <button
                onClick={handleAddItem}
                className="px-3 py-2 text-sm bg-indigo-600 text-white rounded-md hover:bg-indigo-700"
              >
                Add
              </button>
            </div>
          </div>
        ) : (
          <div className="text-center py-16 text-gray-400">
            <p className="text-lg mb-2">Select a section</p>
            <p className="text-sm">Choose a section from the sidebar to edit its items</p>
          </div>
        )}
      </div>
    </div>
  );
}
