create table chromium_dataset (
-- use the metadata_id as the primary key for simplicity
    id uuid primary key references dataset_metadata on delete restrict on update restrict,
    gems_id uuid references gems on delete restrict on update restrict not null,
    metrics jsonb [] not null, -- validated on Rust side to be the correct json
    web_summary text not null -- validated on Rust side to be HTML
);
