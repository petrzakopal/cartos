CREATE TABLE users_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    card_serial_number TEXT NOT NULL,
    email TEXT NOT NULL,
    note TEXT
);

INSERT INTO users_new (id, card_serial_number, email, note)
SELECT id, card_serial_number, email, note FROM users;

DROP TABLE users;

ALTER TABLE users_new RENAME TO users;

