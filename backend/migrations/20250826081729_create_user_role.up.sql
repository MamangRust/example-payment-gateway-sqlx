-- Add up migration script here
CREATE TABLE IF NOT EXISTS "user_roles" (
    "user_role_id" SERIAL PRIMARY KEY,
    "user_id" INT NOT NULL REFERENCES "users" ("user_id") ON DELETE CASCADE,
    "role_id" INT NOT NULL REFERENCES "roles" ("role_id") ON DELETE CASCADE,
    "created_at" timestamp DEFAULT current_timestamp,
    "updated_at" timestamp DEFAULT current_timestamp,
    "deleted_at" TIMESTAMP DEFAULT NULL
);

CREATE INDEX idx_user_roles_user_id ON user_roles (user_id);

CREATE INDEX idx_user_roles_role_id ON user_roles (role_id);

CREATE INDEX idx_user_roles_user_id_role_id ON user_roles (user_id, role_id);

CREATE INDEX idx_user_roles_created_at ON user_roles (created_at);

CREATE INDEX idx_user_roles_updated_at ON user_roles (updated_at);