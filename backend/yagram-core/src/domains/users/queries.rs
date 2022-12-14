use entity::prelude::User;
use entity::user;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

pub async fn find_user_by_id(db: &DbConn, id: i32) -> Result<Option<user::Model>, DbErr> {
    User::find_by_id(id).one(db).await
}

pub async fn find_user_by_sub(db: &DbConn, sub: String) -> Result<Option<user::Model>, DbErr> {
    User::find().filter(user::Column::Sub.eq(sub)).one(db).await
}

#[derive(Serialize, Deserialize)]
pub struct Auth0UserInfo {
    pub sub: String,
    pub given_name: String,
    pub family_name: String,
    pub nickname: String,
    pub email: String,
}
pub async fn auth0_user_info(
    auth_domain: String,
    token: &str,
) -> Result<Auth0UserInfo, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let result = client
        .get(format!("https://{}/userinfo", auth_domain))
        .bearer_auth(token)
        .send()
        .await?
        .json::<Auth0UserInfo>()
        .await?;

    Ok(result)
}
