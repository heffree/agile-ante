CREATE TABLE IF NOT EXISTS rooms
(
  id                       INTEGER PRIMARY KEY NOT NULL,
  external_id              TEXT                NOT NULL,
  current_ticket_id        INTEGER                     ,
  host_player_device_id    TEXT                        
);


CREATE TABLE IF NOT EXISTS players 
(
  id                      INTEGER PRIMARY KEY NOT NULL,
  device_id               TEXT                NOT NULL,
  name                    TEXT                        
);


CREATE TABLE IF NOT EXISTS tickets 
(
  id                      INTEGER PRIMARY KEY NOT NULL,
  code                    TEXT                NOT NULL,
  description             TEXT                        
);

