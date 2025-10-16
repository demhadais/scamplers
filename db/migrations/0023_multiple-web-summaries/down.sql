-- This file should undo anything in `up.sql`
alter table chromium_dataset drop column web_summaries;
alter table chromium_dataset add column web_summary text not null;
