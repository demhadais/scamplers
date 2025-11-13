-- Anyone can read any table (API keys are okay because only their hashes are stored, and they are protected by
-- row-level security anyways)
grant select on all tables in schema public to public;

-- Users with app_admin can do anything (TODO: app_admin should be more restricted)
grant all on all tables in schema public to app_admin;

-- scamplers_ui creates people and their API keys
grant insert on people, api_keys to scamplers_ui;
grant update on people to scamplers_ui;

-- Anyone can delete an API key
grant delete on api_keys to public;

-- A person can only insert or delete their own API key, but scamplers_api needs to see all of them and scamplers_ui
-- needs to create them
alter table api_keys enable row level security;
create policy user_api_key on api_keys using (
    current_user = user_id::text or current_user in ('scamplers_api', 'scamplers_ui')
);
