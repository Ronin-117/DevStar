import { create } from 'zustand';
import * as api from '../lib/api';
import type { Template, SectionWithItems, Project, ProjectSectionWithItems } from '../lib/types';

interface AppState {
  // Data
  templates: Template[];
  templateSections: SectionWithItems[];
  projects: Project[];
  projectSections: ProjectSectionWithItems[];
  projectProgress: [number, number] | null;
  projectProgressMap: Map<number, [number, number]>;
  templateCounts: Map<number, { sections: number; items: number }>;

  // UI
  view: 'projects' | 'templates' | 'template-editor';
  selectedProjectId: number | null;
  selectedTemplateId: number | null;
  editingProjectId: number | null;
  loading: boolean;
  error: string | null;

  // Actions
  fetchTemplates: () => Promise<void>;
  fetchTemplateSections: (templateId: number) => Promise<void>;
  fetchProjects: () => Promise<void>;
  fetchProjectDetail: (projectId: number, silent?: boolean) => Promise<void>;
  createProject: (input: { name: string; description?: string; template_id: number; color?: string }) => Promise<void>;
  deleteProject: (id: number) => Promise<void>;
  deleteTemplate: (id: number) => Promise<void>;
  toggleItem: (itemId: number, checked: boolean) => Promise<void>;
  addProjectSection: (name: string) => Promise<void>;
  addProjectItem: (sectionId: number, title: string) => Promise<void>;
  deleteProjectItem: (id: number) => Promise<void>;
  deleteProjectSection: (id: number) => Promise<void>;

  setView: (view: 'projects' | 'templates') => void;
  setSelectedProjectId: (id: number | null) => void;
  setSelectedTemplateId: (id: number | null) => void;
  setEditingProjectId: (id: number | null) => void;
  clearError: () => void;
}

export const useStore = create<AppState>((set, get) => ({
  templates: [],
  templateSections: [],
  projects: [],
  projectSections: [],
  projectProgress: null,
  projectProgressMap: new Map(),
  templateCounts: new Map(),
  view: 'projects',
  selectedProjectId: null,
  selectedTemplateId: null,
  editingProjectId: null,
  loading: false,
  error: null,

  fetchTemplates: async () => {
    try {
      const templates = await api.apiListTemplates();
      set({ templates });
      // Fetch section/item counts for each template
      const counts = new Map<number, { sections: number; items: number }>();
      for (const t of templates) {
        try {
          const sections = await api.apiListTemplateSectionsWithItems(t.id);
          let totalItems = 0;
          for (const s of sections) {
            totalItems += s.items.length;
          }
          counts.set(t.id, { sections: sections.length, items: totalItems });
        } catch {
          counts.set(t.id, { sections: 0, items: 0 });
        }
      }
      set({ templateCounts: counts });
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  fetchTemplateSections: async (templateId) => {
    try {
      const sections = await api.apiListTemplateSectionsWithItems(templateId);
      set({ templateSections: sections });
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  fetchProjects: async () => {
    try {
      const projects = await api.apiListProjects();
      set({ projects });
      // Fetch progress for each project
      const progressMap = new Map<number, [number, number]>();
      for (const p of projects) {
        try {
          const progress = await api.apiGetProjectProgress(p.id);
          progressMap.set(p.id, progress);
        } catch {
          progressMap.set(p.id, [0, 0]);
        }
      }
      set({ projectProgressMap: progressMap });
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  fetchProjectDetail: async (projectId, silent = false) => {
    if (!silent) set({ loading: true, error: null });
    try {
      const [sections, progress] = await Promise.all([
        api.apiListProjectSectionsWithItems(projectId),
        api.apiGetProjectProgress(projectId),
      ]);
      set({ projectSections: sections, projectProgress: progress, loading: false });
    } catch (e: unknown) {
      set({ error: (e as Error).message, loading: false });
    }
  },

  createProject: async (input) => {
    set({ loading: true, error: null });
    try {
      await api.apiCreateProjectFromTemplate(input);
      await get().fetchProjects();
      set({ loading: false });
    } catch (e: unknown) {
      set({ error: (e as Error).message, loading: false });
    }
  },

  deleteProject: async (id) => {
    set({ loading: true, error: null });
    try {
      await api.apiDeleteProject(id);
      await get().fetchProjects();
      set({ loading: false });
    } catch (e: unknown) {
      set({ error: (e as Error).message, loading: false });
    }
  },

  deleteTemplate: async (id) => {
    try {
      await api.apiDeleteTemplate(id);
      await get().fetchTemplates();
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  toggleItem: async (itemId, checked) => {
    try {
      await api.apiUpdateProjectItem({ id: itemId, checked });
      const { selectedProjectId } = get();
      if (selectedProjectId) {
        await get().fetchProjectDetail(selectedProjectId, true);
      }
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  addProjectSection: async (name) => {
    const { selectedProjectId } = get();
    if (!selectedProjectId) return;
    try {
      await api.apiAddProjectSection({ project_id: selectedProjectId, name });
      await get().fetchProjectDetail(selectedProjectId, true);
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  addProjectItem: async (sectionId, title) => {
    try {
      await api.apiAddProjectItem({ section_id: sectionId, title });
      const { selectedProjectId } = get();
      if (selectedProjectId) {
        await get().fetchProjectDetail(selectedProjectId, true);
      }
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  deleteProjectItem: async (id) => {
    try {
      await api.apiDeleteProjectItem(id);
      const { selectedProjectId } = get();
      if (selectedProjectId) {
        await get().fetchProjectDetail(selectedProjectId, true);
      }
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  deleteProjectSection: async (id) => {
    try {
      await api.apiDeleteProjectSection(id);
      const { selectedProjectId } = get();
      if (selectedProjectId) {
        await get().fetchProjectDetail(selectedProjectId, true);
      }
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  setView: (view) => set({ view }),
  setSelectedProjectId: (id) => {
    set({ selectedProjectId: id });
    if (id) localStorage.setItem('pt_active_project_id', String(id));
  },
  setSelectedTemplateId: (id) => set({ selectedTemplateId: id }),
  setEditingProjectId: (id) => set({ editingProjectId: id }),
  clearError: () => set({ error: null }),
}));
