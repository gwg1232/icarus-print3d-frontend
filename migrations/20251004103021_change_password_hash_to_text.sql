-- Change password_hash column from bytea to text
ALTER TABLE users ALTER COLUMN password_hash TYPE text USING convert_from(password_hash, 'UTF8');
