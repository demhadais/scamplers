create table chromium_dataset (
    id uuid primary key default uuidv7(),
    links jsonb generated always as (
        construct_links('chromium_datasets', id, '{specimens, suspensions, suspension-pools, cdna, libraries}')
    ) stored not null,
    name text not null,
    lab_id uuid references lab on delete restrict on update restrict not null,
    data_path text not null,
    delivered_at timestamptz not null,
    gems_id uuid references gems on delete restrict on update restrict not null,
    metrics jsonb not null,
    web_summary text not null
);
