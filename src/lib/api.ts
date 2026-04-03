import { invoke } from '@tauri-apps/api/core';
import { emit } from '@tauri-apps/api/event';
import type {
  Template, TemplateSprint, TemplateSprintWithSections,
  SharedSection, SharedSectionItem, SectionWithItems,
  SharedSprint, SharedSprintWithSections,
  Project, ProjectSprintWithSections, ProjectItem, ProjectSprintSection, ProjectSprint,
} from './types';

export async function apiListTemplates(): Promise<Template[]> {
  return invoke<Template[]>('list_templates');
}

export async function apiCreateTemplate(input: { name: string; description?: string; color?: string }): Promise<Template> {
  return invoke<Template>('create_template', { input });
}

export async function apiUpdateTemplate(id: number, name?: string, description?: string, color?: string): Promise<Template> {
  return invoke<Template>('update_template', { id, name, description, color });
}

export async function apiDeleteTemplate(id: number): Promise<void> {
  return invoke<void>('delete_template', { id });
}

export async function apiListTemplateSprints(templateId: number): Promise<TemplateSprint[]> {
  return invoke<TemplateSprint[]>('list_template_sprints', { templateId });
}

export async function apiGetTemplateSprintWithSections(sprintId: number): Promise<TemplateSprintWithSections> {
  return invoke<TemplateSprintWithSections>('get_template_sprint_with_sections', { sprintId });
}

export async function apiAddTemplateSprint(templateId: number, name: string, description: string): Promise<TemplateSprint> {
  return invoke<TemplateSprint>('add_template_sprint', { templateId, name, description });
}

export async function apiUpdateTemplateSprint(id: number, name?: string, description?: string): Promise<TemplateSprint> {
  return invoke<TemplateSprint>('update_template_sprint', { id, name, description });
}

export async function apiDeleteTemplateSprint(id: number): Promise<void> {
  return invoke<void>('delete_template_sprint', { id });
}

export async function apiAddTemplateSprintSection(sprintId: number, sectionId: number, isLinked: boolean): Promise<{ id: number }> {
  return invoke<{ id: number }>('add_template_sprint_section', { sprintId, sectionId, isLinked });
}

export async function apiDeleteTemplateSprintSection(id: number): Promise<void> {
  return invoke<void>('delete_template_sprint_section', { id });
}

export async function apiListSharedSections(): Promise<SharedSection[]> {
  return invoke<SharedSection[]>('list_shared_sections');
}

export async function apiGetSharedSectionWithItems(sectionId: number): Promise<SectionWithItems> {
  return invoke<SectionWithItems>('get_shared_section_with_items', { sectionId });
}

export async function apiCreateSharedSection(input: { name: string; description?: string; color?: string }): Promise<SharedSection> {
  return invoke<SharedSection>('create_shared_section', { input });
}

export async function apiUpdateSharedSection(id: number, name?: string, description?: string, color?: string): Promise<SharedSection> {
  return invoke<SharedSection>('update_shared_section', { id, name, description, color });
}

export async function apiDeleteSharedSection(id: number): Promise<void> {
  return invoke<void>('delete_shared_section', { id });
}

export async function apiAddSharedSectionItem(input: { section_id: number; title: string; description?: string }): Promise<SharedSectionItem> {
  return invoke<SharedSectionItem>('add_shared_section_item', { input });
}

export async function apiUpdateSharedSectionItem(id: number, title?: string, description?: string): Promise<SharedSectionItem> {
  return invoke<SharedSectionItem>('update_shared_section_item', { id, title, description });
}

export async function apiDeleteSharedSectionItem(id: number): Promise<void> {
  return invoke<void>('delete_shared_section_item', { id });
}

export async function apiListSharedSprints(): Promise<SharedSprint[]> {
  return invoke<SharedSprint[]>('list_shared_sprints');
}

export async function apiGetSharedSprintWithSections(sprintId: number): Promise<SharedSprintWithSections> {
  return invoke<SharedSprintWithSections>('get_shared_sprint_with_sections', { sprintId });
}

export async function apiCreateSharedSprint(input: { name: string; description?: string }): Promise<SharedSprint> {
  return invoke<SharedSprint>('create_shared_sprint', { input });
}

export async function apiUpdateSharedSprint(id: number, name?: string, description?: string): Promise<SharedSprint> {
  return invoke<SharedSprint>('update_shared_sprint', { id, name, description });
}

