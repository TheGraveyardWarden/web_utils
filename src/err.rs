use mongodb::error::Error as MongoError;
use serde_json::Error as JsonError;
use jsonwebtoken::errors::Error as JWTError;

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
    TokenExpired
}

impl_from!(MongoError, Error::MongoError);
impl_from!(JsonError, Error::JsonError);
impl_from!(JWTError, Error::JWTError);

pub type Result<T> = core::result::Result<T, Error>;
