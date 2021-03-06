-- Rather than having users here, we will store group name with user directly, 
-- since user's group will be fetched more frequently than members of a particular group

CREATE TABLE IF NOT EXISTS groups (
  id INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS group_permissions (
    group_id INT REFERENCES groups(id) ON UPDATE CASCADE ON DELETE CASCADE,
    permission_key VARCHAR REFERENCES permission(key) ON UPDATE CASCADE ON DELETE CASCADE,
    enabled BOOLEAN NOT NULL DEFAULT 't',
    PRIMARY KEY (group_id, permission_key)
);
