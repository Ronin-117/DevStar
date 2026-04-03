import { useState } from 'react';
import { useStore } from '../../store';
import { Modal } from '../shared/Modal';
import { ProgressBar } from '../shared/ProgressBar';
import type { Template } from '../../lib/types';

export function ProjectsView() {
  const projects = useStore((s) => s.projects);
  const templates = useStore((s) => s.templates);
  const projectProgressMap = useStore((s) => s.projectProgressMap);
  const setSelectedProjectId = useStore((s) => s.setSelectedProjectId);
  const setEditingProjectId = useStore((s) => s.setEditingProjectId);
  const deleteProject = useStore((s) => s.deleteProject);
  const createProject = useStore((s) => s.createProject);
  const [showCreate, setShowCreate] = useState(false);
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [templateId, setTemplateId] = useState(0);
  const [color, setColor] = useState('#6366f1');

  const templateMap = new Map<number, Template>();
  templates.forEach((t) => templateMap.set(t.id, t));

  const handleCreate = (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim() || !templateId) return;
    createProject({ name: name.trim(), description: description.trim() || undefined, template_id: templateId, color });
    setShowCreate(false);
    setName('');
    setDescription('');
    setTemplateId(0);
  };

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-xl font-semibold">Projects</h2>
        <button
          onClick={() => setShowCreate(true)}
          className="px-4 py-2 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
        >
          + New Project
        </button>
      </div>

      {projects.length === 0 ? (
        <div className="text-center py-16">
          <p className="text-gray-400 text-lg mb-4">No projects yet</p>
          <button
            onClick={() => setShowCreate(true)}
            className="px-4 py-2 text-sm bg-indigo-600 text-white rounded-lg hover:bg-indigo-700"
          >
            Create your first project
          </button>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {projects.map((project) => {
            const template = templateMap.get(project.template_id);
            const [checked, total] = projectProgressMap.get(project.id) ?? [0, 0];
            return (
              <div
                key={project.id}
                onClick={() => {
                  setSelectedProjectId(project.id);
                  setEditingProjectId(project.id);
                  localStorage.setItem('pt_active_project_id', String(project.id));
                }}
                className="bg-white border rounded-xl p-4 cursor-pointer hover:shadow-md transition-all group"
              >
                <div className="flex items-start justify-between mb-3">
                  <div className="flex items-center gap-2">
                    <span className="w-3 h-3 rounded-full" style={{ backgroundColor: project.color }} />
                    <h3 className="font-medium">{project.name}</h3>
                  </div>
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      deleteProject(project.id);
                    }}
                    className="opacity-0 group-hover:opacity-100 text-gray-400 hover:text-red-500 text-sm"
                  >
                    &times;
                  </button>
                </div>
                <p className="text-xs text-gray-500 mb-3">{template?.name ?? 'Unknown template'}</p>
                <ProgressBar checked={checked} total={total} size="sm" color="bg-indigo-600" />
              </div>
            );
          })}
        </div>
      )}

      <Modal open={showCreate} onClose={() => setShowCreate(false)} title="New Project">
        <form onSubmit={handleCreate} className="space-y-4">
          <div>
            <label className="block text-sm font-medium mb-1">Name</label>
            <input
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full border rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
              placeholder="Project name"
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
            <label className="block text-sm font-medium mb-1">Template</label>
            <select
              value={templateId}
              onChange={(e) => setTemplateId(Number(e.target.value))}
              className="w-full border rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
              required
            >
              <option value={0}>Select a template</option>
              {templates.map((t) => (
                <option key={t.id} value={t.id}>{t.name}</option>
              ))}
            </select>
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
