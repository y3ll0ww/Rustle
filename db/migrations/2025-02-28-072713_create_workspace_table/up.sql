/* -------------------------------------
   TABLES
------------------------------------- */
-- Create 'workspace' table if not already created
CREATE TABLE workspaces (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    owner UUID NOT NULL,
    name VARCHAR(40) NOT NULL,
    description TEXT,
    image_url TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (owner) REFERENCES users(id) ON DELETE CASCADE
);

-- Create 'workspace_members' table if not already created
CREATE TABLE workspace_members (
    workspace UUID NOT NULL,
    member UUID NOT NULL,
    role SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (workspace, member),
    FOREIGN KEY (workspace) REFERENCES workspaces(id) ON DELETE CASCADE,
    FOREIGN KEY (member) REFERENCES users(id) ON DELETE CASCADE
);

/* -------------------------------------
   FUNCTIONS
------------------------------------- */
-- Create a function to edit updated_at of workspaces when something changes in workspace_members
CREATE FUNCTION touch_workspaces_updated_at() RETURNS TRIGGER AS $$
BEGIN
    -- If coming from workspace_members table
    IF TG_TABLE_NAME = 'workspace_members' THEN
        UPDATE workspaces SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.workspace;
    
    -- If coming from the users table
    ELSIF TG_TABLE_NAME = 'users' THEN
        UPDATE workspaces SET updated_at = CURRENT_TIMESTAMP WHERE id IN (
            SELECT workspace FROM workspace_members WHERE user = NEW.id
        );
    
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

/* -------------------------------------
   TRIGGERS
------------------------------------- */
-- Create a trigger for updating the 'workspaces' table
CREATE TRIGGER trigger_update_workspaces_timestamp
BEFORE UPDATE ON workspaces
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

-- Create triggers on the workspace_members table:
-- When a member is added
CREATE TRIGGER trigger_workspace_members_insert
AFTER INSERT ON workspace_members
FOR EACH ROW
EXECUTE FUNCTION touch_workspaces_updated_at();
-- When a member is removed
CREATE TRIGGER trigger_workspace_members_delete
AFTER DELETE ON workspace_members
FOR EACH ROW
EXECUTE FUNCTION touch_workspaces_updated_at();

-- Create a trigger on the users table
CREATE TRIGGER trigger_user_status_update
AFTER UPDATE OF status ON users
FOR EACH ROW
WHEN (OLD.status IS DISTINCT FROM NEW.status)
EXECUTE FUNCTION touch_workspaces_updated_at();

/* -------------------------------------
   INDEXES
------------------------------------- */
-- Create indexes on workspace ID and user ID for optimized queries
CREATE INDEX IF NOT EXISTS idx_workspace_id ON workspace_members(workspace);
CREATE INDEX IF NOT EXISTS idx_user_id ON workspace_members(member);
