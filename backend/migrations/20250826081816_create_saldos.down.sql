-- Add down migration script here
DROP INDEX IF EXISTS idx_saldos_card_number;

DROP INDEX IF EXISTS idx_saldos_withdraw_time;

DROP INDEX IF EXISTS idx_saldos_total_balance;

DROP INDEX IF EXISTS idx_saldos_card_number_withdraw_time;

DROP TABLE IF EXISTS "saldos";