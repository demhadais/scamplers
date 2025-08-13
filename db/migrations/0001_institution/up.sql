create table institution (
    id uuid primary key,
    links jsonb generated always as (construct_links('institutions', id)) stored not null,
    name text unique not null
);
