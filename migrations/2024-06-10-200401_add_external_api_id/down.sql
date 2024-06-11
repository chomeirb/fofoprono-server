-- This file should undo anything in `up.sql`
ALTER TABLE games DROP COLUMN external_api_id;

ALTER TABLE games ALTER COLUMN odds_home SET NOT NULL;
ALTER TABLE games ALTER COLUMN odds_away SET NOT NULL;
ALTER TABLE games ALTER COLUMN odds_draw SET NOT NULL;