export async function apiDeleteSharedSprint(id: number): Promise<void> {
  return invoke<void>('delete_shared_sprint', { id });
}

export async function apiAddSharedSprintSection(input: { sprint_id: number; section_id: number; is_linked: boolean }): Promise<{ id: number }> {
  return invoke<{ id: number }>('add_shared_sprint_section', { input });
}

export async function apiDeleteSharedSprintSection(id: number): Promise<void> {
  return invoke<void>('delete_shared_sprint_section', { id });
}

export async function apiListProjects(): Promise<Project[]> {
  return invoke<Project[]>('list_projects');
}

export async function apiCreateProjectFromTemplate(input: { name: string; description?: string; template_id: number; color?: string }): Promise<Project> {
  return invoke<Project>('create_project_from_template', { input });
}

export async function apiDeleteProject(id: number): Promise<void> {
  return invoke<void>('delete_project', { id });
}

export async function apiListProjectSprints(projectId: number): Promise<ProjectSprintWithSections[]> {
  return invoke<ProjectSprintWithSections[]>('list_project_sprints', { projectId });
}

export async function apiSetSprintStatus(sprintId: number, status: string): Promise<void> {
  return invoke<void>('set_sprint_status', { sprintId, status });
}

export async function apiGetActiveSprint(projectId: number): Promise<ProjectSprint | null> {
  return invoke<ProjectSprint | null>('get_active_sprint', { projectId });
}

export async function apiGetProjectProgress(projectId: number): Promise<[number, number]> {
  return invoke<[number, number]>('get_project_progress', { projectId });
}

export async function apiCheckAndAdvanceSprint(projectId: number): Promise<ProjectSprint | null> {
  return invoke<ProjectSprint | null>('check_and_advance_sprint', { projectId });
}

export async function apiCompleteSprint(sprintId: number, projectId: number): Promise<ProjectSprint | null> {
  return invoke<ProjectSprint | null>('complete_sprint', { sprintId, projectId });
}

export async function apiUpdateProjectItem(input: { id: number; checked?: boolean; notes?: string }): Promise<ProjectItem> {
  return invoke<ProjectItem>('update_project_item', { input });
}

export async function apiToggleProjectItem(id: number): Promise<ProjectItem> {
  const result = await invoke<ProjectItem>('toggle_project_item', { id });
  await emit('project-item-toggled', { itemId: id, checked: result.checked });
  return result;
}

export async function apiAddProjectSection(input: { sprint_id: number; name: string; description?: string; linked_from_section_id?: number }): Promise<ProjectSprintSection> {
  return invoke<ProjectSprintSection>('add_project_section', { input });
}

export async function apiAddProjectItem(input: { section_id: number; title: string; description?: string }): Promise<ProjectItem> {
  return invoke<ProjectItem>('add_project_item', { input });
}

export async function apiDeleteProjectItem(id: number): Promise<void> {
  return invoke<void>('delete_project_item', { id });
}

export async function apiDeleteProjectSection(id: number): Promise<void> {
  return invoke<void>('delete_project_section', { id });
}

export async function apiAddProjectSprint(projectId: number, name: string, description: string): Promise<ProjectSprint> {
  return invoke<ProjectSprint>('add_project_sprint', { projectId, name, description });
}

export async function apiAddSharedSprintToProject(projectId: number, sharedSprintId: number, isLinked: boolean): Promise<ProjectSprint> {
  return invoke<ProjectSprint>('add_shared_sprint_to_project', { projectId, sharedSprintId, isLinked });
}

export async function apiToggleMode(mode: 'management' | 'active'): Promise<void> {
  const rustMode = mode === 'management' ? 'Management' : 'Active';
  await invoke<void>('toggle_mode', { mode: rustMode });
}

export async function apiResizeActiveWindow(width: number, height: number): Promise<void> {
  return invoke<void>('resize_active_window', { width, height });
}

export async function apiCloseWindow(): Promise<void> {
  return invoke<void>('close_window');
}

export async function apiMinimizeWindow(): Promise<void> {
  return invoke<void>('minimize_window');
}

export async function apiToggleMaximizeWindow(): Promise<void> {
  return invoke<void>('toggle_maximize_window');
}

export async function apiGetWindowLabel(): Promise<string> {
  return invoke<string>('get_window_label');
}

export async function apiSetActiveWindowCompact(): Promise<void> {
  return invoke<void>('set_active_window_compact');
}

export async function apiSetActiveWindowFull(): Promise<void> {
  return invoke<void>('set_active_window_full');
}
