-- Your SQL goes here
CREATE TABLE IF NOT EXISTS notification_channel (
  id INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  destination VARCHAR NOT NULL,
  settings JSONB DEFAULT '{}'::jsonb NOT NULL,
  active_start_time TIMESTAMPTZ NOT NULL,
  active_end_time TIMESTAMPTZ NOT NULL,
  enabled BOOLEAN NOT NULL DEFAULT 't'
);