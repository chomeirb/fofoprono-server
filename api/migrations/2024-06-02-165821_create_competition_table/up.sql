-- Your SQL goes here
CREATE TABLE competitions (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL
);

INSERT INTO competitions (name) VALUES ('Worldcup 2022');
INSERT INTO competitions (name) VALUES ('Euro 2024');

-- Add competition_id column to games table with default value 1
ALTER TABLE games 
  ADD COLUMN competition_id INTEGER NOT NULL DEFAULT 1 REFERENCES competitions(id);