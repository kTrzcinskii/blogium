-- Add up migration script here

CREATE TABLE images (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL,
    file_name TEXT
);

CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    -- https://www.sqlite.org/faq.html#q9 -> We can just use TEXT for any string-like field
    username TEXT NOT NULL,
    content TEXT NOT NULL,
    posted_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    post_image_id INTEGER,
    user_avatar_id INTEGER,
    FOREIGN KEY (post_image_id) REFERENCES images(id)
    FOREIGN KEY (user_avatar_id) REFERENCES images(id)
);