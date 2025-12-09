-- Add down migration script here
DROP INDEX IF EXISTS idx_topups_card_number;

DROP INDEX IF EXISTS idx_topups_topup_no;

DROP INDEX IF EXISTS idx_topups_topup_time;

DROP INDEX IF EXISTS idx_topups_topup_method;

DROP INDEX IF EXISTS idx_topups_card_number_topup_time;

DROP TABLE IF EXISTS "topups";