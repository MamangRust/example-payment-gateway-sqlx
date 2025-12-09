-- Add down migration script here
DROP INDEX IF EXISTS idx_withdraws_card_number;

DROP INDEX IF EXISTS idx_withdraws_withdraw_time;

DROP INDEX IF EXISTS idx_withdraws_withdraw_amount;

DROP INDEX IF EXISTS idx_withdraws_card_number_withdraw_time;

DROP TABLE IF EXISTS "withdraws";