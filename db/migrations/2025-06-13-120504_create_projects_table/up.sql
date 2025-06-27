/* -------------------------------------
   TABLES
------------------------------------- */
-- Table for storing information about a project
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workspace UUID NOT NULL,
    name VARCHAR(40) NOT NULL,
    description TEXT,
    member_count INTEGER NOT NULL DEFAULT 0 CHECK (member_count >= 0),
    image_url TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (workspace) REFERENCES workspaces(id) ON DELETE CASCADE
);

-- Table for linking projects to users
CREATE TABLE project_members (
    project UUID NOT NULL,
    member UUID NOT NULL,
    role SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (project, member),
    FOREIGN KEY (project) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (member) REFERENCES users(id) ON DELETE CASCADE
);

/* -------------------------------------
   FUNCTIONS
------------------------------------- */
-- Function for editing the updated_at adn the member_count field of the projects table when
-- something changes in the project_members table
CREATE FUNCTION handle_project_membership_change() RETURNS TRIGGER AS $$
BEGIN
    -- Handle member count update
    IF TG_OP = 'INSERT' THEN
        UPDATE projects
        SET member_count = member_count + 1, updated_at = CURRENT_TIMESTAMP
        WHERE id = NEW.project;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE projects
        SET member_count = member_count - 1, updated_at = CURRENT_TIMESTAMP
        WHERE id = OLD.project;
    END IF;

    -- Add update timestamp
    UPDATE projects
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = COALESCE(NEW.project, OLD.project);

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

/* -------------------------------------
   TRIGGERS
------------------------------------- */
-- Trigger for updating the updated_at field in the projects table
CREATE TRIGGER trigger_update_projects_timestamp
BEFORE UPDATE ON projects
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

-- Trigger for updating the updated_at and the members_count field in the projects table, based
-- on changes in the project_members table:
CREATE TRIGGER trigger_project_members_change
AFTER INSERT OR DELETE ON project_members
FOR EACH ROW
EXECUTE FUNCTION handle_project_membership_change();

/* -------------------------------------
   INDEXES
------------------------------------- */
-- Indexes on project ID and user ID for optimized queries
CREATE INDEX IF NOT EXISTS idx_project_id ON project_members(project);
CREATE INDEX IF NOT EXISTS idx_project_member_id ON project_members(member);
