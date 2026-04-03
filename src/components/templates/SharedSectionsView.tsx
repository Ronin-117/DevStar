import { useEffect, useState } from 'react';
import { useStore } from '../../store';
import { Modal } from '../shared/Modal';
import { SearchInput } from '../shared/SearchInput';

export function SharedSectionsView() {
  const sharedSections = useStore((s) => s.sharedSections);
  const sharedSectionDetail = useStore((s) => s.sharedSectionDetail);
  const fetchSharedSections = useStore((s) => s.fetchSharedSections);
  const createSharedSection = useStore((s) => s.createSharedSection);
  const deleteSharedSection = useStore((s) => s.deleteSharedSection);
  const addSharedSectionItem = useStore((s) => s.addSharedSectionItem);
  const deleteSharedSectionItem = useStore((s) => s.deleteSharedSectionItem);

  const [expandedId, setExpandedId] = useState<number | null>(null);
  const [showCreate, setShowCreate] = useState(false);
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [color, setColor] = useState('#6b7280');
  const [newItemTitle, setNewItemTitle] = useState('');
  const [search, setSearch] = useState('');

  useEffect(() => {
    fetchSharedSections();
  }, []);

  // Fetch item counts for all sections on mount
  useEffect(() => {
    if (sharedSections.length === 0) return;
    (async () => {
      const { apiGetSharedSectionWithItems } = await import('../../lib/api');
      const detailMap = new Map(useStore.getState().sharedSectionDetail);
      for (const section of sharedSections) {
        if (!detailMap.has(section.id)) {
          try {
            const detail = await apiGetSharedSectionWithItems(section.id);
            detailMap.set(section.id, detail);
          } catch { /* skip */ }
        }
      }
      useStore.setState({ sharedSectionDetail: detailMap });
    })();
  }, [sharedSections]);

  const handleCreate = (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) return;
    createSharedSection({ name: name.trim(), description: description.trim() || undefined, color });
    setShowCreate(false);
    setName('');
    setDescription('');
  };

  const handleExpand = async (id: number) => {
    if (expandedId === id) {
      setExpandedId(null);
      return;
    }
    setExpandedId(id);
    if (!sharedSectionDetail.has(id)) {
      const { apiGetSharedSectionWithItems } = await import('../../lib/api');
      const detail = await apiGetSharedSectionWithItems(id);
      useStore.setState({
        sharedSectionDetail: new Map(useStore.getState().sharedSectionDetail).set(id, detail),
      });
    }
  };

  const handleAddItem = async (sectionId: number) => {
    if (!newItemTitle.trim()) return;
    addSharedSectionItem(sectionId, newItemTitle.trim());
    setNewItemTitle('');
  };

  const q = search.toLowerCase();
  const filteredSections = sharedSections.filter((s) =>
    s.name.toLowerCase().includes(q) || s.description.toLowerCase().includes(q),
  );

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-xl font-semibold">Shared Sections</h2>
        <button
          onClick={() => setShowCreate(true)}
          className="px-4 py-2 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700"
        >
          + New Section
        </button>
      </div>

      <div className="mb-4">
        <SearchInput value={search} onChange={setSearch} placeholder="Search sections..." />
      </div>

      {filteredSections.length === 0 ? (
        <div className="text-center py-16">
          <p className="text-gray-400 text-lg mb-4">
            {sharedSections.length === 0 ? 'No shared sections yet' : 'No sections match your search'}
          </p>
          {sharedSections.length === 0 && (
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
          {filteredSections.map((section) => {
            const detail = sharedSectionDetail.get(section.id);
            return (
              <div key={section.id} className="bg-white border rounded-xl overflow-hidden">
                <button
                  onClick={() => handleExpand(section.id)}
                  className="w-full flex items-center justify-between px-4 py-3 hover:bg-gray-50"
                >
                  <div className="flex items-center gap-3">
                    <span className="w-3 h-3 rounded-full" style={{ backgroundColor: section.color }} />
                    <div className="text-left">
                      <h3 className="font-medium text-sm">{section.name}</h3>
                      {section.description && (
                        <p className="text-xs text-gray-500">{section.description}</p>
                      )}
                    </div>
                  </div>
                  <div className="flex items-center gap-3">
                    <span className="text-xs text-gray-400">
                      {detail ? detail.items.length : '...'} items
                    </span>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        deleteSharedSection(section.id);
                      }}
                      className="text-gray-400 hover:text-red-500 text-sm"
                    >
                      &times;
                    </button>
                  </div>
                </button>

                {expandedId === section.id && detail && (
                  <div className="border-t px-4 py-3 space-y-2">
                    {detail.items.map((item) => (
                      <div
                        key={item.id}
                        className="flex items-center justify-between px-3 py-2 bg-gray-50 rounded group"
                      >
                        <div>
                          <p className="text-sm">{item.title}</p>
                          {item.description && (
                            <p className="text-xs text-gray-500">{item.description}</p>
                          )}
                        </div>
                        <button
                          onClick={() => deleteSharedSectionItem(item.id, section.id)}
                          className="opacity-0 group-hover:opacity-100 text-gray-400 hover:text-red-500 text-xs"
                        >
                          &times;
                        </button>
                      </div>
                    ))}
                    <div className="flex gap-2">
                      <input
                        value={newItemTitle}
                        onChange={(e) => setNewItemTitle(e.target.value)}
                        onKeyDown={(e) => e.key === 'Enter' && handleAddItem(section.id)}
                        placeholder="Add item..."
                        className="flex-1 text-sm border rounded px-2 py-1 focus:outline-none focus:ring-1 focus:ring-indigo-500"
                      />
                      <button
                        onClick={() => handleAddItem(section.id)}
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
        </div>
      )}

      <Modal open={showCreate} onClose={() => setShowCreate(false)} title="New Shared Section">
        <form onSubmit={handleCreate} className="space-y-4">
          <div>
            <label className="block text-sm font-medium mb-1">Name</label>
            <input
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full border rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
              placeholder="Section name"
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
