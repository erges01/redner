CREATE TABLE IF NOT EXISTS project_timelines (
  project_id UUID PRIMARY KEY REFERENCES projects(id) ON DELETE CASCADE,
  schema_version INT NOT NULL DEFAULT 1,
  timeline_json JSONB NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_project_timelines_updated_at
ON project_timelines(updated_at);