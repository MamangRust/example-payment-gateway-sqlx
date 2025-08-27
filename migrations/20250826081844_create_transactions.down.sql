-- Add down migration script here
DROP INDEX IF EXISTS idx_transactions_card_number;

DROP INDEX IF EXISTS idx_transactions_merchant_id;

DROP INDEX IF EXISTS idx_transactions_transaction_time;

DROP INDEX IF EXISTS idx_transactions_payment_method;

DROP INDEX IF EXISTS idx_transactions_card_number_transaction_time;

DROP TABLE IF EXISTS "transactions";