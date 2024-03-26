use mongodb::error::Error as MongoError;
use serde_json::Error as JsonError;
use jsonwebtoken::errors::Error as JWTError;
use std::io::Error as IOError;


macro_rules! impl_from {
    ($from: ty, $err: expr) => {
        impl From<$from> for Error {
            fn from(err: $from) -> Self {
                $err(err)
            }
        }
    };
}

pub enum Error {
    NotFound,
    MongoError(MongoError),
    JsonError(JsonError),
    JWTError(JWTError),
    IOError(IOError),
    TokenExpired,
    InvalidFile,
}

impl_from!(MongoError, Error::MongoError);
impl_from!(JsonError, Error::JsonError);
impl_from!(JWTError, Error::JWTError);
impl_from!(IOError, Error::IOError);

pub type Result<T> = core::result::Result<T, Error>;
