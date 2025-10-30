create table tenx_assays (
    id uuid primary key default uuidv7(),
    links jsonb generated always as (construct_links('10x-assays', id)) stored not null,
    name case_insensitive_text not null,
    library_types case_insensitive_text [],
    sample_multiplexing case_insensitive_text,
    chemistry_version case_insensitive_text not null,
    protocol_url case_insensitive_text not null,
    chromium_chip case_insensitive_text,
    cmdlines case_insensitive_text [],

    unique (name, library_types, sample_multiplexing, chemistry_version)
);
