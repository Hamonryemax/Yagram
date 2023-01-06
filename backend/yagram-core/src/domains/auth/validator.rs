use crate::app_data::AppState;
use crate::auth::JwtUserPayload;
use crate::domains::users;
use crate::errors::ServiceError;
use actix_session::SessionExt;
use actix_web::{dev::ServiceRequest, web, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use alcoholic_jwt::{token_kid, validate, Validation};
use serde_json::from_value;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let data = req.app_data::<web::Data<AppState>>().unwrap();
    let session = req.get_session();
    let oauth_domain = &data.settings.oauth.domain;
    let oauth_audience = &data.settings.oauth.audience;
    let jwks = &data.jwks_store.jwks;

    let token = credentials.token();

    let validations = vec![
        Validation::Issuer(format!("https://{}/", oauth_domain)),
        Validation::Audience(oauth_audience.to_string()),
        Validation::NotExpired,
        Validation::SubjectPresent,
    ];

    // If a JWKS contains multiple keys, the correct KID first
    // needs to be fetched from the token headers.
    let token_claims = match token_kid(token) {
        Ok(token_claims) => token_claims,
        Err(error) => {
            return Err((
                Error::from(ServiceError::AuthenticationError(error.to_string())),
                req,
            ));
        }
    };

    let kid = match token_claims {
        Some(kid) => kid,
        None => {
            return Err((
                Error::from(ServiceError::AuthenticationError(
                    "No 'kid' claim present in token".to_string(),
                )),
                req,
            ));
        }
    };

    let jwk = match jwks.find(&kid) {
        Some(jwk) => jwk,
        None => {
            return Err((
                Error::from(ServiceError::AuthenticationError(
                    "Specified key not found in set".to_string(),
                )),
                req,
            ))
        }
    };

    let valid_jwt = match validate(token, jwk, validations) {
        Ok(valid_jwt) => valid_jwt,
        Err(error) => {
            return Err((
                Error::from(ServiceError::AuthenticationError(error.to_string())),
                req,
            ))
        }
    };

    let jwt_user_payload = match from_value::<JwtUserPayload>(valid_jwt.claims) {
        Ok(jwt_payload) => jwt_payload,
        Err(error) => {
            return Err((
                Error::from(ServiceError::AuthenticationError(error.to_string())),
                req,
            ))
        }
    };

    let user = users::queries::find_user_by_sub(&data.db, jwt_user_payload.sub)
        .await
        .unwrap();

    let db_user = match user {
        Some(user) => user,
        None => {
            let auth0_user_info = users::queries::auth0_user_info(oauth_domain.to_string(), token)
                .await
                .unwrap();
            users::mutations::create_user_from_auth0(&data.db, auth0_user_info)
                .await
                .expect("Failed to create new user")
        }
    };

    session
        .insert("user", db_user)
        .expect("Failed to write to session");

    Ok(req)
}
