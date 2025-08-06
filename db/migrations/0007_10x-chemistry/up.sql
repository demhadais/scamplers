create table chemistry (
    name text primary key,
    description text not null,
    definition jsonb not null,
    cmdlines text [] not null
);
