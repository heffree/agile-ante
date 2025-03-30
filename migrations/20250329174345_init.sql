CREATE TABLE IF NOT EXISTS rooms
(
  id                       INTEGER PRIMARY KEY NOT NULL,
  external_id              TEXT                NOT NULL,
  current_ticket_id        INTEGER            
);
