/*
TABLE FOR TEAMS

This table defines all the teams in the system.

This table stores the information about each team, including its owner, name, description, and
timestamps for when it was created and last updated.

- Primary Key: The `id` field is the primary key for this table, ensuring that each team has a
  unique identifier.
- Foreign Key: The `owner_id` field is a foreign key that references the `id` field of the `users`
  table, signifying the user who owns the team. If the referenced user is deleted, the
  corresponding team is also deleted (`ON DELETE CASCADE`).
- `created_at`: This field automatically gets set to the current timestamp when the team is
  created.

The `teams` table helps store the core information about teams and their relationship to the users
who own them. Any modifications to a team, such as changing the name or description, will
automatically update the `updated_at` timestamp.
*/
CREATE TABLE teams (
    id TEXT PRIMARY KEY NOT NULL,
    owner_id TEXT NOT NULL,
    team_name VARCHAR(40) NOT NULL,
    team_description TEXT,
    image_url TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
);

/*
TABLE FOR TEAM MEMBERSHIP RELATIONS

This table defines which users belong to which teams and their roles in the team.

This table helps track when a team's data was last modified, so the API knows when to refresh
cached data.
- Many-to-Many Relationship: A user can be in multiple teams, and a team can have multiple users.
- Composite Primary Key (team_id, user_id): Prevents duplicate memberships.
- ON DELETE CASCADE: If a team or user is deleted, their memberships are also deleted.
- Indexes: 
  - `team_id` and `user_id` are indexed to optimize queries that filter by these fields.
  - This helps when fetching all users in a team or all teams a user is part of.
*/
CREATE TABLE team_members (
    team_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    team_privilege INTEGER NOT NULL DEFAULT 0,

    PRIMARY KEY (team_id, user_id),
    FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_team_id ON team_members(team_id); -- Index to speed up queries by team_id filter
CREATE INDEX idx_user_id ON team_members(user_id); -- Index to speed up queries by user_id filter

/*
TABLE FOR TRACKING UPDATES

This table helps track when a team's data was last modified, so the API knows when to refresh
cached data.

- Separate table: Keeps update tracking clean and optimized.
- Requires logic: last_updated needs to be updated in API logic.
- ON DELETE CASCADE: If a team is deleted, its update record is also deleted.
*/
CREATE TABLE team_updates (
    team_id TEXT PRIMARY KEY NOT NULL,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,

    FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE CASCADE
);
