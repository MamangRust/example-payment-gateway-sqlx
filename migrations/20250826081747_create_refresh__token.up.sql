-- Add up migration script here
CREATE TABLE refresh_tokens (
    refresh_token_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users (user_id) ON DELETE CASCADE,
    token VARCHAR(255) NOT NULL UNIQUE,
    expiration TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT current_timestamp,
    updated_at TIMESTAMP DEFAULT current_timestamp,
    deleted_at TIMESTAMP DEFAULT NULL
);

CREATE INDEX idx_refresh_tokens_user_id ON refresh_tokens (user_id);

CREATE INDEX idx_refresh_tokens_token ON refresh_tokens (token);

CREATE INDEX idx_refresh_tokens_expiration ON refresh_tokens (expiration);