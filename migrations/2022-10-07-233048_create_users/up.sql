-- Your SQL goes here
CREATE TABLE users (
  user_id SERIAL PRIMARY KEY,
  user_name VARCHAR(20) NOT NULL,
  user_mail VARCHAR(100) NOT NULL,
  user_password  VARCHAR(20) NOT NULL
)