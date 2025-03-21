-- Create 'teams' table if not already created
CREATE TABLE IF NOT EXISTS teams (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    owner_id UUID NOT NULL,
    team_name VARCHAR(40) NOT NULL,
    team_description TEXT,
    image_url TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create 'team_members' table if not already created
CREATE TABLE IF NOT EXISTS team_members (
    team_id UUID NOT NULL,
    user_id UUID NOT NULL,
    team_role SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (team_id, user_id),
    FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create indexes on team_id and user_id for optimized queries
CREATE INDEX IF NOT EXISTS idx_team_id ON team_members(team_id);
CREATE INDEX IF NOT EXISTS idx_user_id ON team_members(user_id);

-- Create 'team_updates' table if not already created
CREATE TABLE IF NOT EXISTS team_updates (
    team_id UUID PRIMARY KEY,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE CASCADE
);
