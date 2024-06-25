use crate::config::database_connection;
use crate::schema::artists;
use crate::schema::artists::dsl::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::common::DBError;

#[derive(Queryable, Debug, Identifiable, Serialize)]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub started_at: Option<String>,
    pub origin_country: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = artists)]
pub struct ArtistInsertable<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub started_at: &'a str,
    pub origin_country: &'a str,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name=artists)]
pub struct ArtistUpdatable {
    pub name: Option<String>,
    pub started_at: Option<String>,
    pub description: Option<String>,
    pub origin_country: Option<String>,
}

pub struct ArtistsRepository;

impl ArtistsRepository {
    pub fn get_all_artists(&self) -> Result<Vec<Artist>, DBError> {
        
        let connection = &mut database_connection();
        artists.load::<Artist>(connection)
    }
    pub fn get_artist_by_id(&self, artist_id: i32) -> Result<Artist, DBError> {
        let connection = &mut database_connection();
        artists
            .filter(id.eq(artist_id))
            .first::<Artist>(connection)
    }

    pub fn pinsert_artist<'a>(
        &self,
        new_artist_name: &'a str,
        new_description: &'a str,
        country_code: &'a str,
        new_started_at: &'a str,
    ) -> Result<Artist, DBError> {
        let new_artist = ArtistInsertable {
            name: new_artist_name,
            description: new_description,
            origin_country: country_code,
            started_at: new_started_at,
        };
        let connection = &mut database_connection();
        diesel::insert_into(artists::table)
            .values(new_artist)
            .get_result::<Artist>(connection)
    }

    pub fn delete_artist_by_id(&self, artist_id: i32) -> bool {
        let connection = &mut database_connection();
        diesel::delete(artists.filter(id.eq(artist_id)))
            .execute(connection)
            .is_ok()
    }

    pub fn pupdate_artist<'a>(
        &self,
        artist_id: i32,
        new_artist_name: &'a str,
        new_description: &'a str,
        country_code: &'a str,
        new_started_at: &'a str,
    ) -> Result<Artist, DBError> {
        let updated_artist = ArtistInsertable {
            name: new_artist_name,
            description: new_description,
            origin_country: country_code,
            started_at: new_started_at,
        };
        let connection = &mut database_connection();
        diesel::update(artists)
            .filter(id.eq(artist_id))
            .set(updated_artist)
            .get_result(connection)
    }

    pub fn supdate_artist<'a>(
        &self,
        artist_id: i32,
        updated_artist: &ArtistUpdatable,
    ) -> Result<Artist, DBError> {
        let connection = &mut database_connection();

        diesel::update(artists)
            .filter(id.eq(artist_id))
            .set(updated_artist)
            .get_result(connection)
    }

    pub fn delete_all_artist(&self) -> bool {
        let connection = &mut database_connection();
        diesel::delete(artists).execute(connection).is_ok()
    }

    pub fn get_artist_by_name(&self, artist_name: &str) -> Result<Artist, DBError> {
        let connection = &mut database_connection();
        artists
            .filter(name.eq(artist_name))
            .first::<Artist>(connection)
    }
}
