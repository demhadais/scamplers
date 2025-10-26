-- Table names are pluralized because it makes the Rust module have a different name from the modules in `scamplers-models`
create table institutions (
    id uuid primary key,
    links jsonb generated always as (construct_links('institutions', id, '{"people"}')) stored not null,
    name text unique not null
);
