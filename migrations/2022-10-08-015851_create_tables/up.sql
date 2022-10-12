-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,

  name VARCHAR(20) NOT NULL,
  mail VARCHAR(100) NOT NULL,
  password VARCHAR(20) NOT NULL,

  score INT NOT NULL DEFAULT 0,
  results_good INT NOT NULL DEFAULT 0,
  results_perfect INT NOT NULL DEFAULT 0,

  active BOOLEAN NOT NULL DEFAULT false
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
  id SERIAL PRIMARY KEY,

  id_user INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  id_game INTEGER NOT NULL REFERENCES games(id),

  prediction_home INTEGER NOT NULL CHECK (prediction_home >= 0),
  prediction_away INTEGER NOT NULL CHECK (prediction_away >= 0)
);

CREATE TABLE hashes (
  id TEXT PRIMARY KEY DEFAULT md5(random()::text),
  id_user INTEGER NOT NULL REFERENCES users(id)
)