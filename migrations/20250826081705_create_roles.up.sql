-- Add up migration script here
CREATE TABLE IF NOT EXISTS "roles" (
    "role_id" SERIAL PRIMARY KEY,
    "role_name" VARCHAR(50) UNIQUE NOT NULL,
    "created_at" timestamp DEFAULT current_timestamp,
    "updated_at" timestamp DEFAULT current_timestamp,
    "deleted_at" TIMESTAMP DEFAULT NULL
);

CREATE INDEX idx_roles_role_name ON roles (role_name);

CREATE INDEX idx_roles_created_at ON roles (created_at);

CREATE INDEX idx_roles_updated_at ON roles (updated_at);