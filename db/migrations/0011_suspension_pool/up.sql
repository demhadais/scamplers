create table suspension_pool (
    id uuid primary key default uuidv7(),
    links jsonb generated always as (construct_links('suspension-pools', id)) stored not null,
    name text not null,
    readable_id text unique not null,
    pooled_at timestamptz not null,
    notes text
);

create table suspension_pool_measurement (
    id uuid primary key default uuidv7(),
    pool_id uuid references suspension_pool on delete restrict on update restrict not null,
    measured_by uuid references person on delete restrict on update restrict not null,
    data jsonb not null
);

create table suspension_pool_preparers (
    pool_id uuid references suspension_pool on delete restrict on update restrict not null,
    prepared_by uuid references person on delete restrict on update restrict not null,
    primary key (pool_id, prepared_by)
);
