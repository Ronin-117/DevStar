export interface Template {
  id: number;
  name: string;
  description: string;
  color: string;
  created_at: string;
  updated_at: string;
}

export interface TemplateSection {
  id: number;
  template_id: number;
  name: string;
  description: string;
  sort_order: number;
  linked_from_section_id: number | null;
}

export interface TemplateItem {
  id: number;
  section_id: number;
  title: string;
  description: string;
  sort_order: number;
}

export interface SectionWithItems {
  section: TemplateSection;
  items: TemplateItem[];
}

export interface Project {
  id: number;
  name: string;
  description: string;
  template_id: number;
  color: string;
  created_at: string;
  updated_at: string;
}

export interface ProjectSection {
  id: number;
  project_id: number;
  name: string;
  description: string;
  sort_order: number;
  is_custom: boolean;
  linked_from_section_id: number | null;
}

export interface ProjectItem {
  id: number;
  section_id: number;
  title: string;
  description: string;
  checked: boolean;
  notes: string;
  sort_order: number;
  is_custom: boolean;
}

export interface ProjectSectionWithItems {
  section: ProjectSection;
  items: ProjectItem[];
}
