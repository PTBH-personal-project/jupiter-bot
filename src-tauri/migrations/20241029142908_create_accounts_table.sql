-- Add migration script here
CREATE TABLE accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    private_key TEXT NOT NULL UNIQUE,
    public_key TEXT NOT NULL,
    description TEXT,
    status varchar(30) NOT NULL
);