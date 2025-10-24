create table institution (
    id uuid primary key,
    links jsonb generated always as (construct_links('institutions', id, '{"people"}')) stored not null,
    name text unique not null
);
