-- Your SQL goes here
alter table chromium_dataset drop column web_summary;
alter table chromium_dataset add column web_summaries text [] not null;
