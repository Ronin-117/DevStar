import { useState } from 'react';
import { useStore } from '../../store';
import type { Template } from '../../lib/types';

export function TemplateCard({ template, sectionCount, itemCount, onClick }: {
  template: Template;
  sectionCount: number;
  itemCount: number;
  onClick: () => void;
}) {
  const deleteTemplate = useStore((s) => s.deleteTemplate);
  const isShared = template.name.startsWith('Shared:');

  return (
    <div
      onClick={onClick}
      className="bg-white border rounded-xl p-4 cursor-pointer hover:shadow-md transition-all group relative"
    >
      {isShared && (
        <span className="absolute top-2 right-2 text-xs px-2 py-0.5 rounded-full bg-gray-100 text-gray-600">
          shared
        </span>
      )}
      <div className="flex items-center gap-2 mb-2">
        <span className="w-4 h-4 rounded" style={{ backgroundColor: template.color }} />
        <h3 className="font-medium">{template.name}</h3>
      </div>
      <p className="text-xs text-gray-500 mb-3 line-clamp-2">{template.description}</p>
      <div className="flex items-center gap-3 text-xs text-gray-400">
        <span>{sectionCount} sections</span>
        <span>{itemCount} items</span>
      </div>
      <button
        onClick={(e) => {
          e.stopPropagation();
          if (confirm(`Delete "${template.name}"?`)) {
            deleteTemplate(template.id);
          }
        }}
        className="absolute bottom-2 right-2 opacity-0 group-hover:opacity-100 text-gray-400 hover:text-red-500 text-sm"
      >
        ×
      </button>
    </div>
  );
}

export function CreateTemplateModal({ onClose, isShared }: { onClose: () => void; isShared?: boolean }) {
  const fetchTemplates = useStore((s) => s.fetchTemplates);
  const [name, setName] = useState(isShared ? 'Shared: ' : '');
  const [description, setDescription] = useState('');
  const [color, setColor] = useState('#6366f1');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) return;
    const { invoke } = await import('@tauri-apps/api/core');
    try {
      await invoke('create_template', { input: { name: name.trim(), description: description.trim() || '', color } });
      fetchTemplates();
      onClose();
    } catch (err: unknown) {
      console.error(err);
    }
  };

  return (
    <div className="fixed inset-0 bg-black/40 flex items-center justify-center z-50">
      <form onSubmit={handleSubmit} className="bg-white rounded-xl p-6 w-full max-w-md space-y-4">
        <h2 className="text-lg font-semibold">{isShared ? 'New Shared Section' : 'New Template'}</h2>
        <div>
          <label className="block text-sm font-medium mb-1">Name</label>
          <input
            value={name}
            onChange={(e) => setName(e.target.value)}
            className="w-full border rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
            placeholder={isShared ? 'Shared: Section name' : 'Template name'}
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
            placeholder="What is this for?"
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
          <button type="button" onClick={onClose} className="px-3 py-1.5 text-sm border rounded-md hover:bg-gray-50">
            Cancel
          </button>
          <button type="submit" className="px-3 py-1.5 text-sm bg-indigo-600 text-white rounded-md hover:bg-indigo-700">
            Create
          </button>
        </div>
      </form>
    </div>
  );
}

export function TemplatesView() {
  const templates = useStore((s) => s.templates);
  const templateCounts = useStore((s) => s.templateCounts);
  const setSelectedTemplateId = useStore((s) => s.setSelectedTemplateId);
  const setView = useStore((s) => s.setView);
  const [showCreate, setShowCreate] = useState(false);
  const [showCreateShared, setShowCreateShared] = useState(false);

  const realTemplates = templates.filter((t) => !t.name.startsWith('Shared:'));
  const sharedTemplates = templates.filter((t) => t.name.startsWith('Shared:'));

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-xl font-semibold">Templates</h2>
        <div className="flex gap-2">
          <button
            onClick={() => setShowCreateShared(true)}
            className="px-4 py-2 text-sm border rounded-lg hover:bg-gray-50 transition-colors"
          >
            + Shared Section
          </button>
          <button
            onClick={() => setShowCreate(true)}
            className="px-4 py-2 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
          >
            + New Template
          </button>
        </div>
      </div>

      <h3 className="text-sm font-medium text-gray-500 mb-3">Project Templates</h3>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mb-8">
        {realTemplates.map((t) => {
          const counts = templateCounts.get(t.id) ?? { sections: 0, items: 0 };
          return (
            <TemplateCard
              key={t.id}
              template={t}
              sectionCount={counts.sections}
              itemCount={counts.items}
              onClick={() => {
                setSelectedTemplateId(t.id);
                setView('template-editor' as any);
              }}
            />
          );
        })}
      </div>

      <h3 className="text-sm font-medium text-gray-500 mb-3">Shared Sections</h3>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {sharedTemplates.map((t) => {
          const counts = templateCounts.get(t.id) ?? { sections: 0, items: 0 };
          return (
            <TemplateCard
              key={t.id}
              template={t}
              sectionCount={counts.sections}
              itemCount={counts.items}
              onClick={() => {
                setSelectedTemplateId(t.id);
                setView('template-editor' as any);
              }}
            />
          );
        })}
        {sharedTemplates.length === 0 && (
          <div className="col-span-full text-center py-8 text-gray-400">
            <p className="text-sm">No shared sections yet</p>
            <button
              onClick={() => setShowCreateShared(true)}
              className="mt-2 text-sm text-indigo-600 hover:text-indigo-800"
            >
              Create one
            </button>
          </div>
        )}
      </div>

      {showCreate && <CreateTemplateModal onClose={() => setShowCreate(false)} />}
      {showCreateShared && <CreateTemplateModal onClose={() => setShowCreateShared(false)} isShared />}
    </div>
  );
}
