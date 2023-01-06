use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use sea_orm::prelude::DatabaseConnection;
use sea_orm::Database;
use std::error::Error;

use crate::app_settings::{AppSettings, OAuthSettings};
use crate::auth::JWKSStore;

pub struct AppState {
    pub db: DatabaseConnection,
    pub auth_client: BasicClient,
    pub settings: AppSettings,
    pub jwks_store: JWKSStore,
}

fn create_auth_client(settings: &OAuthSettings) -> Result<BasicClient, Box<dyn Error>> {
    Ok(BasicClient::new(
        ClientId::new(settings.client_id.clone()),
        Some(ClientSecret::new(settings.client_secret.clone())),
        AuthUrl::new(format!("https://{}/authorize", settings.domain))?,
        Some(TokenUrl::new(format!(
            "https://{}/oauth/token",
            settings.domain
        ))?),
    ))
}

impl AppState {
    pub async fn new(settings: &AppSettings) -> Result<Self, Box<dyn Error>> {
        let db: DatabaseConnection = Database::connect(settings.postgres_url.clone())
            .await
            .unwrap();

        // TODO: Change this in future
        let redirect_uri = "https://127.0.0.1:8080/auth/authorize".to_string();
        let auth_client = create_auth_client(&settings.oauth.clone())?.set_redirect_uri(
            RedirectUrl::new(redirect_uri).expect("Failed to create redirect URI"),
        );

        let jwks_store = JWKSStore::new(settings.oauth.domain.clone()).await?;

        Ok(AppState {
            db,
            auth_client,
            settings: settings.clone(),
            jwks_store,
        })
    }
}
