CREATE TABLE posts (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  title TEXT NOT NULL
);

CREATE TABLE tags (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name TEXT NOT NULL
);

CREATE TABLE post_tags (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  post_id INTEGER NOT NULL,
  tag_id INTEGER NOT NULL,
  FOREIGN KEY (post_id) REFERENCES posts(id),
  FOREIGN KEY (tag_id) REFERENCES tags(id)
);


INSERT INTO posts (id, title) VALUES (0, 'batman');
INSERT INTO posts (id, title) VALUES (1, 'joker');
INSERT INTO posts (id, title) VALUES (2, 'captain marvel');
INSERT INTO posts (id, title) VALUES (3, 'suicide squad');

INSERT INTO tags (id, name) VALUES (0, 'hero');
INSERT INTO tags (id, name) VALUES (1, 'male');
INSERT INTO tags (id, name) VALUES (2, 'villain');
INSERT INTO tags (id, name) VALUES (3, 'female');

INSERT INTO post_tags (id, post_id, tag_id) VALUES (0, 0, 0);
INSERT INTO post_tags (id, post_id, tag_id) VALUES (1, 0, 1);
INSERT INTO post_tags (id, post_id, tag_id) VALUES (2, 1, 1);
INSERT INTO post_tags (id, post_id, tag_id) VALUES (3, 1, 2);
INSERT INTO post_tags (id, post_id, tag_id) VALUES (4, 2, 0);
INSERT INTO post_tags (id, post_id, tag_id) VALUES (5, 2, 3);
INSERT INTO post_tags (id, post_id, tag_id) VALUES (6, 3, 2);
INSERT INTO post_tags (id, post_id, tag_id) VALUES (7, 3, 3);
