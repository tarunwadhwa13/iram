-- Create a custom type when diesel supports. Currently it doesn't support this
-- CREATE TYPE ALERTSOURCECHOICES AS ENUM ('Zabbix', 'ApplicationManager', 'NewRelic');
-- CREATE TYPE AUTHCHOICES AS ENUM ('None', 'APIKey', 'BasicAuth');


CREATE TABLE IF NOT EXISTS alert_source_info (
  id SERIAL PRIMARY KEY,
  source_type VARCHAR NOT NULL CHECK (source_type IN ('Zabbix', 'ApplicationManager', 'NewRelic')),
  identifier VARCHAR NOT NULL UNIQUE,
  connect_url VARCHAR NOT NULL,
  auth_type VARCHAR NOT NULL CHECK (auth_type in ('None', 'APIKey', 'BasicAuth')),
  connection_params json NOT NULL,
  enabled BOOLEAN NOT NULL DEFAULT 't'
)