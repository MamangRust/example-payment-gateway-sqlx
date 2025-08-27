-- Add up migration script here
CREATE TABLE IF NOT EXISTS "users" (
    "user_id" serial PRIMARY KEY,
    "firstname" VARCHAR(100) NOT NULL,
    "lastname" varchar(100) NOT NULL,
    "email" varchar(100) UNIQUE NOT NULL,
    "password" varchar(100) NOT NULL,
    "created_at" timestamp DEFAULT current_timestamp,
    "updated_at" timestamp DEFAULT current_timestamp,
    "deleted_at" TIMESTAMP DEFAULT NULL
);

CREATE INDEX idx_users_email ON users (email);

CREATE INDEX idx_users_firstname ON users (firstname);

CREATE INDEX idx_users_lastname ON users (lastname);

CREATE INDEX idx_users_firstname_lastname ON users (firstname, lastname);

CREATE INDEX idx_users_created_at ON users (created_at);