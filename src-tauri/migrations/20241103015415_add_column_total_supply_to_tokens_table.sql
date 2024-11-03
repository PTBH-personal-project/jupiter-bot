-- Add migration script here
ALTER TABLE tokens
ADD COLUMN total_supply VARCHAR(22);
