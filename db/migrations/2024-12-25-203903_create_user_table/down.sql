-- Drop the policies
DROP POLICY IF EXISTS access_password_hash_for_role_10 ON users;
DROP POLICY IF EXISTS user_general_access ON users;

-- Drop the trigger
DROP TRIGGER IF EXISTS trigger_update_timestamp ON users;

-- Drop the function
DROP FUNCTION IF EXISTS update_timestamp;

-- Drop the view
DROP VIEW IF EXISTS user_view;

-- Disable RLS on the users table
ALTER TABLE users DISABLE ROW LEVEL SECURITY;

-- Drop the users table
DROP TABLE IF EXISTS users;

-- Drop the UUID extension if you no longer need it (be careful with this if other tables depend on it)
DROP EXTENSION IF EXISTS "uuid-ossp";
