-- Your SQL goes here
CREATE TABLE scores (
  PRIMARY KEY(user_id, competition_id),
  user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  competition_id INTEGER NOT NULL REFERENCES competitions(id),

  points INT NOT NULL DEFAULT 0,
  good INT NOT NULL DEFAULT 0,
  perfect INT NOT NULL DEFAULT 0
);

-- Update scores by inserting the row corresponding to the user and the competition or updating the existing one if it already exists.
CREATE OR REPLACE FUNCTION update_scores() RETURNS TRIGGER AS $$
DECLARE
  comp_id INTEGER;
BEGIN
  -- First, determine the competition_id for the new game
  SELECT games.competition_id INTO comp_id FROM games WHERE games.id = NEW.game_id;

  -- Then, use this competition_id to filter pronos in the subqueries
  INSERT INTO scores (user_id, competition_id, points, perfect, good) VALUES (
    NEW.user_id,
    comp_id,
    (
      SELECT COALESCE(SUM(
        CASE
          WHEN pronos.result = 'exact' THEN multiplier_odds(games.score_home, games.score_away, games.odds_home, games.odds_away, games.odds_draw) * multiplier_stage(games.stage) * 2
          WHEN pronos.result = 'correct' THEN multiplier_odds(games.score_home, games.score_away, games.odds_home, games.odds_away, games.odds_draw) * multiplier_stage(games.stage)
          ELSE 0
        END
      ), 0) FROM pronos, games WHERE pronos.user_id = NEW.user_id AND pronos.game_id = games.id AND games.competition_id = comp_id
    ),
    (
      SELECT COALESCE(SUM(
        CASE
          WHEN pronos.result = 'exact' THEN 1
          ELSE 0
        END
      ), 0) FROM pronos, games WHERE pronos.user_id = NEW.user_id AND pronos.game_id = games.id AND games.competition_id = comp_id
    ),
    (
      SELECT COALESCE(SUM(
        CASE
          WHEN pronos.result = 'correct' THEN 1
          ELSE 0
        END
      ), 0) FROM pronos, games WHERE pronos.user_id = NEW.user_id AND pronos.game_id = games.id AND games.competition_id = comp_id
    )
  )
  ON CONFLICT (user_id, competition_id) DO UPDATE SET
    points = EXCLUDED.points,
    perfect = EXCLUDED.perfect,
    good = EXCLUDED.good;

  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_scores
  AFTER UPDATE OF result ON pronos
  FOR EACH ROW EXECUTE PROCEDURE update_scores();

ALTER TABLE users DROP COLUMN score;
ALTER TABLE users DROP COLUMN results_perfect;
ALTER TABLE users DROP COLUMN results_good;

DROP FUNCTION update_score() CASCADE;
