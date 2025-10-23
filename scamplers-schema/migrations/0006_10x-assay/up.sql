create table tenx_assay (
    id uuid primary key default uuidv7(),
    links jsonb not null,
    name text not null,
    library_types text [],
    sample_multiplexing text,
    chemistry_version text not null,
    protocol_url text not null,
    chromium_chip text,
    cmdlines text [],

    unique (name, library_types, sample_multiplexing, chemistry_version)
);
