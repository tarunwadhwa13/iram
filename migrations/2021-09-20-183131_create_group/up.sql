-- Rather than having users here, we will store group name with user directly, 
-- since user's group will be fetched more frequently than members of a particular group

CREATE TABLE IF NOT EXISTS groups (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS group_permissions (
    group_id INT REFERENCES groups(id) ON UPDATE CASCADE ON DELETE CASCADE,
    permission VARCHAR REFERENCES user_permissions(permission) ON UPDATE CASCADE ON DELETE CASCADE,
    enabled BOOLEAN NOT NULL DEFAULT 't',
    PRIMARY KEY (group_id, permission)
);
