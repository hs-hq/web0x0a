-- Your SQL goes here
-- Your SQL goes here

CREATE TABLE IF NOT EXISTS users (
  id SERIAL  PRIMARY KEY , 
  username TEXT unique NOT NULL,
  email TEXT unique NOT NULL,
  password TEXT NOT NULL,
  role TEXT CHECK(role IN ('ADMIN','USER','PRODUCER')) DEFAULT 'USER' NOT NULL
);


CREATE TABLE IF NOT EXISTS artists (
  id  SERIAL PRIMARY KEY ,
  name VARCHAR(255) NOT NULL, 
  description TEXT NOT NULL, 
  started_at VARCHAR(255),
  origin_country VARCHAR(4) NOT NULL
);



CREATE TABLE IF NOT EXISTS songs (

  id SERIAL PRIMARY KEY ,
  name  VARCHAR(255) NOT NULL,
  date_of_release VARCHAR(10) NOT NULL,
  song_file VARCHAR(255) NOT NULL,
  song_cover TEXT ,
  artist_id INTEGER NOT NULL,

  CONSTRAINT artist_fk FOREIGN KEY(artist_id) REFERENCES artists(id) 
  ON DELETE CASCADE 
  ON UPDATE NO ACTION
);

CREATE TABLE IF NOT EXISTS artist_song (
artist_key  INTEGER NOT NULL,
song_key INTEGER NOT NULL,

PRIMARY KEY(artist_key,song_key),

CONSTRAINT artist_song_song_key FOREIGN KEY(song_key) 
  REFERENCES songs(id)  
    ON DELETE CASCADE 
    ON UPDATE CASCADE,

CONSTRAINT artist_song_key FOREIGN KEY(artist_key) REFERENCES artists(id) 
  ON DELETE CASCADE 
  ON UPDATE CASCADE
);
