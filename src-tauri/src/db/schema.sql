CREATE TABLE IF NOT EXISTS templates (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT    NOT NULL,
    description TEXT    NOT NULL DEFAULT '',
    color       TEXT    NOT NULL DEFAULT '#6366f1',
    created_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

-- Shared sections (reusable checklist blocks)
CREATE TABLE IF NOT EXISTS shared_sections (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT    NOT NULL,
    description TEXT    NOT NULL DEFAULT '',
    color       TEXT    NOT NULL DEFAULT '#6b7280',
    created_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS shared_section_items (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    section_id  INTEGER NOT NULL REFERENCES shared_sections(id) ON DELETE CASCADE,
    title       TEXT    NOT NULL,
    description TEXT    NOT NULL DEFAULT '',
    sort_order  INTEGER NOT NULL DEFAULT 0
);

-- Shared sprints (reusable sprint templates)
CREATE TABLE IF NOT EXISTS shared_sprints (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT    NOT NULL,
    description TEXT    NOT NULL DEFAULT '',
    sort_order  INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS shared_sprint_sections (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    sprint_id   INTEGER NOT NULL REFERENCES shared_sprints(id) ON DELETE CASCADE,
    section_id  INTEGER NOT NULL REFERENCES shared_sections(id) ON DELETE CASCADE,
    sort_order  INTEGER NOT NULL DEFAULT 0,
    is_linked   INTEGER NOT NULL DEFAULT 1
);

-- Template sprints
CREATE TABLE IF NOT EXISTS template_sprints (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    template_id INTEGER NOT NULL REFERENCES templates(id) ON DELETE CASCADE,
    name        TEXT    NOT NULL,
    description TEXT    NOT NULL DEFAULT '',
    sort_order  INTEGER NOT NULL DEFAULT 0,
    is_custom   INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS template_sprint_sections (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    sprint_id   INTEGER NOT NULL REFERENCES template_sprints(id) ON DELETE CASCADE,
    section_id  INTEGER NOT NULL REFERENCES shared_sections(id) ON DELETE CASCADE,
    sort_order  INTEGER NOT NULL DEFAULT 0,
    is_linked   INTEGER NOT NULL DEFAULT 1
);

-- Projects
CREATE TABLE IF NOT EXISTS projects (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT    NOT NULL,
    description TEXT    NOT NULL DEFAULT '',
    template_id INTEGER NOT NULL,
    color       TEXT    NOT NULL DEFAULT '#6366f1',
    created_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

-- Project sprints
CREATE TABLE IF NOT EXISTS project_sprints (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id  INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name        TEXT    NOT NULL,
    description TEXT    NOT NULL DEFAULT '',
    status      TEXT    NOT NULL DEFAULT 'pending' CHECK(status IN ('pending', 'active', 'done')),
    sort_order  INTEGER NOT NULL DEFAULT 0,
    is_custom   INTEGER NOT NULL DEFAULT 0
);

-- Project sprint sections
CREATE TABLE IF NOT EXISTS project_sprint_sections (
    id                       INTEGER PRIMARY KEY AUTOINCREMENT,
    sprint_id                INTEGER NOT NULL REFERENCES project_sprints(id) ON DELETE CASCADE,
    name                     TEXT    NOT NULL,
    description              TEXT    NOT NULL DEFAULT '',
    sort_order               INTEGER NOT NULL DEFAULT 0,
    is_custom                INTEGER NOT NULL DEFAULT 0,
    linked_from_section_id   INTEGER REFERENCES shared_sections(id)
);

-- Project items
CREATE TABLE IF NOT EXISTS project_items (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    section_id  INTEGER NOT NULL REFERENCES project_sprint_sections(id) ON DELETE CASCADE,
    title       TEXT    NOT NULL,
    description TEXT    NOT NULL DEFAULT '',
    checked     INTEGER NOT NULL DEFAULT 0,
    notes       TEXT    NOT NULL DEFAULT '',
    sort_order  INTEGER NOT NULL DEFAULT 0,
    is_custom   INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_shared_section_items_section ON shared_section_items(section_id);
CREATE INDEX IF NOT EXISTS idx_shared_sprint_sections_sprint ON shared_sprint_sections(sprint_id);
CREATE INDEX IF NOT EXISTS idx_template_sprints_template ON template_sprints(template_id);
CREATE INDEX IF NOT EXISTS idx_template_sprint_sections_sprint ON template_sprint_sections(sprint_id);
CREATE INDEX IF NOT EXISTS idx_project_sprints_project ON project_sprints(project_id);
CREATE INDEX IF NOT EXISTS idx_project_sprint_sections_sprint ON project_sprint_sections(sprint_id);
CREATE INDEX IF NOT EXISTS idx_project_items_section ON project_items(section_id);
CREATE INDEX IF NOT EXISTS idx_projects_template ON projects(template_id);
