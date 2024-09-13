-- Your SQL goes here
CREATE TABLE selectors (
  id UUID PRIMARY KEY,
  function_name VARCHAR NOT NULL,
  felt_selector TEXT NOT NULL,
  selector VARCHAR NOT NULL
)