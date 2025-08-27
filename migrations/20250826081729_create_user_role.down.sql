-- Add down migration script here
DROP INDEX IF EXISTS idx_user_roles_user_id;

DROP INDEX IF EXISTS idx_user_roles_role_id;

DROP INDEX IF EXISTS idx_user_roles_user_id_role_id;

DROP INDEX IF EXISTS idx_user_roles_created_at;

DROP INDEX IF EXISTS idx_user_roles_updated_at;

DROP TABLE IF EXISTS "user_roles";