use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use sea_orm::prelude::DatabaseConnection;
use sea_orm::Database;
use std::env;
use std::error::Error;

pub struct AppState {
    pub db: DatabaseConnection,
    pub auth_client: BasicClient,
}

impl AppState {
    pub async fn from_env() -> Result<Self, Box<dyn Error>> {
        let db: DatabaseConnection = Database::connect(
            env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable"),
        )
        .await
        .unwrap();

        let auth_domain = env::var("OAUTH_DOMAIN")?;
        let client_id = env::var("OAUTH_CLIENT_ID")?;
        let client_secret = env::var("OAUTH_CLIENT_SECRET")?;
        // TODO: Change this in future
        let redirect_uri = "https://127.0.0.1:8080/auth/authorize".to_string();

        let auth_client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(format!("https://{}/authorize", auth_domain))?,
            Some(TokenUrl::new(format!(
                "https://{}/oauth/token",
                auth_domain
            ))?),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri).expect(""));

        Ok(AppState { db, auth_client })
    }
}
