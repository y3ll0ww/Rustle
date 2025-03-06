CREATE TABLE users (
    id TEXT PRIMARY KEY NOT NULL,
    privilege INTEGER NOT NULL DEFAULT 0,
    username VARCHAR(40) UNIQUE NOT NULL,
    display_name VARCHAR(40),
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    bio TEXT,
    avatar_url TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
