create table sequencing_run (
    id uuid primary key default uuidv7(),
    links jsonb not null,
    readable_id text unique not null,
    begun_at timestamptz not null,
    finished_at timestamptz,
    additional_data jsonb
);
