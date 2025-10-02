-- This file should undo anything in `up.sql`
alter table suspension add constraint pooling_is_correctly_specified check (
    (pooled_into is null) = (multiplexing_tag_id is null)
);
