-- Enable UUID extension if not already enabled
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create the table 'users'
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    role SMALLINT NOT NULL DEFAULT 0,
    status SMALLINT NOT NULL DEFAULT 0,
    username VARCHAR(40) UNIQUE NOT NULL,
    display_name VARCHAR(40),
    email VARCHAR(100) UNIQUE NOT NULL,
    password TEXT NOT NULL,
    bio TEXT,
    avatar_url TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Create the function that updates the 'updated_at' field
CREATE FUNCTION update_timestamp() RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create the trigger that calls the function on every update
CREATE TRIGGER trigger_update_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

-- Create the function to insert a user, ensuring the username is unique
-- Modify the insert_user_if_unique function to handle bulk insert
CREATE OR REPLACE FUNCTION insert_users_if_unique(
    p_users users[]  -- Accept an array of users
) RETURNS VOID AS $$
DECLARE
    v_user users%ROWTYPE;
    v_nonce INT := 0;
BEGIN
    -- Loop through each user in the array
    FOREACH v_user IN ARRAY p_users LOOP
        BEGIN
            -- Attempt to insert the user directly
            INSERT INTO users
            VALUES (v_user.*);  -- Insert the entire user record

            -- If insertion is successful, exit
            RETURN;
        EXCEPTION WHEN unique_violation THEN
            -- Handle username conflict by appending a nonce
            IF (SQLSTATE = '23505' AND POSITION('users_username_key' IN SQLERRM) > 0) THEN
                v_nonce := v_nonce + 1;
                v_user.username := v_user.username || '_' || v_nonce;
                -- Retry insertion with the new username
                INSERT INTO users
                VALUES (v_user.*);
            ELSE
                -- If it's not a username conflict, raise an exception (likely an email conflict)
                RAISE EXCEPTION 'Conflict detected for user: %', v_user.email;
            END IF;
        END;
    END LOOP;
END;
$$ LANGUAGE plpgsql;