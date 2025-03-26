-- Drop the team_updates table
DROP TABLE IF EXISTS team_updates;

-- Drop the team_members table
DROP INDEX IF EXISTS idx_team_id;
DROP INDEX IF EXISTS idx_user_id;
DROP TABLE IF EXISTS team_members;

-- Drop the teams table (it depends on team_members and team_updates, so drop them first)
DROP TABLE IF EXISTS teams;