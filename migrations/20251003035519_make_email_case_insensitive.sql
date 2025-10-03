-- Enable citext extension for case-insensitive text
CREATE EXTENSION IF NOT EXISTS citext;

-- Change email column to citext
ALTER TABLE users ALTER COLUMN email TYPE citext;

-- Recreate index (type change invalidates it)
DROP INDEX IF EXISTS idx_users_email;
CREATE INDEX idx_users_email ON users(email);
