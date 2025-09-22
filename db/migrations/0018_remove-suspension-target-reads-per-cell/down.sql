-- This file should undo anything in `up.sql`
alter table suspension add column target_reads_per_cell integer not null;
