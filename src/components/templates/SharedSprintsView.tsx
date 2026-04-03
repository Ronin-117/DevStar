import { useEffect, useState } from 'react';
import { useStore } from '../../store';
import { Modal } from '../shared/Modal';
import { SearchInput } from '../shared/SearchInput';
import { MiniSearchInput } from '../shared/MiniSearchInput';
import { apiCreateSharedSection, apiAddSharedSectionItem } from '../../lib/api';

export function SharedSprintsView() {
  const sharedSprints = useStore((s) => s.sharedSprints);
  const sharedSprintDetail = useStore((s) => s.sharedSprintDetail);
  const sharedSectionDetail = useStore((s) => s.sharedSectionDetail);
  const sharedSections = useStore((s) => s.sharedSections);
  const fetchSharedSprints = useStore((s) => s.fetchSharedSprints);
  const fetchSharedSections = useStore((s) => s.fetchSharedSections);
  const createSharedSprint = useStore((s) => s.createSharedSprint);
  const deleteSharedSprint = useStore((s) => s.deleteSharedSprint);
  const addSharedSprintSection = useStore((s) => s.addSharedSprintSection);
  const deleteSharedSprintSection = useStore((s) => s.deleteSharedSprintSection);
  const addSharedSectionItem = useStore((s) => s.addSharedSectionItem);
  const deleteSharedSectionItem = useStore((s) => s.deleteSharedSectionItem);

  const [expandedSprintId, setExpandedSprintId] = useState<number | null>(null);
  const [expandedSectionId, setExpandedSectionId] = useState<number | null>(null);
  const [showCreate, setShowCreate] = useState(false);
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [showAddSection, setShowAddSection] = useState<number | null>(null);
  const [sectionAddMode, setSectionAddMode] = useState<'shared' | 'custom'>('shared');
  const [selectedSectionId, setSelectedSectionId] = useState(0);
  const [isLinked, setIsLinked] = useState(true);

  const [newItemTitle, setNewItemTitle] = useState('');
  const [customSectionName, setCustomSectionName] = useState('');
  const [customSectionItems, setCustomSectionItems] = useState<string[]>([]);
  const [newItemInput, setNewItemInput] = useState('');
  const [search, setSearch] = useState('');
  const [sectionSearch, setSectionSearch] = useState('');

  useEffect(() => {
    fetchSharedSprints();
    fetchSharedSections();
  }, []);

  // Fetch sprint details (section counts) and section details (item counts) on mount
  useEffect(() => {
    if (sharedSprints.length === 0) return;
    (async () => {
      const { apiGetSharedSprintWithSections, apiGetSharedSectionWithItems } = await import('../../lib/api');
      const sprintDetailMap = new Map(useStore.getState().sharedSprintDetail);
      const sectionDetailMap = new Map(useStore.getState().sharedSectionDetail);
      for (const sprint of sharedSprints) {
        if (!sprintDetailMap.has(sprint.id)) {
          try {
            const detail = await apiGetSharedSprintWithSections(sprint.id);
            sprintDetailMap.set(sprint.id, detail);
            // Also fetch item counts for each section in this sprint
            for (const ss of detail.sections) {
              if (!sectionDetailMap.has(ss.section_id)) {
                try {
                  const secDetail = await apiGetSharedSectionWithItems(ss.section_id);
                  sectionDetailMap.set(ss.section_id, secDetail);
                } catch { /* skip */ }
              }
            }
          } catch { /* skip */ }
        }
      }
      useStore.setState({ sharedSprintDetail: sprintDetailMap, sharedSectionDetail: sectionDetailMap });
    })();
  }, [sharedSprints]);

  const handleCreate = (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) return;
    createSharedSprint({ name: name.trim(), description: description.trim() || undefined });
    setShowCreate(false);
    setName('');
    setDescription('');
  };

  const handleExpandSprint = async (id: number) => {
    if (expandedSprintId === id) {
      setExpandedSprintId(null);
      return;
    }
    setExpandedSprintId(id);
    if (!sharedSprintDetail.has(id)) {
      const { apiGetSharedSprintWithSections } = await import('../../lib/api');
      const detail = await apiGetSharedSprintWithSections(id);
      useStore.setState({
        sharedSprintDetail: new Map(useStore.getState().sharedSprintDetail).set(id, detail),
      });
    }
  };

  const handleExpandSection = async (sectionId: number) => {
    if (expandedSectionId === sectionId) {
      setExpandedSectionId(null);
      return;
    }
    setExpandedSectionId(sectionId);
    if (!sharedSectionDetail.has(sectionId)) {
      const { apiGetSharedSectionWithItems } = await import('../../lib/api');
      const detail = await apiGetSharedSectionWithItems(sectionId);
      useStore.setState({
        sharedSectionDetail: new Map(useStore.getState().sharedSectionDetail).set(sectionId, detail),
      });
    }
  };

  const handleAddItem = async (sectionId: number) => {
    if (!newItemTitle.trim()) return;
    addSharedSectionItem(sectionId, newItemTitle.trim());
    setNewItemTitle('');
  };

  const handleAddSection = async (sprintId: number) => {
    if (sectionAddMode === 'shared') {
      if (!selectedSectionId) return;
      addSharedSprintSection(sprintId, selectedSectionId, isLinked);
    } else {
      if (!customSectionName.trim()) return;
      const section = await apiCreateSharedSection({ name: customSectionName.trim(), description: '' });
      for (const item of customSectionItems) {
        if (item.trim()) {
          await apiAddSharedSectionItem({ section_id: section.id, title: item.trim() });
        }
      }
      addSharedSprintSection(sprintId, section.id, false);
      fetchSharedSections();
    }
    setShowAddSection(null);
    setSelectedSectionId(0);
    setCustomSectionName('');
    setCustomSectionItems([]);
  };

  const q = search.toLowerCase();
  const filteredSprints = sharedSprints.filter((s) =>
    s.name.toLowerCase().includes(q) || s.description.toLowerCase().includes(q),
  );

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-xl font-semibold">Shared Sprints</h2>
        <button
          onClick={() => setShowCreate(true)}
          className="px-4 py-2 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700"
        >
          + New Sprint
        </button>
      </div>

      <div className="mb-4">
        <SearchInput value={search} onChange={setSearch} placeholder="Search sprints..." />
      </div>

      {filteredSprints.length === 0 ? (
        <div className="text-center py-16">
          <p className="text-gray-400 text-lg mb-4">
            {sharedSprints.length === 0 ? 'No shared sprints yet' : 'No sprints match your search'}
          </p>
          {sharedSprints.length === 0 && (
            <button
              onClick={() => setShowCreate(true)}
              className="px-4 py-2 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700"
            >
              Create one
            </button>
          )}
        </div>
      ) : (
        <div className="space-y-3">
          {filteredSprints.map((sprint) => {
            const detail = sharedSprintDetail.get(sprint.id);
            return (
              <div key={sprint.id} className="bg-white border rounded-xl overflow-hidden">
                <button
                  onClick={() => handleExpandSprint(sprint.id)}
                  className="w-full flex items-center justify-between px-4 py-3 hover:bg-gray-50"
                >
                  <div className="text-left">
                    <h3 className="font-medium text-sm">{sprint.name}</h3>
                    {sprint.description && (
                      <p className="text-xs text-gray-500">{sprint.description}</p>
                    )}
                  </div>
                  <div className="flex items-center gap-3">
                    <span className="text-xs text-gray-400">
                      {detail ? detail.sections.length : '...'} sections
                    </span>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        deleteSharedSprint(sprint.id);
                      }}
                      className="text-gray-400 hover:text-red-500 text-sm"
                    >
                      &times;
                    </button>
                  </div>
                </button>

                {expandedSprintId === sprint.id && detail && (
                  <div className="border-t px-4 py-3 space-y-2">
                    {detail.sections.map((ss) => {
                      const section = sharedSections.find((s) => s.id === ss.section_id);
                      const sectionDetail = sharedSectionDetail.get(ss.section_id);
                      const sectionExpanded = expandedSectionId === ss.section_id;
                      return (
                        <div key={ss.id} className="border rounded-lg overflow-hidden">
                          <button
                            onClick={() => handleExpandSection(ss.section_id)}
                            className="w-full flex items-center justify-between px-3 py-2 hover:bg-gray-50"
                          >
                            <div className="flex items-center gap-2">
                              <svg
                                className={`w-3 h-3 text-gray-400 transition-transform ${sectionExpanded ? 'rotate-90' : ''}`}
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                              >
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                              </svg>
                              {section && (
                                <span className="w-2 h-2 rounded-full" style={{ backgroundColor: section.color }} />
                              )}
                              <span className="text-sm">{section?.name ?? 'Unknown'}</span>
                              {ss.is_linked && (
                                <span className="text-xs px-1.5 py-0.5 rounded bg-blue-50 text-blue-600">linked</span>
                              )}
                            </div>
                            <div className="flex items-center gap-2">
                              <span className="text-xs text-gray-400">
                                {sectionDetail ? sectionDetail.items.length : '...'} items
                              </span>
                              <button
                                onClick={(e) => {
                                  e.stopPropagation();
                                  deleteSharedSprintSection(sprint.id, ss.section_id);
                                }}
                                className="text-gray-400 hover:text-red-500 text-xs"
                              >
                                &times;
                              </button>
                            </div>
                          </button>

                          {sectionExpanded && sectionDetail && (
                            <div className="border-t px-3 py-2 space-y-1.5 bg-gray-50">
                              {sectionDetail.items.map((item) => (
                                <div
                                  key={item.id}
                                  className="flex items-center justify-between px-3 py-1.5 bg-white rounded group"
                                >
                                  <div>
                                    <p className="text-sm">{item.title}</p>
                                    {item.description && (
                                      <p className="text-xs text-gray-500">{item.description}</p>
                                    )}
                                  </div>
                                  <button
                                    onClick={() => deleteSharedSectionItem(item.id, ss.section_id)}
                                    className="opacity-0 group-hover:opacity-100 text-gray-400 hover:text-red-500 text-xs"
                                  >
                                    &times;
                                  </button>
                                </div>
                              ))}
                              <div className="flex gap-2 pt-1">
                                <input
                                  value={newItemTitle}
                                  onChange={(e) => setNewItemTitle(e.target.value)}
                                  onKeyDown={(e) => e.key === 'Enter' && handleAddItem(ss.section_id)}
                                  placeholder="Add item..."
                                  className="flex-1 text-sm border rounded px-2 py-1 focus:outline-none focus:ring-1 focus:ring-indigo-500"
                                />
                                <button
                                  onClick={() => handleAddItem(ss.section_id)}
                                  className="text-xs px-2 py-1 bg-indigo-600 text-white rounded hover:bg-indigo-700"
                                >
                                  Add
                                </button>
                              </div>
                            </div>
                          )}
                        </div>
                      );
                    })}
                    {showAddSection === sprint.id ? (
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
                          <div className="space-y-2">
                            <MiniSearchInput value={sectionSearch} onChange={setSectionSearch} placeholder="Search sections..." />
                            <div className="flex gap-2 flex-wrap">
                              <select
                                value={selectedSectionId}
                                onChange={(e) => setSelectedSectionId(Number(e.target.value))}
                                className="flex-1 text-sm border rounded px-2 py-1"
                              >
                                <option value={0}>Select section</option>
                                {sharedSections
                                  .filter((s) => s.name.toLowerCase().includes(sectionSearch.toLowerCase()))
                                  .map((s) => (
                                    <option key={s.id} value={s.id}>{s.name}</option>
                                  ))}
                            </select>
                            <label className="flex items-center gap-1 text-xs">
                              <input
                                type="checkbox"
                                checked={isLinked}
                                onChange={(e) => setIsLinked(e.target.checked)}
                              />
                              Link
                            </label>
                            <button
                              onClick={() => handleAddSection(sprint.id)}
                              className="text-xs px-2 py-1 bg-indigo-600 text-white rounded hover:bg-indigo-700"
                            >
                              Add
                            </button>
                            <button
                              onClick={() => { setShowAddSection(null); setSelectedSectionId(0); }}
                              className="text-xs px-2 py-1 border rounded hover:bg-gray-50"
                            >
                              Cancel
                            </button>
                          </div>
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
                                onClick={() => handleAddSection(sprint.id)}
                                className="text-xs px-2 py-1 bg-indigo-600 text-white rounded hover:bg-indigo-700"
                              >
                                Create Section
                              </button>
                              <button
                                onClick={() => {
                                  setShowAddSection(null);
                                  setCustomSectionName('');
                                  setCustomSectionItems([]);
                                }}
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
                        onClick={() => setShowAddSection(sprint.id)}
                        className="w-full text-left text-sm text-indigo-600 py-2 hover:bg-gray-50 rounded border border-dashed"
                      >
                        + Add section
                      </button>
                    )}
                  </div>
                )}
              </div>
            );
          })}
        </div>
      )}

      <Modal open={showCreate} onClose={() => setShowCreate(false)} title="New Shared Sprint">
        <form onSubmit={handleCreate} className="space-y-4">
          <div>
            <label className="block text-sm font-medium mb-1">Name</label>
            <input
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full border rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
              placeholder="Sprint name"
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
