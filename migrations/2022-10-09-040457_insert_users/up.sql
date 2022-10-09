-- Your SQL goes here
INSERT INTO users (name, mail, password) 
    SELECT CONCAT('MyName', id), CONCAT('mymail@mail.com', id), CONCAT('MyPassword', id)
    FROM generate_series(1, 50) id;