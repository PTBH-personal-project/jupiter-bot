-- Add migration script here
CREATE TABLE tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    address TEXT NOT NULL UNIQUE,
    symbol TEXT,
    decimals INTEGER NOT NULL,
    name TEXT,
    logo_uri TEXT,
    uri TEXT NOT NULL
);
CREATE INDEX idx_tokens_address ON tokens(address);

