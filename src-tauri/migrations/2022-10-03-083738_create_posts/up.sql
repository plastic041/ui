CREATE TABLE posts (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  title TEXT NOT NULL
);

CREATE TABLE tags (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name TEXT NOT NULL,
  post_id INTEGER NOT NULL,
  FOREIGN KEY (post_id) REFERENCES posts(id)
);

INSERT INTO posts (id, title) VALUES (0, 'apple');
INSERT INTO posts (id, title) VALUES (1, 'banana');
INSERT INTO posts (id, title) VALUES (2, 'cherry');
INSERT INTO posts (id, title) VALUES (3, 'durian');
INSERT INTO posts (id, title) VALUES (4, 'elderberry');
