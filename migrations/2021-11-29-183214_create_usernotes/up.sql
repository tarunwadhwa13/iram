CREATE TABLE IF NOT EXISTS user_notes (
  id SERIAL PRIMARY KEY,
  note TEXT,
  user_id INT REFERENCES users(id) ON DELETE CASCADE,
  alert_event_id INT REFERENCES alert_event(id) ON DELETE CASCADE,
  visibility VARCHAR NOT NULL CHECK (visibility IN ('Public', 'Private'))
);