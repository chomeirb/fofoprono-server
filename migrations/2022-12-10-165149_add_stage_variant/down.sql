-- Dropping a type variant is not supported

-- ALTER TYPE STAGE RENAME TO STAGE_OLD;
-- CREATE TYPE STAGE AS ENUM ('group', 'sixteen', 'quarter', 'semi', 'final');
-- ALTER TABLE games ALTER stage TYPE STAGE USING stage::text::STAGE;
-- DROP TYPE STAGE;