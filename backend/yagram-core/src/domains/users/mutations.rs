use crate::domains::users::queries::Auth0UserInfo;
use chrono::prelude::*;
use entity::prelude::User;
use entity::user;
use sea_orm::entity::prelude::*;
use sea_orm::InsertResult;

pub async fn create_user_from_auth0(
    db: &DbConn,
    data: Auth0UserInfo,
) -> Result<InsertResult<user::ActiveModel>, DbErr> {
    let mut new_user: user::ActiveModel = Default::default();
    new_user.set(user::Column::FirstName, data.given_name.into());
    new_user.set(user::Column::LastName, data.family_name.into());
    new_user.set(user::Column::Login, data.nickname.into());
    new_user.set(user::Column::Sub, data.sub.into());
    new_user.set(user::Column::Email, data.email.into());
    new_user.set(user::Column::CreatedAt, Utc::now().naive_utc().into());

    user::Entity::insert(new_user).exec(db).await
}
