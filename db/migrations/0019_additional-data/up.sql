-- Your SQL goes here
alter table specimen add column additional_data jsonb;
alter table specimen drop column notes;
alter table specimen drop column storage_buffer;

alter table sequencing_run add column additional_data jsonb;
alter table sequencing_run drop column notes;

alter table suspension_pool add column additional_data jsonb;
alter table suspension_pool drop column notes;

alter table suspension add column additional_data jsonb;
alter table suspension drop column notes;

alter table chromium_run add column additional_data jsonb;
alter table chromium_run drop column notes;

alter table chip_loading add column additional_data jsonb;
alter table chip_loading drop column notes;

alter table cdna add column additional_data jsonb;
alter table cdna drop column notes;

alter table library add column additional_data jsonb;
alter table library drop column notes;
