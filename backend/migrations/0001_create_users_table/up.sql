/* -------------------------------------
   EXTENSIONS
------------------------------------- */
-- Enable UUID extension if not already enabled
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

/* -------------------------------------
   TABLES
------------------------------------- */
-- Create the table 'users'
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(40) UNIQUE NOT NULL,
    first_name VARCHAR(20) NOT NULL,
    last_name VARCHAR(20) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    phone VARCHAR(20) UNIQUE,
    role SMALLINT NOT NULL DEFAULT 0,
    status SMALLINT NOT NULL DEFAULT 0,
    job_title VARCHAR(20),
    password TEXT NOT NULL,
    bio TEXT,
    avatar_url TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

/* -------------------------------------
   FUNCTIONS
------------------------------------- */
-- Create the function that updates the 'updated_at' field
CREATE FUNCTION update_timestamp() RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

/* -------------------------------------
   TRIGGERS
------------------------------------- */
-- Create the trigger that calls the function on every update
CREATE TRIGGER trigger_update_users_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

/* -------------------------------------
   INDEXES
------------------------------------- */
-- Adding indexation for the following fields
CREATE INDEX idx_username_lower ON users (LOWER(username));
CREATE INDEX idx_first_name_lower ON users (LOWER(first_name));
CREATE INDEX idx_last_name_lower ON users (LOWER(last_name));
CREATE INDEX idx_email_lower ON users (LOWER(email));
CREATE INDEX idx_phone ON users (phone);
