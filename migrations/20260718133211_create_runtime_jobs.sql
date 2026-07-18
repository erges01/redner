CREATE TABLE runtime_jobs (
    id UUID PRIMARY KEY,
    project_id UUID NOT NULL,
    status VARCHAR(50) NOT NULL,
    graph_state JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);