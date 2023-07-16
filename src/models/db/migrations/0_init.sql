CREATE TABLE IF NOT EXISTS users (
  id TEXT PRIMARY KEY,
  username TEXT,
  pw TEXT
);

CREATE TABLE IF NOT EXISTS keys (
  id INT PRIMARY KEY,
  userid TEXT,
  key TEXT
);
