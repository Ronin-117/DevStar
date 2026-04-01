import { invoke } from '@tauri-apps/api/core';
import type {
  Template, SectionWithItems, TemplateSection, TemplateItem,
  Project, ProjectSectionWithItems, ProjectItem,
} from './types';

// --- Templates ---

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

// --- Template Sections ---

export async function apiListTemplateSectionsWithItems(templateId: number): Promise<SectionWithItems[]> {
  return invoke<SectionWithItems[]>('list_template_sections_with_items', { templateId });
}

export async function apiAddTemplateSection(input: { template_id: number; name: string; description?: string; linked_from_section_id?: number }): Promise<TemplateSection> {
  return invoke<TemplateSection>('add_template_section', { input });
}

export async function apiUpdateTemplateSection(id: number, name?: string, description?: string): Promise<TemplateSection> {
  return invoke<TemplateSection>('update_template_section', { id, name, description });
}

export async function apiDeleteTemplateSection(id: number): Promise<void> {
  return invoke<void>('delete_template_section', { id });
}

// --- Template Items ---

export async function apiAddTemplateItem(input: { section_id: number; title: string; description?: string }): Promise<TemplateItem> {
  return invoke<TemplateItem>('add_template_item', { input });
}

export async function apiUpdateTemplateItem(id: number, title?: string, description?: string): Promise<TemplateItem> {
  return invoke<TemplateItem>('update_template_item', { id, title, description });
}

export async function apiDeleteTemplateItem(id: number): Promise<void> {
  return invoke<void>('delete_template_item', { id });
}

// --- Projects ---

export async function apiListProjects(): Promise<Project[]> {
  return invoke<Project[]>('list_projects');
}

export async function apiCreateProjectFromTemplate(input: { name: string; description?: string; template_id: number; color?: string }): Promise<Project> {
  return invoke<Project>('create_project_from_template', { input });
}

export async function apiDeleteProject(id: number): Promise<void> {
  return invoke<void>('delete_project', { id });
}

export async function apiGetProjectProgress(projectId: number): Promise<[number, number]> {
  return invoke<[number, number]>('get_project_progress', { projectId });
}

// --- Project Sections/Items ---

export async function apiListProjectSectionsWithItems(projectId: number): Promise<ProjectSectionWithItems[]> {
  return invoke<ProjectSectionWithItems[]>('list_project_sections_with_items', { projectId });
}

export async function apiUpdateProjectItem(input: { id: number; checked?: boolean; notes?: string }): Promise<ProjectItem> {
  return invoke<ProjectItem>('update_project_item', { input });
}

export async function apiAddProjectSection(input: { project_id: number; name: string; description?: string }): Promise<{ id: number }> {
  return invoke<{ id: number }>('add_project_section', { input });
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

// --- Window ---

export async function apiToggleMode(mode: 'management' | 'active'): Promise<void> {
  console.log('[api] toggle_mode called:', mode);
  try {
    const rustMode = mode === 'management' ? 'Management' : 'Active';
    await invoke<void>('toggle_mode', { mode: rustMode });
    console.log('[api] toggle_mode succeeded');
  } catch (e) {
    console.error('[api] toggle_mode failed:', e);
    throw e;
  }
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
