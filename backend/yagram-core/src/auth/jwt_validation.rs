use crate::app_state::AppState;
use crate::errors::ServiceError;
use actix_web::{dev::ServiceRequest, web, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};

pub struct JWKSFetcher {
    jwks: Option<JWKS>,
}

async fn fetch_jwks(auth_domain: &String) -> Result<JWKS, Box<dyn std::error::Error>> {
    Ok(
        reqwest::get(format!("https://{}/.well-known/jwks.json", auth_domain))
            .await?
            .json::<JWKS>()
            .await?,
    )
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let data = req.app_data::<web::Data<AppState>>().unwrap();
    let oauth_domain = &data.settings.oauth.domain;
    let jwks = &data.jwks_store.jwks;

    let token = credentials.token();

    let validations = vec![
        Validation::Issuer(format!("https://{}/", oauth_domain)),
        Validation::SubjectPresent,
    ];

    // If a JWKS contains multiple keys, the correct KID first
    // needs to be fetched from the token headers.
    let kid = match token_kid(token) {
        Ok(asd) => match asd {
            Some(kid) => kid,
            None => {
                return Err((
                    Error::from(ServiceError::AuthenticationError(
                        "No 'kid' claim present in token".to_string(),
                    )),
                    req,
                ));
            }
        },
        Err(error) => {
            return Err((
                Error::from(ServiceError::AuthenticationError(error.to_string())),
                req,
            ));
        }
    };

    let jwk = jwks.find(&kid).expect("Specified key not found in set");

    let res = validate(token, jwk, validations);
    match res {
        Ok(_) => Ok(req),
        Err(error) => {
            println!("{:?}", error);
            Err((
                Error::from(ServiceError::AuthenticationError(error.to_string())),
                req,
            ))
        }
    }
}
