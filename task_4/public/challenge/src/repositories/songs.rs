use crate::config::database_connection;
use crate::schema::songs;
use crate::schema::songs::dsl::*;
use crate::schema::songs::{id, name};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Identifiable, Serialize)]
pub struct Song {
    pub id: i32,
    pub name: String,
    pub date_of_release: String,
    pub song_file: String,
    pub song_cover: Option<String>,
    pub artist_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = songs)]
pub struct SongInsertable<'a> {
    pub name: &'a str,
    pub date_of_release: &'a str,
    pub song_file: &'a str,
    pub song_cover: &'a str,
    pub artist_id: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = songs)]
pub struct SongUpdatable {
    pub name: Option<String>,
    pub date_of_release: Option<String>,
    pub song_file: Option<String>,
    pub song_cover: Option<String>,
    pub artist_id: Option<i32>,
}

type DBError = diesel::result::Error;

pub struct SongsRepository;

impl SongsRepository {
    pub fn get_all_songs(&self) -> Result<Vec<Song>, DBError> {
        let connection = &mut database_connection();
        songs.load::<Song>(connection)
    }
    pub fn get_song_by_id(&self, user_id: i32) -> Result<Song, DBError> {
        let connection = &mut database_connection();
        songs.filter(id.eq(user_id)).first::<Song>(connection)
    }

    pub fn sinsert_song(&self, song: &SongInsertable) -> Result<Song, DBError> {
        let connection = &mut database_connection();
        diesel::insert_into(songs::table)
            .values(song)
            .get_result::<Song>(connection)
    }
    pub fn pinsert_song<'a>(
        &self,
        new_name: &'a str,
        new_date_of_release: &'a str,
        new_file: &'a str,
        new_cover: &'a str,
        new_artist_id: i32,
    ) -> Result<Song, DBError> {
        let new_song = SongInsertable {
            name: new_name,
            date_of_release: new_date_of_release,
            song_file: new_file,
            song_cover: new_cover,
            artist_id: new_artist_id,
        };
        let connection = &mut database_connection();
        diesel::insert_into(songs::table)
            .values(new_song)
            .get_result::<Song>(connection)
    }

    pub fn delete_song_by_id(&self, song_id: i32) -> bool {
        let connection = &mut database_connection();
        diesel::delete(songs.filter(id.eq(song_id)))
            .execute(connection)
            .map(|rows_affected| if rows_affected > 0 { true } else { false })
            .unwrap()
    }

    pub fn supdate_song<'a>(
        &self,
        song_id: i32,
        updated_song: &SongUpdatable,
    ) -> Result<Song, DBError> {
        let connection = &mut database_connection();
        diesel::update(songs)
            .filter(id.eq(song_id))
            .set(updated_song)
            .get_result::<Song>(connection)
    }
    pub fn pupdate_song<'a>(
        &self,
        song_id: i32,
        new_name: &'a str,
        new_date_of_release: &'a str,
        new_file: &'a str,
        new_cover: &'a str,
        new_artist_id: i32,
    ) -> Result<Song, DBError> {
        let updated_song = SongInsertable {
            name: new_name,
            date_of_release: new_date_of_release,
            song_file: new_file,
            song_cover: new_cover,
            artist_id: new_artist_id,
        };
        let connection = &mut database_connection();
        diesel::update(songs)
            .filter(id.eq(song_id))
            .set(updated_song)
            .get_result::<Song>(connection)
    }

    pub fn delete_all_songs(&self) -> bool {
        let connection = &mut database_connection();
        diesel::delete(songs)
            .execute(connection)
            .map(|rows_affected| if rows_affected > 0 { true } else { false })
            .unwrap()
    }

    pub fn get_song_by_name(&self, song_name: &str) -> Result<Song, DBError> {
        let connection = &mut database_connection();
        songs.filter(name.eq(song_name)).first::<Song>(connection)
    }
}

