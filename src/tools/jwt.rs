use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header,Algorithm, EncodingKey,decode, DecodingKey, Validation };
use std::time::{Duration, SystemTime};
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
   uid: i64,
   exp:usize,
}


const SECRET: &str = "jwt_secret";

pub async fn en_token(uid: i64) -> String {
    let my_claims = Claims {
        uid: uid,
        exp: SystemTime::now()
            .checked_add(Duration::from_secs(10))
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize,
    };
    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(SECRET.as_ref())).unwrap();
    println!("token: {:?}", token);

    return token
}

pub async fn dn_token(token: String) -> Result<i64, String> {
    match decode::<Claims>(&token, &DecodingKey::from_secret(SECRET.as_ref()), &Validation::new(Algorithm::HS256)) {
        Ok(token_data) => {
            let uid = token_data.claims.uid;
              println!("decoding token: {:?}", uid);
            Ok(uid)
        }
        Err(err) => {
            println!("Error decoding token: {:?}", err);
            Err("Error decoding token".to_string())
        }
    }
}
