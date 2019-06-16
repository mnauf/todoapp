-- Your SQL goes here
CREATE TABLE tasks (
  done BOOLEAN NOT NULL DEFAULT 'f',
  id SERIAL PRIMARY KEY,
  task TEXT NOT NULL

)