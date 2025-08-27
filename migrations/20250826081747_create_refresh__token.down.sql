-- Add down migration script here
DROP INDEX IF EXISTS idx_refresh_tokens_user_id;

DROP INDEX IF EXISTS idx_refresh_tokens_token;

DROP INDEX IF EXISTS idx_refresh_tokens_expiration;

DROP TABLE IF EXISTS refresh_tokens;