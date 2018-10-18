CREATE TABLE posts_tags (
  id INTEGER NOT NULL PRIMARY KEY,
  post_id INTEGER NOT NULL,
  tag_id INTEGER NOT NULL,
  FOREIGN KEY(post_id) REFERENCES posts(id),
  FOREIGN KEY(tag_id) REFERENCES tags(id)
);
