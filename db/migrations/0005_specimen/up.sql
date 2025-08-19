create table specimen (
    id uuid primary key default uuidv7(),
    readable_id text unique not null,
    links jsonb generated always as (construct_links('specimens', id, '{"datasets"}')) stored not null,
    name text not null,
    submitted_by uuid references person on delete restrict on update restrict not null,
    lab_id uuid references lab on delete restrict on update restrict not null,
    received_at timestamptz not null,
    species text [] not null,
    notes text,
    returned_at timestamptz,
    returned_by uuid references person on delete restrict on update restrict,
    type text not null,
    embedded_in text,
    fixative text,
    frozen bool not null default false,
    cryopreserved bool not null default false,
    storage_buffer text,

    constraint not_both_frozen_and_cryopreserved check (not (cryopreserved and frozen))
);

create table committee_approval (
    institution_id uuid references institution on delete restrict on update restrict not null,
    specimen_id uuid references specimen on delete restrict on update restrict not null,
    committee_type text not null,
    compliance_identifier text not null,
    primary key (institution_id, committee_type, specimen_id)
);

create table specimen_measurement (
    id uuid primary key default uuidv7(),
    specimen_id uuid not null references specimen on delete restrict on update restrict,
    measured_by uuid not null references person on delete restrict on update restrict,
    data jsonb not null
);
