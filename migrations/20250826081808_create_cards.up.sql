-- Add up migration script here
CREATE TABLE "cards" (
    "card_id" SERIAL PRIMARY KEY,
    "user_id" INT NOT NULL REFERENCES "users" ("user_id"),
    "card_number" VARCHAR(16) UNIQUE NOT NULL,
    "card_type" VARCHAR(50) NOT NULL,
    "expire_date" DATE NOT NULL,
    "cvv" VARCHAR(3) NOT NULL,
    "card_provider" VARCHAR(50) NOT NULL,
    "created_at" timestamp DEFAULT current_timestamp,
    "updated_at" timestamp DEFAULT current_timestamp,
    "deleted_at" TIMESTAMP DEFAULT NULL
);

CREATE INDEX idx_cards_card_number ON cards (card_number);

CREATE INDEX idx_cards_user_id ON cards (user_id);

CREATE INDEX idx_cards_card_type ON cards (card_type);

CREATE INDEX idx_cards_expire_date ON cards (expire_date);

CREATE INDEX idx_cards_user_id_card_type ON cards (user_id, card_type);