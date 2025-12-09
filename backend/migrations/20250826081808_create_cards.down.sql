-- Add down migration script here
DROP INDEX IF EXISTS idx_cards_card_number;

DROP INDEX IF EXISTS idx_cards_user_id;

DROP INDEX IF EXISTS idx_cards_card_type;

DROP INDEX IF EXISTS idx_cards_expire_date;

DROP INDEX IF EXISTS idx_cards_user_id_card_type;

DROP TABLE IF EXISTS "cards";