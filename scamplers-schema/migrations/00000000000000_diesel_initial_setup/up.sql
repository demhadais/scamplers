-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.


-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```

create or replace function diesel_manage_updated_at(_tbl regclass) returns void as $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ language plpgsql;

create or replace function diesel_set_updated_at() returns trigger as $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ language plpgsql;

create function role_exists(user_id text) returns boolean language plpgsql volatile strict as $$
    declare role_exists boolean;
    begin
        select exists (select 1 from pg_roles where rolname = user_id) into role_exists;
        return role_exists;
    end;
$$;

create function grant_roles_to_user(
    user_id text,
    roles text []
) returns void language plpgsql volatile strict as $$
    declare r text;
    begin
        foreach r in array roles loop
            execute format('grant %I to %I', r, user_id);
        end loop;
    end;
$$;

create function revoke_roles_from_user(
    user_id text,
    roles text []
) returns void language plpgsql volatile strict as $$
    declare r text;
    begin
        if not role_exists(user_id) then
            return;
        end if;

        foreach r in array roles loop
            execute format('revoke %I from %I', r, user_id);
        end loop;
    end;
$$;

create function create_role_if_not_exists(
    role_name text
) returns void language plpgsql volatile strict as $$
    begin
        if not role_exists(role_name) then
            execute format('create role %I', role_name);
        end if;
    end;
$$;

create function create_user_if_not_exists(
    user_id text,
    roles text []
) returns void language plpgsql volatile strict as $$
    begin
        perform create_role_if_not_exists(user_id);
        execute format('grant %I to scamplers_api with admin true, inherit false', user_id);
        execute format('alter role %I with login', user_id);
        perform grant_roles_to_user(user_id, roles);
    end;
$$;

create function get_user_roles(
    user_id text
) returns text [] language plpgsql volatile strict as $$
    declare roles text [];
    begin
        select coalesce(nullif(array_agg(pg_roles.rolname), '{null}'), '{}') from pg_roles inner join pg_auth_members on pg_roles.oid = pg_auth_members.roleid and pg_auth_members.member = (select usesysid from pg_user where usename = user_id) into roles;
        return roles;
    end;
$$;

create function construct_links(
    self_name text,
    id uuid,
    children text [] default '{}'
) returns jsonb language plpgsql immutable strict as $$
    declare links jsonb;
    declare child text;
    begin
        select json_object('self_': concat('/', self_name, '/', id)) into links;
        foreach child in array children loop
            select links || json_object(child: concat('/', self_name, '/', id, '/', child))::jsonb into links;
        end loop;
        return links;
    end;
$$;

-- Roles assigned to people
select create_role_if_not_exists('app_admin');
select create_role_if_not_exists('biology_staff');
select create_role_if_not_exists('computational_staff');

select create_role_if_not_exists('scamplers_api');
alter role scamplers_api with login;

select create_role_if_not_exists('scamplers_ui');
alter role scamplers_ui with createrole login;
