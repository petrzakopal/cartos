CREATE TABLE IF NOT EXISTS user (
id INTEGER PRIMARY KEY AUTOINCREMENT,
card_serial_number TEXT NOT NULL,
email TEXT NOT NULL,
note TEXT,
status TEXT,
updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS log (
id INTEGER PRIMARY KEY AUTOINCREMENT,
timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
card_serial_number TEXT NOT NULL,
email TEXT NOT NULL,
result TEXT,
note TEXT
);

CREATE TRIGGER IF NOT EXISTS update_user_updated_at
AFTER UPDATE ON user
FOR EACH ROW
BEGIN
    UPDATE user
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;
