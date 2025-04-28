/* -------------------------------------
   INDEXES
------------------------------------- */
DROP INDEX IF EXISTS idx_workspace_id;
DROP INDEX IF EXISTS idx_user;

/* -------------------------------------
   TRIGGERS
------------------------------------- */
DROP TRIGGER IF EXISTS trigger_update_workspaces_timestamp ON workspaces;
DROP TRIGGER IF EXISTS trigger_workspace_members_change ON workspace_members;

/* -------------------------------------
   FUNCTIONS
------------------------------------- */
DROP FUNCTION IF EXISTS handle_workspace_membership_change;

/* -------------------------------------
   TABLES
------------------------------------- */
DROP TABLE IF EXISTS workspace_members;
DROP TABLE IF EXISTS workspaces;
