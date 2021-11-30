CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  username VARCHAR,
  password VARCHAR,
  is_active BOOLEAN NOT NULL DEFAULT 't',
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  is_admin BOOLEAN NOT NULL DEFAULT 'f',
  last_login TIMESTAMP,
  date_joined TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user_groups (
  user_id INT REFERENCES users(id) ON DELETE CASCADE,
  group_id INT REFERENCES groups(id) ON UPDATE CASCADE ON DELETE CASCADE,
  PRIMARY KEY(user_id, group_id)
);