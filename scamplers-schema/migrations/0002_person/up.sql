create table person (
    id uuid primary key default uuidv7(),
    links jsonb generated always as (
        construct_links('people', id, '{"labs", "specimens", "chromium-datasets"}')
    ) stored not null,
    name text not null,
    email text unique,
    institution_id uuid references institution on delete restrict on update restrict not null,
    orcid text unique,
    ms_user_id uuid unique,
    hashed_api_key hashed_key unique
);
