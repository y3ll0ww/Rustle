/* -------------------------------------
   INDEXES
------------------------------------- */
DROP INDEX IF EXISTS idx_username_lower;
DROP INDEX IF EXISTS idx_first_name_lower;
DROP INDEX IF EXISTS idx_last_name_lower;
DROP INDEX IF EXISTS idx_email_lower;
DROP INDEX IF EXISTS idx_phone;

/* -------------------------------------
   TRIGGERS
------------------------------------- */
DROP TRIGGER IF EXISTS trigger_update_users_timestamp ON users;

/* -------------------------------------
   FUNCTIONS
------------------------------------- */
DROP FUNCTION IF EXISTS update_timestamp;

/* -------------------------------------
   TABLES
------------------------------------- */
DROP TABLE IF EXISTS users;

/* -------------------------------------
   EXTENSIONS
------------------------------------- */
DROP EXTENSION IF EXISTS "uuid-ossp";
