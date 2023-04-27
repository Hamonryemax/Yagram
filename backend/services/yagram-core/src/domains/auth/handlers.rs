use crate::app_data::AppState;
use actix_session::Session;
use actix_web::{get, http::header, web, HttpResponse, Responder};
use oauth2::{
    reqwest::async_http_client, AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    Scope,
};
use serde::Deserialize;

#[get("/login")]
pub async fn login(data: web::Data<AppState>, session: Session) -> impl Responder {
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256_len(96);

    session
        .insert("state", pkce_code_verifier)
        .expect("Failed to write in session");

    // Generate the authorization URL to which we'll redirect the user.
    let (auth_url, csrf_token) = &data
        .auth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_extra_param("audience", &data.settings.oauth.audience)
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    session
        .insert("csrf", csrf_token)
        .expect("Failed to write in session");

    HttpResponse::Found()
        .append_header((header::LOCATION, auth_url.to_string()))
        .finish()
}

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[get("/authorize")]
pub async fn authorize(
    data: web::Data<AppState>,
    params: web::Query<AuthRequest>,
    session: Session,
) -> impl Responder {
    let code = AuthorizationCode::new(params.code.clone());
    let state = CsrfToken::new(params.state.clone());

    let pkce_code_verifier = session.get::<PkceCodeVerifier>("state").unwrap().unwrap();
    let csrf = session.get::<CsrfToken>("csrf").unwrap().unwrap();

    assert_eq!(
        state.secret(),
        csrf.secret(),
        "Invalid csrf was provided\ngot {}\n expected{}",
        state.secret(),
        csrf.secret()
    );

    // Exchange the code with a token
    let token = &data
        .auth_client
        .exchange_code(code)
        .set_pkce_verifier(pkce_code_verifier)
        .request_async(async_http_client)
        .await;

    match token {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::InternalServerError().json(format!("{:?}", err)),
    }
}
