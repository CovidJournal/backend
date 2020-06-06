use super::common::{Context, ProfessionalUser, PublicUser};
use super::error::Error;
use base64::decode;
use std::str::FromStr;
use warp::{reject, Filter, Rejection};

const SCHEME: &str = "Basic";

struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn public_user_filter(
    context: Context,
) -> impl Filter<Extract = (PublicUser,), Error = Rejection> + Copy {
    warp::header::<String>("authorization").and_then(|header: String| async move {
        match decrypt_basic_header(header) {
            Some(credentials) => {
                return Ok(PublicUser {
                    id: uuid::Uuid::parse_str("3731796d-06ab-49c7-b603-b12c93852552").unwrap(),
                });
            }
            None => Err(reject::custom(Error::Unauthorized)),
        }
    })
}

pub fn professional_user_filter(
    context: Context,
) -> impl Filter<Extract = (ProfessionalUser,), Error = Rejection> + Copy {
    warp::header::<String>("authorization").and_then(|header: String| async move {
        match decrypt_basic_header(header) {
            Some(credentials) => {
                return Ok(ProfessionalUser {
                    id: uuid::Uuid::parse_str("3731796d-06ab-49c7-b603-b12c93852552").unwrap(),
                });
            }
            None => Err(reject::custom(Error::Unauthorized)),
        }
    })
}

fn decrypt_basic_header(header: String) -> Option<Credentials> {
    if !header.starts_with(SCHEME) || header.len() <= SCHEME.len() + 1 {
        return None;
    }

    match header[SCHEME.len() + 1..].parse::<Credentials>() {
        Ok(h) => Some(h),
        Err(_) => None,
    }
}

struct CredentialsError;

impl FromStr for Credentials {
    type Err = CredentialsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match decode(s) {
            Ok(decoded) => match String::from_utf8(decoded) {
                Ok(text) => {
                    let parts = &mut text.split(':');

                    let username = match parts.next() {
                        Some(part) => part.to_owned(),
                        None => return Err(CredentialsError),
                    };

                    let password = match parts.next() {
                        Some(part) => part.to_owned(),
                        None => return Err(CredentialsError),
                    };

                    Ok(Credentials { username, password })
                }
                Err(e) => {
                    log::debug!("Basic::from_utf8 error={:?}", e);
                    Err(CredentialsError)
                }
            },
            Err(e) => {
                log::debug!("Basic::from_base64 error={:?}", e);
                Err(CredentialsError)
            }
        }
    }
}