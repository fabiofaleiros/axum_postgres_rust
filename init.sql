CREATE TABLE IF NOT EXISTS TASKS (
    task_id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    priority INTEGER,
    status VARCHAR NOT NULL DEFAULT 'Open',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS status_history (
    id UUID PRIMARY KEY,
    task_id INTEGER NOT NULL REFERENCES tasks(task_id),
    from_status VARCHAR,
    to_status VARCHAR NOT NULL,
    changed_at TIMESTAMPTZ NOT NULL,
    changed_by VARCHAR NOT NULL,
    comment TEXT,
    user_role VARCHAR NOT NULL
);
