# install

% cargo init diesel_sample

% vi Cargo.toml
[dependencies]
diesel = { version = "1.0.0", features = ["postgres"] }
dotenv = "0.9.0"

% cargo install diesel_cli

% echo DATABASE_URL=./sample.db > .env

% diesel setup

% diesel migration generate create_posts

% vi up.sql
CREATE TABLE posts (
  id INTEGER NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 0
)

% vi down.sql
DROP TABLE posts

% diesel migration run

% disel migration revert

% diesel migration redo

% vi main.rs

% vi models.rs

