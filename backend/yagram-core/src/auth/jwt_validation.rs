use crate::errors::ServiceError;
use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::Deserialize;
use std::env;

async fn fetch_jwks() -> Result<JWKS, Box<dyn std::error::Error>> {
    Ok(reqwest::get(format!(
        "https://{}/.well-known/jwks.json",
        env::var("OAUTH_DOMAIN").expect("FAILED")
    ))
    .await?
    .json::<JWKS>()
    .await?)
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwks_result = fetch_jwks().await;

    match jwks_result {
        Ok(jwks) => {
            let token = credentials.token();

            let validations = vec![
                Validation::Issuer(format!(
                    "https://{}/",
                    env::var("OAUTH_DOMAIN")
                        .expect("Failed to read OAUTH_DOMAIN in environment variable")
                )),
                Validation::SubjectPresent,
            ];

            // If a JWKS contains multiple keys, the correct KID first
            // needs to be fetched from the token headers.
            let kid = token_kid(token)
                .expect("Failed to decode token headers")
                .expect("No 'kid' claim present in token");

            let jwk = jwks.find(&kid).expect("Specified key not found in set");

            let res = validate(token, jwk, validations);
            match res {
                Ok(_) => Ok(req),
                Err(error) => {
                    println!("{:?}", error);
                    Err((Error::from(ServiceError::AuthenticationError), req))
                }
            }
        }
        _ => Err((Error::from(ServiceError::JWKSFetchError), req)),
    }
}
