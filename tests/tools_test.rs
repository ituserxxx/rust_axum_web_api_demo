use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header,Algorithm, EncodingKey,decode, DecodingKey, Validation };

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
   sub: String,
   company: String
}

// cargo test --test tools_test jwt_en
#[tokio::test]
async fn jwt_en() {
    let my_claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned()
    };
    // my_claims is a struct that implements Serialize
    // This will create a JWT using HS256 as algorithm
    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
     eprintln!("token: {:?}", token);

    let token_message = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::HS256));
    eprintln!("token_message: {:?}", token_message);
}
