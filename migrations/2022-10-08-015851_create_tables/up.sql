-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(20) NOT NULL,
  mail VARCHAR(100) NOT NULL,
  password  VARCHAR(20) NOT NULL,
  score INT NOT NULL DEFAULT 0,
  goodResult INT NOT NULL DEFAULT 0,
  perfectResult INT NOT NULL DEFAULT 0
);

CREATE TABLE games (
  id SERIAL PRIMARY KEY,
  homeTeam VARCHAR(20) NOT NULL,
  awayTeam VARCHAR(20) NOT NULL,
  homeScore INTEGER,
  awayScore INTEGER,
  time INTEGER NOT NULL,
  stage VARCHAR(20) NOT NULL,
  homeOdds FLOAT NOT NULL,
  awayOdds FLOAT NOT NULL,
  drawOdds FLOAT NOT NULL
);

CREATE TABLE pronos (
  id SERIAL PRIMARY KEY,
  fk_gameId INTEGER NOT NULL REFERENCES games(id),
  fk_userId INTEGER NOT NULL REFERENCES users(id),
  homePrediction INTEGER NOT NULL CHECK (homePrediction >= 0),
  awayPrediction INTEGER NOT NULL CHECK (awayPrediction >= 0)
);


