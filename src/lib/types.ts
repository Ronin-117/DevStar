export interface Template {
  id: number;
  name: string;
  description: string;
  color: string;
  created_at: string;
  updated_at: string;
}

export interface TemplateSprint {
  id: number;
  template_id: number;
  name: string;
  description: string;
  sort_order: number;
  is_custom: boolean;
}

export interface TemplateSprintSection {
  id: number;
  sprint_id: number;
  section_id: number;
  sort_order: number;
  is_linked: boolean;
}

export interface TemplateSprintWithSections {
  sprint: TemplateSprint;
  sections: TemplateSprintSection[];
}

export interface SharedSection {
  id: number;
  name: string;
  description: string;
  color: string;
  created_at: string;
  updated_at: string;
}

export interface SharedSectionItem {
  id: number;
  section_id: number;
  title: string;
  description: string;
  sort_order: number;
}

export interface SectionWithItems {
  section: SharedSection;
  items: SharedSectionItem[];
}

export interface SharedSprint {
  id: number;
  name: string;
  description: string;
  sort_order: number;
  created_at: string;
  updated_at: string;
}

export interface SharedSprintSection {
  id: number;
  sprint_id: number;
  section_id: number;
  sort_order: number;
  is_linked: boolean;
}

export interface SharedSprintWithSections {
  sprint: SharedSprint;
  sections: SharedSprintSection[];
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

export interface ProjectSprint {
  id: number;
  project_id: number;
  name: string;
  description: string;
  status: string;
  sort_order: number;
  is_custom: boolean;
}

export interface ProjectSprintSection {
  id: number;
  sprint_id: number;
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

export interface ProjectSprintSectionWithItems {
  section: ProjectSprintSection;
  items: ProjectItem[];
}

export interface ProjectSprintWithSections {
  sprint: ProjectSprint;
  sections: ProjectSprintSectionWithItems[];
}
