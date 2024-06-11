-- This file should undo anything in `up.sql`
ALTER TABLE games DROP COLUMN competition_id;
DROP TABLE competitions;
