/* -------------------------------------
   TABLES
------------------------------------- */
-- Table for storing information about a workspace
CREATE TABLE workspaces (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    owner UUID NOT NULL,
    name VARCHAR(40) NOT NULL,
    description TEXT,
    member_count INTEGER NOT NULL DEFAULT 0 CHECK (member_count >= 0),
    image_url TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (owner) REFERENCES users(id) ON DELETE CASCADE
);

-- Table for linking workspaces to users
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
-- Function for editing the updated_at adn the member_count field of the workspaces table when
-- something changes in the workspace_members table
CREATE FUNCTION handle_workspace_membership_change() RETURNS TRIGGER AS $$
BEGIN
    -- Handle member count update
    IF TG_OP = 'INSERT' THEN
        UPDATE workspaces
        SET member_count = member_count + 1, updated_at = CURRENT_TIMESTAMP
        WHERE id = NEW.workspace;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE workspaces
        SET member_count = member_count - 1, updated_at = CURRENT_TIMESTAMP
        WHERE id = OLD.workspace;
    END IF;

    -- Add update timestamp
    UPDATE workspaces
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = COALESCE(NEW.workspace, OLD.workspace);

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

/* -------------------------------------
   TRIGGERS
------------------------------------- */
-- Trigger for updating the updated_at field in the workspaces table
CREATE TRIGGER trigger_update_workspaces_timestamp
BEFORE UPDATE ON workspaces
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

-- Trigger for updating the updated_at and the members_count field in the workspaces table, based
-- on changes in the workspace_members table:
CREATE TRIGGER trigger_workspace_members_change
AFTER INSERT OR DELETE ON workspace_members
FOR EACH ROW
EXECUTE FUNCTION handle_workspace_membership_change();

/* -------------------------------------
   INDEXES
------------------------------------- */
-- Indexes on workspace ID and user ID for optimized queries
CREATE INDEX IF NOT EXISTS idx_workspace_id ON workspace_members(workspace);
CREATE INDEX IF NOT EXISTS idx_user_id ON workspace_members(member);
