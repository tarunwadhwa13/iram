CREATE TABLE IF NOT EXISTS incident_report (
  id INT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
  segments_lost NUMERIC(2) NOT NULL,
  loss_details VARCHAR NOT NULL,
  cost NUMERIC(2) NOT NULL,
  acked_at TIMESTAMPTZ NOT NULL,
  resolved_at TIMESTAMPTZ,
  status VARCHAR NOT NULL,
  resolution VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL default (now() at time zone 'utc'),
  last_updated TIMESTAMPTZ NOT NULL default (now() at time zone 'utc')
);

CREATE TABLE IF NOT EXISTS incident_alert (
  incident_id INT REFERENCES incident_report(id) ON DELETE CASCADE,
  alert_id INT REFERENCES alerts(id) ON DELETE CASCADE,
  PRIMARY KEY (incident_id, alert_id)
);