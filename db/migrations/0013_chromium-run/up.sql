create table chromium_run (
    id uuid primary key default uuidv7(),
    links jsonb generated always as (construct_links('chromium-runs', id)) stored not null,
    readable_id text unique not null,
    chip text not null,
    run_at timestamptz not null,
    run_by uuid references person on delete restrict on update restrict not null,
    succeeded boolean not null,
    notes text
);

create table gems (
    id uuid primary key default uuidv7(),
    links jsonb generated always as (construct_links('gems', id)) stored not null,
    readable_id text unique not null,
    chemistry text references chemistry on delete restrict on update restrict,
    chromium_run_id uuid not null references chromium_run on delete restrict on update restrict
);

create table chip_loading (
    id uuid primary key default uuidv7(),
    gems_id uuid references gems on delete restrict on update restrict not null,
    suspension_id uuid references suspension on delete restrict on update restrict,
    suspension_pool_id uuid references suspension_pool on delete restrict on update restrict,
    suspension_volume_loaded jsonb not null,
    buffer_volume_loaded jsonb not null,
    notes text,

    unique (gems_id, suspension_id),
    unique (gems_id, suspension_pool_id),
    constraint has_suspension check ((suspension_id is null) != (suspension_pool_id is null))
);
