create table multiplexing_tags (
    id uuid primary key default uuidv7(),
    tag_id text not null,
    type text not null, -- constrained by Rust enum
    unique (tag_id, type)
);

create table suspensions (
    id uuid primary key default uuidv7(),
    links jsonb generated always as (construct_links('suspensions', id, '{"measurements"}')) stored not null,
    readable_id text unique not null,
    parent_specimen_id uuid references specimens on delete restrict on update restrict not null,
    biological_material text not null,
    created_at timestamptz,
    pooled_into uuid references suspension_pools on delete restrict on update restrict,
    multiplexing_tag_id uuid references multiplexing_tags on delete restrict on update restrict,
    lysis_duration_minutes real,
    target_cell_recovery real not null,
    additional_data jsonb,

    -- two suspensions cannot be pooled together and tagged with the same tag
    unique (pooled_into, multiplexing_tag_id)
);

create table suspension_measurements (
    id uuid primary key default uuidv7(),
    suspension_id uuid references suspensions on delete restrict on update restrict not null,
    measured_by uuid references people on delete restrict on update restrict not null,
    data jsonb not null
);

create table suspension_preparers (
    suspension_id uuid references suspensions on delete restrict on update restrict not null,
    prepared_by uuid references people on delete restrict on update restrict not null,

    primary key (suspension_id, prepared_by)
);
