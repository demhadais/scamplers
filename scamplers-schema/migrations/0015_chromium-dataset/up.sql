create table chromium_dataset (
    id uuid primary key default uuidv7(),
    links jsonb not null,
    name text not null,
    lab_id uuid references lab on delete restrict on update restrict not null,
    data_path text not null,
    delivered_at timestamptz not null,
    metrics jsonb not null,
    web_summaries text [] not null
);

create table chromium_dataset_libraries (
    dataset_id uuid references chromium_dataset on delete restrict on update restrict not null,
    library_id uuid references library on delete restrict on update restrict not null,
    primary key (dataset_id, library_id)
);
