create table chemistry (
-- we use name as primary key because that will not change
    name text primary key,
    description text not null,
    definition jsonb not null,
    library_types text [] not null,
    cmdline text not null
);
