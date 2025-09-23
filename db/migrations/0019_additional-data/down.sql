-- This file should undo anything in `up.sql`
alter table specimen drop column additional_data;
alter table specimen add column notes text;

alter table sequencing_run drop column additional_data;
alter table sequencing_run add column notes text;

alter table suspension_pool drop column additional_data;
alter table suspension_pool add column notes text;

alter table suspension drop column additional_data;
alter table suspension add column notes text;

alter table chromium_run drop column additional_data;
alter table chromium_run add column notes text;

alter table chip_loading drop column additional_data;
alter table chip_loading add column notes text;

alter table cdna drop column additional_data;
alter table cdna add column notes text;

alter table library drop column additional_data;
alter table library add column notes text;
