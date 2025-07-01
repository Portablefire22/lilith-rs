-- Your SQL goes here
CREATE TABLE blog_posts (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  image VARCHAR,
  content_path VARCHAR NOT NULL,
  blog_finished BOOLEAN NOT NULL,
  project_finished BOOLEAN NOT NULL,
  hiatus_since DATETIME,
  modified DATETIME NOT NULL
);

CREATE TABLE blog_tags (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  tag VARCHAR NOT NULL,
  project INTEGER NOT NULL
)
