CREATE TABLE session (
    session_id VARCHAR(255) PRIMARY KEY,
    user_id INT UNIQUE NOT NULL,
    created_at DATETIME NOT NULL,
    FOREIGN KEY (user_id)
        REFERENCES user(user_id)
        ON DELETE CASCADE
);
