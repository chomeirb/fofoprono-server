-- Your SQL goes here
INSERT INTO users (name, mail, password, active)
    SELECT CONCAT('id', id), CONCAT('mymail@mail.com', id), CONCAT('pw', id), true
    FROM generate_series(1, 20) id;