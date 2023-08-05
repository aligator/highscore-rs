CREATE TABLE highscore (
    id INTEGER PRIMARY KEY NOT NULL, -- an INTEGER PRIMARY KEY will auto-increment in Sqlite.
    name VARCHAR(255) NOT NULL,
    score REAL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)