pub(super) static GET_LATEST_MIGRATION: &str =
    "SELECT id FROM migrations ORDER BY id DESC LIMIT 1;";

pub(super) static ADD_MIGRATION: &str =
    "INSERT INTO migrations (id, date, query) VALUES (?, ?, ?);";

pub(super) static CREATE_MIGRATIONS_TABLE: &str = "CREATE TABLE IF NOT EXISTS migrations (
        id INT PRIMARY KEY,
        date TEXT,
        query TEXT
    );";

pub(super) static CREATE_USERS_TABLE: &str = "CREATE TABLE IF NOT EXISTS users (
        id TEXT PRIMARY KEY,
        username TEXT UNIQUE,
        hash TEXT,
        salt TEXT
    );";

pub(super) static CREATE_KEYS_TABLE: &str = "CREATE TABLE IF NOT EXISTS keys (
        id INT PRIMARY KEY,
        userid TEXT,
        key TEXT
        );";

pub(super) static CREATE_PROPOSALS_TABLE: &str = "CREATE TABLE IF NOT EXISTS proposals (
        id INT PRIMARY KEY,
        title TEXT,
        description TEXT,
        authorId TEXT,
        createdAt TEXT,
        updatedAt TEXT
    );";
