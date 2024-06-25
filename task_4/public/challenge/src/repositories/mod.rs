use std::env;

use pwhash::bcrypt;

use crate::{handlers::common::UserStruct, utils::save_user};

use self::{artists::ArtistsRepository, songs::SongsRepository, users::UsersRepository};

pub mod artists;
mod common;
pub mod songs;
pub mod users;

pub fn seed() {
    let songs_repository = SongsRepository {};
    let artists_repository = ArtistsRepository {};
    let users_repository = UsersRepository {};

    users_repository.delete_all_users_except_admin();
    songs_repository.delete_all_songs();
    artists_repository.delete_all_artist();

    users_repository
        .pinsert_user(
            "admin",
            &bcrypt::hash(env::var("ADMIN_PASSWORD").expect("Error Admin password not set"))
                .unwrap(),
            "admin@gmail.com",
            "ADMIN",
        )
        .unwrap();

    let admin_struct = UserStruct {
        email: "admin@gmail.com".to_string(),
        username: "admin".to_string(),
    };

    save_user(&admin_struct).expect("Error saving admin user");

    let first_artist=artists_repository.pinsert_artist("Kendrick Lamar", "Kendrick Lamar Duckworth (born June 17, 1987) is an American rapper and songwriter. Known for his progressive musical styles and socially conscious songwriting, he is often considered one of the most influential hip hop artists of his generation.[1][2] Born and raised in Compton, California, Lamar began his career as a teenager performing under the stage name K.Dot. He quickly garnered local attention which led to him signing a recording contract with Top Dawg Entertainment (TDE) in 2005", "USA", "REDACTED").unwrap();
    let second_artist=artists_repository.pinsert_artist("Pink Floyd", "Pink Floyd are an English rock band formed in London in 1965. Gaining an early following as one of the first British psychedelic groups, they were distinguished by their extended compositions, sonic experimentation, philosophical lyrics and elaborate live shows. They became a leading band of the progressive rock genre, cited by some as the greatest progressive rock band of all time. ", "UK", "REDACTED").unwrap();
    let third_artist= artists_repository.pinsert_artist("Radiohead", "Radiohead are an English rock band formed in Abingdon, Oxfordshire, in 1985. The band consists of Thom Yorke (vocals, guitar, piano, keyboards); brothers Jonny Greenwood (lead guitar, keyboards, other instruments) and Colin Greenwood (bass); Ed O'Brien (guitar, backing vocals); and Philip Selway (drums, percussion). They have worked with the artist Nigel Godrich and the cover artist Stanley Donwood since 1994. Radiohead's experimental approach is credited with advancing the sound of alternative rock.  ", "UK", "REDACTED").unwrap();
    let fourth_artist=artists_repository.pinsert_artist("King Crismon", "King Crimson are a progressive rock band formed in 1968 in London, England. The band draws inspiration from a wide variety of music, incorporating elements of classical, jazz, folk, heavy metal, gamelan, industrial, electronic, experimental music and new wave. They exerted a strong influence on the early 1970s progressive rock movement, including on contemporaries such as Yes and Genesis, and continue to inspire subsequent generations of artists across multiple genres.[1] The band has earned a large cult following", "UK", "REDACTED").unwrap();

    songs_repository
        .pinsert_song(
            "u",
            "2015",
            "u - Kendrick Lamar (To Pimp a Butterfly).mp3",
            "save_buttefly.webp",
            first_artist.id,
        )
        .unwrap();
    songs_repository
        .pinsert_song(
            "Wish you were here",
            "1975",
            "Pink Floyd - Wish You Were Here.mp3",
            "wish_you_were_here.webp",
            second_artist.id,
        )
        .unwrap();
    songs_repository
        .pinsert_song(
            "No Surprises",
            "1997",
            "Radiohead - No Surprises.mp3",
            "ok_computer.webp",
            third_artist.id,
        )
        .unwrap();
    songs_repository
        .pinsert_song(
            "The Court of the Crimson King",
            "1969",
            "King Crimson - The Court Of The Crimson King.mp3",
            "in_the_court_of_king_crismon.webp",
            fourth_artist.id,
        )
        .unwrap();
}
