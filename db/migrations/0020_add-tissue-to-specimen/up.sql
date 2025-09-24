-- Your SQL goes here

-- We want `tissue` to be `not null`, so we need to create the column as nullable, populate all the rows with a value,
-- then set it to not null
alter table specimen add column tissue text;
update specimen set tissue = '';
alter table specimen alter tissue set not null;
