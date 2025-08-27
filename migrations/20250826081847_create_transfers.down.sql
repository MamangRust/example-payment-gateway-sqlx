-- Add down migration script here
DROP INDEX IF EXISTS idx_transfers_transfer_from;

DROP INDEX IF EXISTS idx_transfers_transfer_to;

DROP INDEX IF EXISTS idx_transfers_transfer_time;

DROP INDEX IF EXISTS idx_transfers_transfer_amount;

DROP INDEX IF EXISTS idx_transfers_transfer_from_transfer_time;

DROP TABLE IF EXISTS "transfers";