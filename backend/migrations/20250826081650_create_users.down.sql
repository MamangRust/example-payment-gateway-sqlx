-- Add down migration script here
DROP INDEX IF EXISTS idx_users_email;

DROP INDEX IF EXISTS idx_users_firstname;

DROP INDEX IF EXISTS idx_users_lastname;

DROP INDEX IF EXISTS idx_users_firstname_lastname;

DROP INDEX IF EXISTS idx_users_created_at;

DROP TABLE IF EXISTS "users";