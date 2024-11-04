-- Add migration script here
CREATE TABLE IF NOT EXISTS strategies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    strategy_type VARCHAR(40) NOT NULL,
    status VARCHAR(30) NOT NULL,
    next_time_execute INTEGER NOT NULL, -- Stored in seconds
    interval_time INTEGER NOT NULL, -- Stored in seconds
    account_private_key TEXT NOT NULL,
    token_address TEXT NOT NULL,
    price INTEGER NOT NULL,
    amount INTEGER NOT NULL,
    prioritization_fee INTEGER NOT NULL
    slippage INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_strategies_status_next_time_execute ON strategies(status, next_time_execute);

ALTER TABLE strategies ADD COLUMN txHash TEXT;
