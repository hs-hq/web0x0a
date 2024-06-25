use super::handlers::common::UserStruct;
use std::fs;
use std::io::prelude::*;
use std::str;
use std::{env, fs::File, path::Path};

#[derive(Debug)]
pub enum SessionHandlingErrorKind {
    DeserializationFailed,
    SerializationFailed,
    NoWrite,
    BadFilePath,
    SessionNotFound,
    Base64Failed,
}

use tracing::log::debug;
use SessionHandlingErrorKind::*;
pub fn save_user(user_struct: &UserStruct) -> Result<String, SessionHandlingErrorKind> {
    let user_des = serde_yaml::to_string(&user_struct);
    match user_des {
        Ok(user_des) => {
            let base64_value = base64::encode(&user_des.trim());

            let user_data = generate_uuid(base64_value.clone());
            let path_value = format!(
                "{}/sessions/{}",
                env::current_dir().unwrap().to_str().unwrap(),
                user_data.0
            );

            let path = Path::new(path_value.as_str());

            let mut file = match File::create(&path) {
                Err(_) => {
                    return Err(SessionNotFound);
                }
                Ok(file) => file,
            };

            match file.write_all(base64_value.as_bytes()) {
                Err(why) => {
                    debug!("{:?}", why);
                    Err(NoWrite)
                }
                Ok(_) => Ok(user_data.1),
            }
        }
        Err(_) => Err(SerializationFailed),
    }
}

pub fn generate_uuid(user_des: String) -> (String, String) {
    let xor_res = xor(
        user_des.into_bytes(),
        &env::var("SECRET_KEY")
            .expect("SECRET_KEY env variable not found")
            .into_bytes(),
    );
    let x = base64::encode(&xor_res);

    let digest = md5::compute(x.as_bytes());
    (format!("{:x}", digest), x)
}

pub fn xor(s: Vec<u8>, key: &[u8]) -> Vec<u8> {
    let mut b = key.iter().cycle();
    s.into_iter().map(|x| x ^ b.next().unwrap()).collect()
}

pub fn retrieve_user_file(token: &str) -> Result<UserStruct, SessionHandlingErrorKind> {
    println!("{:?}", token);
    let digest = md5::compute(token.as_bytes());

    let path_value = format!(
        "{}/sessions/{}",
        env::current_dir().unwrap().to_str().unwrap(),
        format!("{:x}", digest)
    );

    let contents = fs::read_to_string(path_value);
    match contents {
        Ok(contents) => {
            if let Ok(ser_decoded_user) = base64::decode(contents.as_bytes()) {
                let deserialized_user =
                    serde_yaml::from_str(str::from_utf8(&ser_decoded_user).unwrap());

                match deserialized_user {
                    Ok(deserialized_user) => Ok(deserialized_user),
                    Err(_) => Err(DeserializationFailed),
                }
            } else {
                Err(Base64Failed)
            }
        }
        Err(_) => Err(BadFilePath),
    }
}

pub fn sanitize_filepath(input: &str) -> String {
    let options = sanitize_filename::Options {
        truncate: true,  // true by default, truncates to 255 bytes
        windows: true, // default value depends on the OS, removes reserved names like `con` from start of strings on Windows
        replacement: "", // str to replace sanitized chars/strings
    };

    sanitize_filename::sanitize_with_options(input, options)
}
