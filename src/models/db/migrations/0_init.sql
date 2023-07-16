CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY,
  username TEXT,
  pw TEXT
);

CREATE TABLE IF NOT EXISTS keys (
  id UUID PRIMARY KEY,
  userid UUID,
  key JSONB
);