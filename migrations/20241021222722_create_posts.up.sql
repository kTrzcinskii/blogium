-- Add up migration script here
CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    -- https://www.sqlite.org/faq.html#q9 -> We can just use TEXT for any string-like field
    username TEXT NOT NULL,
    content TEXT NOT NULL,
    posted_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    image_path TEXT,
    avatar_path TEXT
)