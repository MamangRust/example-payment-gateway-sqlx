-- Add down migration script here
DROP INDEX IF EXISTS idx_merchants_api_key;

DROP INDEX IF EXISTS idx_merchants_user_id;

DROP INDEX IF EXISTS idx_merchants_status;

DROP INDEX IF EXISTS idx_merchants_name;

DROP INDEX IF EXISTS idx_merchants_user_id_status;

DROP TABLE IF EXISTS "merchants";