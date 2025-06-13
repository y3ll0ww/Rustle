/* -------------------------------------
   INDEXES
------------------------------------- */
DROP INDEX IF EXISTS idx_project_id;
DROP INDEX IF EXISTS idx_project_member_id;

/* -------------------------------------
   TRIGGERS
------------------------------------- */
DROP TRIGGER IF EXISTS trigger_update_projects_timestamp ON projects;
DROP TRIGGER IF EXISTS trigger_project_members_change ON project_members;

/* -------------------------------------
   FUNCTIONS
------------------------------------- */
DROP FUNCTION IF EXISTS handle_project_membership_change;

/* -------------------------------------
   TABLES
------------------------------------- */
DROP TABLE IF EXISTS project_members;
DROP TABLE IF EXISTS projects;
