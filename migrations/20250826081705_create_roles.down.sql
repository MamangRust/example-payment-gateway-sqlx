-- Add down migration script here
DROP INDEX IF EXISTS idx_roles_role_name;

DROP INDEX IF EXISTS idx_roles_created_at;

DROP INDEX IF EXISTS idx_roles_updated_at;

DROP TABLE IF EXISTS "roles";