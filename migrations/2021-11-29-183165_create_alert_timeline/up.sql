CREATE TABLE IF NOT EXISTS alert_timeline (
    id INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    event_type VARCHAR NOT NULL,
    reported_at TIMESTAMPTZ NOT NULL,
    event_info VARCHAR NOT NULL
)