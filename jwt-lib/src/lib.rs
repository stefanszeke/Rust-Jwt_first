use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub email: String
}

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: usize,
    sub: String
}

pub fn get_jwt(user: User) -> Result<String, String> {
    let header: Header = Header::default();
    let claims: Claims = Claims {
        exp: (Utc::now() + Duration::minutes(1)).timestamp() as usize,
        sub: user.email
    };
    let secret: EncodingKey = EncodingKey::from_secret("bad_secret".as_bytes());

    let token = jsonwebtoken::encode(&header, &claims, &secret) 
        .map_err(|e| format!("Error encoding token: {:?}", e))?;

    Ok(token)
 
}

pub fn decode_jwt(token: &str) -> Result<User, String> {
    let secret = "bad_secret".as_bytes();

    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &jsonwebtoken::Validation::default()
    ).map_err(|e| format!("Error decoding token: {:?}", e))?;

    Ok(User {
        email: token_data.claims.sub
    })
}