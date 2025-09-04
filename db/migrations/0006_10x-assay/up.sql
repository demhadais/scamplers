create table tenx_assay (
    id uuid primary key default uuidv7(),
    name text not null,
    library_types text [],
    sample_multiplexing text,
    chemistry_version text not null,
    protocol_url text unique not null,
    chromium_chip text,
    cmdline text,

    unique (name, library_types, sample_multiplexing, chemistry_version)
);
