CREATE TABLE user_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    card_serial_number TEXT NOT NULL,
    email TEXT NOT NULL,
    note TEXT
);

INSERT INTO user_new (id, card_serial_number, email, note)
SELECT id, card_serial_number, email, note FROM user;

DROP TABLE user;

ALTER TABLE user_new RENAME TO user;

