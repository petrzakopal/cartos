CREATE TABLE user_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    card_serial_number TEXT NOT NULL,
    email TEXT NOT NULL,
    note TEXT,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO user_new (id, card_serial_number, email, note, updated_at)
SELECT id, card_serial_number, email, note, updated_at FROM user;

DROP TABLE user;

ALTER TABLE user_new RENAME TO user;

CREATE TRIGGER IF NOT EXISTS update_user_updated_at
AFTER UPDATE ON user
FOR EACH ROW
BEGIN
    UPDATE user
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;
