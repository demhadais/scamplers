create table sequencing_runs (
    id uuid primary key default uuidv7(),
    links jsonb generated always as (construct_links('sequencing-runs', id, '{"libraries"}')) stored not null,
    readable_id text unique not null,
    begun_at timestamptz not null,
    finished_at timestamptz,
    additional_data jsonb
);
