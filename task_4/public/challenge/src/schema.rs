// @generated automatically by Diesel CLI.

diesel::table! {
    artist_song (artist_key, song_key) {
        artist_key -> Int4,
        song_key -> Int4,
    }
}

diesel::table! {
    artists (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        started_at -> Nullable<Varchar>,
        origin_country -> Varchar,
    }
}

diesel::table! {
    songs (id) {
        id -> Int4,
        name -> Varchar,
        date_of_release -> Varchar,
        song_file -> Varchar,
        song_cover -> Nullable<Text>,
        artist_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        password -> Text,
        role -> Text,
    }
}

diesel::joinable!(artist_song -> artists (artist_key));
diesel::joinable!(artist_song -> songs (song_key));
diesel::joinable!(songs -> artists (artist_id));

diesel::allow_tables_to_appear_in_same_query!(artist_song, artists, songs, users,);
