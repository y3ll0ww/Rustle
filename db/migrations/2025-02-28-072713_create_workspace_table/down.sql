/* -------------------------------------
   INDEXES
------------------------------------- */
DROP INDEX IF EXISTS idx_workspace_id;
DROP INDEX IF EXISTS idx_user;

/* -------------------------------------
   TRIGGERS
------------------------------------- */
DROP TRIGGER IF EXISTS trigger_update_workspaces_timestamp ON workspaces;
DROP TRIGGER IF EXISTS trigger_workspace_members_insert ON workspace_members;
DROP TRIGGER IF EXISTS trigger_workspace_members_delete ON workspace_members;
DROP TRIGGER IF EXISTS trigger_user_status_update ON users;

/* -------------------------------------
   FUNCTIONS
------------------------------------- */
DROP FUNCTION IF EXISTS touch_workspaces_updated_at;

/* -------------------------------------
   TABLES
------------------------------------- */
DROP TABLE IF EXISTS workspace_members;
DROP TABLE IF EXISTS workspaces;
