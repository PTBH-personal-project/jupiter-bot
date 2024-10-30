-- Add migration script here
-- Add migration script here
CREATE TABLE rpcs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    url TEXT NOT NULL,
    description TEXT
);

INSERT INTO rpcs (name, url, description) VALUES ('Public', 'https://api.mainnet-beta.solana.com', 'This is the public and free RPC endpoint for Solana');