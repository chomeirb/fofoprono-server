-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,

  name VARCHAR(20) NOT NULL,
  mail VARCHAR(100) NOT NULL,
  password VARCHAR(20) NOT NULL,

  score INT NOT NULL DEFAULT 0,
  results_good INT NOT NULL DEFAULT 0,
  results_perfect INT NOT NULL DEFAULT 0,

  active BOOLEAN NOT NULL DEFAULT false,

  CONSTRAINT user_mail UNIQUE (mail)
);

CREATE TABLE games (
  id SERIAL PRIMARY KEY,
  time INTEGER NOT NULL,
  stage VARCHAR(20) NOT NULL,

  team_home VARCHAR(20) NOT NULL,
  team_away VARCHAR(20) NOT NULL,

  score_home INTEGER,
  score_away INTEGER,

  odds_home FLOAT NOT NULL,
  odds_away FLOAT NOT NULL,
  odds_draw FLOAT NOT NULL
);

CREATE TABLE pronos (
  PRIMARY KEY(user_id, game_id),

  user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  game_id INTEGER NOT NULL REFERENCES games(id),

  prediction_home INTEGER NOT NULL CHECK (prediction_home >= 0),
  prediction_away INTEGER NOT NULL CHECK (prediction_away >= 0),

  result TEXT NOT NULL
);

CREATE TABLE hashes (
  id TEXT PRIMARY KEY DEFAULT md5(random()::text),
  user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE
);

-- Create a trigger to compute the score of a user when a match result is updated
CREATE OR REPLACE FUNCTION update_score() RETURNS TRIGGER AS $$
BEGIN
  UPDATE users SET score = (
    SELECT COALESCE(SUM(
      CASE
        WHEN pronos.prediction_home = games.score_home AND pronos.prediction_away = games.score_away THEN 3
        WHEN pronos.prediction_home > pronos.prediction_away AND games.score_home > games.score_away THEN 1
        WHEN pronos.prediction_home < pronos.prediction_away AND games.score_home < games.score_away THEN 1
        WHEN pronos.prediction_home = pronos.prediction_away AND games.score_home = games.score_away THEN 1
        ELSE 0
      END
    ), 0) FROM pronos, games WHERE pronos.user_id = users.id AND pronos.game_id = games.id
  );

  UPDATE pronos SET result = (
    SELECT
      CASE
        WHEN pronos.prediction_home = games.score_home AND pronos.prediction_away = games.score_away THEN 'exact'
        WHEN pronos.prediction_home > pronos.prediction_away AND games.score_home > games.score_away THEN 'correct'
        WHEN pronos.prediction_home < pronos.prediction_away AND games.score_home < games.score_away THEN 'correct'
        WHEN pronos.prediction_home = pronos.prediction_away AND games.score_home = games.score_away THEN 'correct'
        ELSE 'wrong'
      END
    FROM games WHERE pronos.game_id = games.id
  );

  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_score
  AFTER UPDATE OF score_home, score_away
  ON games
  FOR EACH ROW
  EXECUTE PROCEDURE update_score();
