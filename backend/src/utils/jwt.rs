use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {

    pub sub: String,
    pub exp: usize,
}

pub fn generate_token(user: &str) -> String {

    let claims = Claims {

        sub: user.to_string(),
        exp: 2000000000,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref())
    )
    .unwrap()
}
