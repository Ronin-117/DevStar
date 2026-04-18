import { create } from 'zustand';
import { listen } from '@tauri-apps/api/event';
import * as api from '../lib/api';
import type {
  Template, Project, ProjectSprintWithSections,
  SharedSection, SectionWithItems, SharedSprint, SharedSprintWithSections,
  TemplateSprintWithSections,
} from '../lib/types';

export type LibraryTab = 'templates' | 'shared-sections' | 'shared-sprints';

interface AppState {
  // Data
  templates: Template[];
  projects: Project[];
  sharedSections: SharedSection[];
  sharedSprints: SharedSprint[];

  // Detail caches
  templateSprints: Map<number, TemplateSprintWithSections[]>;
  projectSprints: Map<number, ProjectSprintWithSections[]>;
  sharedSectionDetail: Map<number, SectionWithItems>;
  sharedSprintDetail: Map<number, SharedSprintWithSections>;
  projectProgressMap: Map<number, [number, number]>;
  currentSprintMap: Map<number, string>;
  settings: api.Settings | null;

  // UI
  view: 'projects' | 'library' | 'template-editor' | 'settings';
  libraryTab: LibraryTab;
  selectedProjectId: number | null;
  selectedTemplateId: number | null;
  editingProjectId: number | null;
  loading: boolean;
  error: string | null;

  // Actions - fetch
  fetchTemplates: () => Promise<void>;
  fetchProjects: () => Promise<void>;
  fetchProjectDetail: (projectId: number, silent?: boolean) => Promise<void>;
  fetchSharedSections: () => Promise<void>;
  fetchSharedSprints: () => Promise<void>;
  fetchTemplateSprints: (templateId: number) => Promise<void>;
  fetchSettings: () => Promise<void>;
  updateSettings: (settings: api.Settings) => Promise<void>;

  // Actions - shared sections
  createSharedSection: (input: { name: string; description?: string; color?: string }) => Promise<void>;
  deleteSharedSection: (id: number) => Promise<void>;
  addSharedSectionItem: (sectionId: number, title: string) => Promise<void>;
  deleteSharedSectionItem: (itemId: number, sectionId: number) => Promise<void>;

  // Actions - shared sprints
  createSharedSprint: (input: { name: string; description?: string }) => Promise<void>;
  deleteSharedSprint: (id: number) => Promise<void>;
  addSharedSprintSection: (sprintId: number, sectionId: number, isLinked: boolean) => Promise<void>;
  deleteSharedSprintSection: (sprintId: number, sectionId: number) => Promise<void>;

  // Actions - template sprints
  addTemplateSprint: (templateId: number, name: string, description: string) => Promise<void>;
  deleteTemplateSprint: (id: number, templateId: number) => Promise<void>;
  addTemplateSprintSection: (sprintId: number, sectionId: number, isLinked: boolean, templateId: number) => Promise<void>;
  deleteTemplateSprintSection: (id: number, templateId: number) => Promise<void>;

  // Actions - projects
  createProject: (input: { name: string; description?: string; template_id: number; color?: string }) => Promise<void>;
  deleteProject: (id: number) => Promise<void>;
  deleteTemplate: (id: number) => Promise<void>;
  setSprintStatus: (sprintId: number, status: string, projectId: number) => Promise<void>;
  toggleItem: (itemId: number, projectId: number) => Promise<void>;
  addProjectSection: (input: { sprint_id: number; name: string; description?: string; linked_from_section_id?: number }, projectId: number) => Promise<void>;
  addProjectItem: (input: { section_id: number; title: string; description?: string }, projectId: number) => Promise<void>;
  deleteProjectItem: (id: number, projectId: number) => Promise<void>;
  deleteProjectSection: (id: number, projectId: number) => Promise<void>;

  // UI actions
  setView: (view: AppState['view']) => void;
  setLibraryTab: (tab: LibraryTab) => void;
  setSelectedProjectId: (id: number | null) => void;
  setSelectedTemplateId: (id: number | null) => void;
  setEditingProjectId: (id: number | null) => void;
  clearError: () => void;
}

