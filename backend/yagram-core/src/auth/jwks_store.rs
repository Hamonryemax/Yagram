use alcoholic_jwt::JWKS;

pub struct JWKSStore {
    auth_domain: String,
    pub jwks: JWKS,
}

impl JWKSStore {
    pub async fn new(auth_domain: String) -> Result<Self, Box<dyn std::error::Error>> {
        let jwks = reqwest::get(format!("https://{}/.well-known/jwks.json", auth_domain))
            .await?
            .json::<JWKS>()
            .await?;
        Ok(JWKSStore { auth_domain, jwks })
    }
}
