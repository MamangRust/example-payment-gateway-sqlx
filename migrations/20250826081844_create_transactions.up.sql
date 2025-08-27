-- Add up migration script here
CREATE TABLE "transactions" (
    "transaction_id" SERIAL PRIMARY KEY,
    "transaction_no" UUID NOT NULL DEFAULT gen_random_uuid (),
    "card_number" VARCHAR(16) NOT NULL REFERENCES "cards" ("card_number"),
    "amount" INT NOT NULL,
    "payment_method" VARCHAR(50) NOT NULL,
    "merchant_id" INT NOT NULL REFERENCES "merchants" ("merchant_id"),
    "transaction_time" TIMESTAMP NOT NULL,
    "status" VARCHAR(20) NOT NULL DEFAULT 'pending',
    "created_at" timestamp DEFAULT current_timestamp,
    "updated_at" timestamp DEFAULT current_timestamp,
    "deleted_at" TIMESTAMP DEFAULT NULL
);

CREATE INDEX idx_transactions_card_number ON transactions (card_number);

CREATE INDEX idx_transactions_merchant_id ON transactions (merchant_id);

CREATE INDEX idx_transactions_transaction_time ON transactions (transaction_time);

CREATE INDEX idx_transactions_payment_method ON transactions (payment_method);

CREATE INDEX idx_transactions_card_number_transaction_time ON transactions (card_number, transaction_time);