export const useStore = create<AppState>((set, get) => ({
  templates: [],
  projects: [],
  sharedSections: [],
  sharedSprints: [],
  templateSprints: new Map(),
  projectSprints: new Map(),
  sharedSectionDetail: new Map(),
  sharedSprintDetail: new Map(),
  projectProgressMap: new Map(),
  currentSprintMap: new Map(),
  settings: null,
  view: 'projects',
  libraryTab: 'templates',
  selectedProjectId: null,
  selectedTemplateId: null,
  editingProjectId: null,
  loading: false,
  error: null,

  fetchTemplates: async () => {
    try {
      set({ templates: await api.apiListTemplates() });
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  fetchProjects: async () => {
    try {
      const projects = await api.apiListProjects();
      set({ projects });
      const progressMap = new Map<number, [number, number]>();
      const sprintMap = new Map<number, string>();
      for (const p of projects) {
        try {
          progressMap.set(p.id, await api.apiGetProjectProgress(p.id));
        } catch {
          progressMap.set(p.id, [0, 0]);
        }
        // Also fetch the active sprint so the sprint tag shows for all projects
        try {
          const active = await api.apiGetActiveSprint(p.id);
          if (active) {
            sprintMap.set(p.id, `Sprint ${active.sort_order + 1}: ${active.name}`);
          }
        } catch {
          // skip
        }
      }
      set({ projectProgressMap: progressMap, currentSprintMap: sprintMap });
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  fetchProjectDetail: async (projectId, silent = false) => {
    if (!silent) set({ loading: true, error: null });
    try {
      const sprints = await api.apiListProjectSprints(projectId);
      const activeSprint = sprints.find((s) => s.sprint.status === 'active');
      const currentSprintMap = new Map(get().currentSprintMap);
      if (activeSprint) {
        currentSprintMap.set(projectId, `Sprint ${activeSprint.sprint.sort_order + 1}: ${activeSprint.sprint.name}`);
      }
      // Update progress map from fetched data
      const progressMap = new Map(get().projectProgressMap);
      const totalChecked = sprints.reduce(
        (sum, s) => sum + s.sections.reduce((s2, sec) => s2 + sec.items.filter((i) => i.checked).length, 0),
        0,
      );
      const totalItems = sprints.reduce(
        (sum, s) => sum + s.sections.reduce((s2, sec) => s2 + sec.items.length, 0),
        0,
      );
      progressMap.set(projectId, [totalChecked, totalItems]);
      set({
        projectSprints: new Map(get().projectSprints).set(projectId, sprints),
        currentSprintMap,
        projectProgressMap: progressMap,
        loading: false,
      });
    } catch (e: unknown) {
      set({ error: (e as Error).message, loading: false });
    }
  },

  fetchSharedSections: async () => {
    try {
      set({ sharedSections: await api.apiListSharedSections() });
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  fetchSharedSprints: async () => {
    try {
      set({ sharedSprints: await api.apiListSharedSprints() });
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  fetchTemplateSprints: async (templateId) => {
    try {
      const sprints = await api.apiListTemplateSprints(templateId);
      const details: TemplateSprintWithSections[] = [];
      for (const s of sprints) {
        try {
          details.push(await api.apiGetTemplateSprintWithSections(s.id));
        } catch { /* skip */ }
      }
      set({ templateSprints: new Map(get().templateSprints).set(templateId, details) });
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  fetchSettings: async () => {
    try {
      set({ settings: await api.apiGetSettings() });
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  updateSettings: async (settings) => {
    try {
      await api.apiUpdateSettings(settings);
      set({ settings });
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  createSharedSection: async (input) => {
    try {
      await api.apiCreateSharedSection(input);
      await get().fetchSharedSections();
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  deleteSharedSection: async (id) => {
    try {
      await api.apiDeleteSharedSection(id);
      await get().fetchSharedSections();
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  addSharedSectionItem: async (sectionId, title) => {
    try {
      await api.apiAddSharedSectionItem({ section_id: sectionId, title });
      const detail = await api.apiGetSharedSectionWithItems(sectionId);
      set({ sharedSectionDetail: new Map(get().sharedSectionDetail).set(sectionId, detail) });
      await get().fetchSharedSections();
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  deleteSharedSectionItem: async (itemId, sectionId) => {
    try {
      await api.apiDeleteSharedSectionItem(itemId);
      const detail = await api.apiGetSharedSectionWithItems(sectionId);
      set({ sharedSectionDetail: new Map(get().sharedSectionDetail).set(sectionId, detail) });
      await get().fetchSharedSections();
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  createSharedSprint: async (input) => {
    try {
      await api.apiCreateSharedSprint(input);
      await get().fetchSharedSprints();
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  deleteSharedSprint: async (id) => {
    try {
      await api.apiDeleteSharedSprint(id);
      await get().fetchSharedSprints();
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  addSharedSprintSection: async (sprintId, sectionId, isLinked) => {
    try {
      await api.apiAddSharedSprintSection({ sprint_id: sprintId, section_id: sectionId, is_linked: isLinked });
      const detail = await api.apiGetSharedSprintWithSections(sprintId);
      set({ sharedSprintDetail: new Map(get().sharedSprintDetail).set(sprintId, detail) });
      await get().fetchSharedSprints();
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  deleteSharedSprintSection: async (sprintId, sectionId) => {
    try {
      const detail = await api.apiGetSharedSprintWithSections(sprintId);
      const section = detail.sections.find((s) => s.section_id === sectionId);
      if (section) await api.apiDeleteSharedSprintSection(section.id);
      const refreshed = await api.apiGetSharedSprintWithSections(sprintId);
      set({ sharedSprintDetail: new Map(get().sharedSprintDetail).set(sprintId, refreshed) });
      await get().fetchSharedSprints();
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  addTemplateSprint: async (templateId, name, description) => {
    try {
      await api.apiAddTemplateSprint(templateId, name, description);
      await get().fetchTemplateSprints(templateId);
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  deleteTemplateSprint: async (id, templateId) => {
    try {
      await api.apiDeleteTemplateSprint(id);
      await get().fetchTemplateSprints(templateId);
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  addTemplateSprintSection: async (sprintId, sectionId, isLinked, templateId) => {
    try {
      await api.apiAddTemplateSprintSection(sprintId, sectionId, isLinked);
      await get().fetchTemplateSprints(templateId);
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  deleteTemplateSprintSection: async (id, templateId) => {
    try {
      await api.apiDeleteTemplateSprintSection(id);
      await get().fetchTemplateSprints(templateId);
    } catch (e: unknown) {
      set({ error: (e as Error).message });
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

  setSprintStatus: async (sprintId, status, projectId) => {
    try {
      await api.apiSetSprintStatus(sprintId, status);
      await get().fetchProjectDetail(projectId, true);
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  toggleItem: async (itemId, projectId) => {
    try {
      await api.apiToggleProjectItem(itemId);
      await get().fetchProjectDetail(projectId, true);
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  addProjectSection: async (input, projectId) => {
    try {
      await api.apiAddProjectSection(input);
      await get().fetchProjectDetail(projectId, true);
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  addProjectItem: async (input, projectId) => {
    try {
      await api.apiAddProjectItem(input);
      await get().fetchProjectDetail(projectId, true);
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  deleteProjectItem: async (id, projectId) => {
    try {
      await api.apiDeleteProjectItem(id);
      await get().fetchProjectDetail(projectId, true);
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  deleteProjectSection: async (id, projectId) => {
    try {
      await api.apiDeleteProjectSection(id);
      await get().fetchProjectDetail(projectId, true);
    } catch (e: unknown) {
      set({ error: (e as Error).message });
    }
  },

  setView: (view) => set({ view }),
  setLibraryTab: (libraryTab) => set({ libraryTab }),
  setSelectedProjectId: (id) => {
    set({ selectedProjectId: id });
    if (id) localStorage.setItem('pt_active_project_id', String(id));
  },
  setSelectedTemplateId: (id) => set({ selectedTemplateId: id }),
  setEditingProjectId: (id) => set({ editingProjectId: id }),
  clearError: () => set({ error: null }),
}));

// Listen for events from the active window and update cached data in-place.
// When all items in the active sprint become checked, also advance the sprint
// (mark current as 'done', next pending as 'active') to keep the UI in sync.
listen('project-item-toggled', (event: { payload: { itemId: number; checked: boolean } }) => {
  const { itemId, checked } = event.payload;
  const state = useStore.getState();
  const updated = new Map(state.projectSprints);
  let changedProjectId: number | null = null;
  let changedSprints: ProjectSprintWithSections[] | null = null;

  for (const [projectId, sprints] of updated.entries()) {
    const newSprints = sprints.map((sprint) => {
      const newSections = sprint.sections.map((section) => {
        const newItems = section.items.map((item) => {
          if (item.id === itemId) {
            changedProjectId = projectId;
            return { ...item, checked };
          }
          return item;
        });
        return { ...section, items: newItems };
      });
      return { ...sprint, sections: newSections };
    });
    if (changedProjectId !== null) {
      changedSprints = newSprints;
      break;
    }
  }

  if (changedProjectId !== null && changedSprints !== null) {
    // Check if the active sprint just got fully completed — if so, advance it
    const activeSprint = changedSprints.find((s) => s.sprint.status === 'active');
    if (activeSprint) {
      const totalItems = activeSprint.sections.reduce((sum, sec) => sum + sec.items.length, 0);
      const checkedItems = activeSprint.sections.reduce(
        (sum, sec) => sum + sec.items.filter((i) => i.checked).length,
        0,
      );
      const allDone = totalItems > 0 && checkedItems >= totalItems;

      if (allDone) {
        // Find the next pending sprint by sort_order
        const nextPending = changedSprints
          .filter((s) => s.sprint.status === 'pending')
          .sort((a, b) => a.sprint.sort_order - b.sprint.sort_order)[0];

        const resultSprints = changedSprints.map((sprint) => {
          if (sprint.sprint.id === activeSprint.sprint.id) {
            return { ...sprint, sprint: { ...sprint.sprint, status: 'done' as const } };
          }
          if (nextPending && sprint.sprint.id === nextPending.sprint.id) {
            return { ...sprint, sprint: { ...sprint.sprint, status: 'active' as const } };
          }
          return sprint;
        });

        updated.set(changedProjectId, resultSprints);

        // Update progress map
        const totalChecked = resultSprints.reduce(
          (sum, s) => sum + s.sections.reduce((s2, sec) => s2 + sec.items.filter((i) => i.checked).length, 0),
          0,
        );
        const total = resultSprints.reduce(
          (sum, s) => sum + s.sections.reduce((s2, sec) => s2 + sec.items.length, 0),
          0,
        );
        const progressMap = new Map(state.projectProgressMap);
        progressMap.set(changedProjectId, [totalChecked, total]);

        // Update current sprint map
        const currentSprintMap = new Map(state.currentSprintMap);
        const newActive = resultSprints.find((s) => s.sprint.status === 'active');
        if (newActive) {
          currentSprintMap.set(changedProjectId, `Sprint ${newActive.sprint.sort_order + 1}: ${newActive.sprint.name}`);
        } else {
          currentSprintMap.delete(changedProjectId);
        }

        useStore.setState({ projectSprints: updated, projectProgressMap: progressMap, currentSprintMap });
        return;
      }
    }

    // No sprint advancement needed, just update item state
    updated.set(changedProjectId, changedSprints);
    const totalChecked = changedSprints.reduce(
      (sum, s) => sum + s.sections.reduce((s2, sec) => s2 + sec.items.filter((i) => i.checked).length, 0),
      0,
    );
    const totalItems = changedSprints.reduce(
      (sum, s) => sum + s.sections.reduce((s2, sec) => s2 + sec.items.length, 0),
      0,
    );
    const progressMap = new Map(state.projectProgressMap);
    progressMap.set(changedProjectId, [totalChecked, totalItems]);
    useStore.setState({ projectSprints: updated, projectProgressMap: progressMap });
  }
}).catch(() => {});
