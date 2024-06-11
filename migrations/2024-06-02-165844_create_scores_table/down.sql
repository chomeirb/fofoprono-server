-- This file should undo anything in `up.sql`
DROP TABLE scores;
DROP FUNCTION update_scores() CASCADE;

ALTER TABLE users ADD COLUMN score INT NOT NULL DEFAULT 0;
ALTER TABLE users ADD COLUMN results_perfect INT NOT NULL DEFAULT 0;
ALTER TABLE users ADD COLUMN results_good INT NOT NULL DEFAULT 0;

CREATE OR REPLACE FUNCTION update_score() RETURNS TRIGGER AS $$
BEGIN
  UPDATE users SET
  score = (
    SELECT COALESCE(SUM(
      CASE
        WHEN pronos.result = 'exact' THEN multiplier_odds(games.score_home, games.score_away, games.odds_home, games.odds_away, games.odds_draw) * multiplier_stage(games.stage) * 2
        WHEN pronos.result = 'correct' THEN multiplier_odds(games.score_home, games.score_away, games.odds_home, games.odds_away, games.odds_draw) * multiplier_stage(games.stage)
        ELSE 0
      END
    ), 0) FROM pronos, games WHERE pronos.user_id = users.id AND pronos.game_id = games.id
  ),
  results_perfect = (
    SELECT COALESCE(SUM(
      CASE
        WHEN pronos.result = 'exact' THEN 1
        ELSE 0
      END
    ), 0) FROM pronos WHERE pronos.user_id = users.id
  ),
  results_good = (
    SELECT COALESCE(SUM(
      CASE
        WHEN pronos.result = 'correct' THEN 1
        ELSE 0
      END
    ), 0) FROM pronos WHERE pronos.user_id = users.id
  );

  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_score
  AFTER UPDATE OF result ON pronos
  FOR EACH ROW EXECUTE PROCEDURE update_score();
