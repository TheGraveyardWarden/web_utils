use jsonwebtoken::{
    encode, Algorithm, Header, EncodingKey,
    decode, DecodingKey, Validation
};
use chrono::{Duration, Utc};
use mongodb::bson::oid::ObjectId;
use crate::err::{Result, Error};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub id: ObjectId,
    exp: usize
}

impl Token {
    pub fn encode(id: &ObjectId, exp: Duration, secret: &str) -> Result<String> {
        let token = Token {
            id: *id,
            exp: Utc::now().timestamp() as usize + exp.num_seconds() as usize
        };

        Ok(encode(&Header::default(), &token, &EncodingKey::from_secret(secret.as_ref()))?)
    }

    pub fn decode(raw_token: &str, secret: &str) -> Result<Self> {
        let token: Token = decode::<Token>(
            &raw_token, &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256)
        )?.claims;

        token.check_expired()?;

        Ok(token)
    }

    fn check_expired(&self) -> Result<()> {
        if Utc::now().timestamp() as usize > self.exp {
            return Err(Error::TokenExpired);
        }

        Ok(())
    }
}
