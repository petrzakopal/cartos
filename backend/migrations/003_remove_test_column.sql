CREATE TABLE users_new AS SELECT id, cardSerialNumber, email FROM users;

DROP TABLE users;

ALTER TABLE users_new RENAME TO users;
