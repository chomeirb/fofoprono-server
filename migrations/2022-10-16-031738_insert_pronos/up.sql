-- Your SQL goes here
INSERT INTO pronos (user_id, game_id, prediction_home, prediction_away, result)
VALUES (1, 1, 0, 1, 'correct');

INSERT INTO pronos (user_id, game_id, prediction_home, prediction_away, result)
VALUES (1, 2, 1, 1, 'exact');

INSERT INTO pronos (user_id, game_id, prediction_home, prediction_away, result)
VALUES (1, 3, 3, 1, 'wrong');

INSERT INTO pronos (user_id, game_id, prediction_home, prediction_away, result)
VALUES (2, 1, 2, 1, null);