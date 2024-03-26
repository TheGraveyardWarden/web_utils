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

#[derive(Debug)]
pub enum Error {
    NotFound,
    MongoError(MongoError),
    JsonError(JsonError),
    JWTError(JWTError),
    IOError(IOError),
    TokenExpired,
    InvalidFile,
    DirNotSpecified
}

impl Error {
    pub fn msg(self) -> String {
        use Error::*;

        match self {
            NotFound => format!("Document not found"),
            MongoError(err) => format!("MongoError: {:?}", err),
            JsonError(err) => format!("JsonError: {:?}", err),
            JWTError(err) => format!("JWTError: {:?}", err),
            IOError(err) => format!("IOError: {:?}", err),
            TokenExpired => format!("Token expired"),
            InvalidFile => format!("Invalid file"),
            DirNotSpecified => format!("directory not specified")
        }
    }
}

impl_from!(MongoError, Error::MongoError);
impl_from!(JsonError, Error::JsonError);
impl_from!(JWTError, Error::JWTError);
impl_from!(IOError, Error::IOError);

pub type Result<T> = core::result::Result<T, Error>;
