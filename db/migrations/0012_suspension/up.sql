create table multiplexing_tag (
    id uuid primary key default uuidv7(),
    tag_id text not null,
    type text not null, -- constrained by Rust enum
    unique (tag_id, type)
);

create table suspension (
    id uuid primary key default uuidv7(),
    link text generated always as ('/samples/' || id) stored not null,
    readable_id text unique not null,
    metadata_id uuid references sample_metadata on delete restrict on update restrict,
    parent_specimen_id uuid references specimen on delete restrict on update restrict,
    is_derived boolean generated always as (parent_specimen_id is not null) stored,
    biological_material text not null,
    created_at timestamptz not null,
    pooled_into_id uuid references multiplexed_suspension on delete restrict on update restrict,
    multiplexing_tag_id uuid references multiplexing_tag on delete restrict on update restrict,
    lysis_duration_min real,
    target_cell_recovery real not null, -- validated on Rust side
    target_reads_per_cell integer not null, -- validated on Rust side
    notes text [],

    -- a derived suspension must not have its own metadata
    constraint has_metadata check (is_derived = (metadata_id is null)),
    -- a derived suspension must have a creation date
    constraint has_creation_time check (is_derived = (created_at is not null)),
    -- either both are specified or neither is specified
    constraint pooling_is_correctly_specified check ((pooled_into_id is null) = (multiplexing_tag_id is null))
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
