-- Add up migration script here
CREATE TABLE "withdraws" (
    "withdraw_id" SERIAL PRIMARY KEY,
    "withdraw_no" UUID NOT NULL DEFAULT gen_random_uuid (),
    "card_number" VARCHAR(16) NOT NULL REFERENCES cards ("card_number"),
    "withdraw_amount" INT NOT NULL,
    "withdraw_time" TIMESTAMP NOT NULL,
    "status" VARCHAR(20) NOT NULL DEFAULT 'pending',
    "created_at" timestamp DEFAULT current_timestamp,
    "updated_at" timestamp DEFAULT current_timestamp,
    "deleted_at" TIMESTAMP DEFAULT NULL
);

CREATE INDEX idx_withdraws_card_number ON withdraws (card_number);

CREATE INDEX idx_withdraws_withdraw_time ON withdraws (withdraw_time);

CREATE INDEX idx_withdraws_withdraw_amount ON withdraws (withdraw_amount);

CREATE INDEX idx_withdraws_card_number_withdraw_time ON withdraws (card_number, withdraw_time);