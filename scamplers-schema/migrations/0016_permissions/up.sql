-- Anyone can read any table
grant select on all tables in schema public to public;

-- Users with app_admin can do anything
grant all on all tables in schema public to app_admin;

-- login_user must be able to create people
grant insert, select, update on people to login_user;

-- login_user must be able to read api_keys to switch to the correct user
grant select on api_keys to login_user;

-- Anyone can insert or delete API keys
grant insert, delete on api_keys to public;

-- A person can only insert or delete their own API key, but login_user needs to view all of them
alter table api_keys enable row level security;
create policy user_api_key on api_keys using (current_user = user_id::text or current_user = 'login_user');
