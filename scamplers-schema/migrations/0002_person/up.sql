create table people (
    id uuid primary key default uuidv7(),
    links jsonb generated always as (
        construct_links('people', id, '{"labs", "specimens", "chromium-datasets"}')
    ) stored not null,
    name text not null,
    email text unique not null,
    email_verified boolean not null default false,
    institution_id uuid references institutions on delete restrict on update restrict not null,
    orcid text unique,
    microsoft_entra_oid uuid unique
);

create table api_keys (
    prefix text unique not null,
    hash text unique not null,
    user_id uuid references people on delete cascade on update cascade not null,
    primary key (prefix, hash)
);
