create table multiplexing_tag (
    id uuid primary key default uuidv7(),
    tag_id text not null,
    type text not null, -- constrained by Rust enum
    unique (tag_id, type)
);

create table suspension (
    id uuid primary key default uuidv7(),
    links jsonb generated always as (construct_links('suspensions', id)) stored not null,
    readable_id text unique not null,
    parent_specimen_id uuid references specimen on delete restrict on update restrict not null,
    biological_material text not null,
    created_at timestamptz,
    pooled_into uuid references suspension_pool on delete restrict on update restrict,
    multiplexing_tag_id uuid references multiplexing_tag on delete restrict on update restrict,
    lysis_duration_minutes real,
    target_cell_recovery real not null, -- validated on Rust side
    target_reads_per_cell integer not null, -- validated on Rust side
    notes text,

    -- two suspensions cannot be pooled together and tagged with the same tag
    unique (pooled_into, multiplexing_tag_id),
    -- either both are specified or neither is specified
    constraint pooling_is_correctly_specified check ((pooled_into is null) = (multiplexing_tag_id is null))
);

create table suspension_measurement (
    id uuid primary key default uuidv7(),
    suspension_id uuid references suspension on delete restrict on update restrict not null,
    measured_by uuid references person on delete restrict on update restrict not null,
    data jsonb not null
);

create table suspension_preparers (
    suspension_id uuid references suspension on delete restrict on update restrict not null,
    prepared_by uuid references person on delete restrict on update restrict not null,

    primary key (suspension_id, prepared_by)
);
