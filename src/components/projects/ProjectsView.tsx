import { useState } from 'react';
import { useStore } from '../../store';
import type { Template } from '../../lib/types';

export function CreateProjectModal({ onClose }: { onClose: () => void }) {
  const templates = useStore((s) => s.templates);
  const createProject = useStore((s) => s.createProject);
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [templateId, setTemplateId] = useState(0);
  const [color, setColor] = useState('#6366f1');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim() || !templateId) return;
    createProject({ name: name.trim(), description: description.trim() || undefined, template_id: templateId, color });
    onClose();
  };

  return (
    <div className="fixed inset-0 bg-black/40 flex items-center justify-center z-50">
      <form onSubmit={handleSubmit} className="bg-white rounded-xl p-6 w-full max-w-md space-y-4">
        <h2 className="text-lg font-semibold">New Project</h2>
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
            {templates
              .filter((t) => !t.name.startsWith('Shared:'))
              .map((t) => (
                <option key={t.id} value={t.id}>
                  {t.name}
                </option>
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

export function ProjectCard({ project, templateName, checked, total, onClick }: {
  project: { id: number; name: string; description: string; color: string };
  templateName: string;
  checked: number;
  total: number;
  onClick: () => void;
}) {
  const deleteProject = useStore((s) => s.deleteProject);
  const pct = total > 0 ? Math.round((checked / total) * 100) : 0;
  const done = checked === total && total > 0;

  return (
    <div
      onClick={onClick}
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
          ×
        </button>
      </div>
      <p className="text-xs text-gray-500 mb-3">{templateName}</p>
      <div className="w-full bg-gray-200 rounded-full h-2 overflow-hidden">
        <div
          className="h-2 rounded-full transition-all duration-300"
          style={{ width: `${pct}%`, backgroundColor: project.color }}
        />
      </div>
      <div className="flex items-center justify-between mt-2">
        <span className="text-xs text-gray-500">{checked}/{total}</span>
        <span
          className={`text-xs px-2 py-0.5 rounded-full ${
            done ? 'bg-green-100 text-green-700' : 'bg-amber-100 text-amber-700'
          }`}
        >
          {done ? 'done' : 'pending'}
        </span>
      </div>
    </div>
  );
}

export function ProjectsView() {
  const projects = useStore((s) => s.projects);
  const templates = useStore((s) => s.templates);
  const projectProgressMap = useStore((s) => s.projectProgressMap);
  const setSelectedProjectId = useStore((s) => s.setSelectedProjectId);
  const setEditingProjectId = useStore((s) => s.setEditingProjectId);
  const [showCreate, setShowCreate] = useState(false);

  const templateMap = new Map<number, Template>();
  templates.forEach((t) => templateMap.set(t.id, t));

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
              <ProjectCard
                key={project.id}
                project={project}
                templateName={template?.name ?? 'Unknown template'}
                checked={checked}
                total={total}
                onClick={() => {
                  setSelectedProjectId(project.id);
                  setEditingProjectId(project.id);
                  localStorage.setItem('pt_active_project_id', String(project.id));
                }}
              />
            );
          })}
        </div>
      )}

      {showCreate && <CreateProjectModal onClose={() => setShowCreate(false)} />}
    </div>
  );
}
