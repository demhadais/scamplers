create table institution (
    id uuid primary key,
    links jsonb not null,
    name text unique not null
);
