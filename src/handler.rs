use std::ops::Add;

use chrono::NaiveDateTime;
use log::debug;
use rand::Rng;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    dto::{SignInBody, SignUpBody},
    entity::users,
};

pub async fn register_user(db: &DatabaseConnection, user: SignUpBody) -> Result<String, DbErr> {
    let salt: [u8; 8] = rand::thread_rng().gen();
    let model: users::ActiveModel = users::ActiveModel {
        id: Set(Uuid::new_v4().to_simple().to_string()),
        username: Set(user.username),
        password: Set(argon2::hash_encoded(
            user.password.as_bytes(),
            &salt,
            &argon2::Config::default(),
        )
        .unwrap()),
        created_at: NotSet,
    };
    debug!("{:?}", model);
    match users::Entity::insert(model).exec(db).await {
        Ok(res) => Ok(res.last_insert_id),
        Err(err) => Err(err),
    }
}

pub async fn get_one_by_username(
    db: &DatabaseConnection,
    username: String,
) -> Result<Option<users::Model>, DbErr> {
    users::Entity::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await
}

pub async fn get_one_by_id(
    db: &DatabaseConnection,
    id: String,
) -> Result<Option<users::Model>, DbErr> {
    users::Entity::find_by_id(id).one(db).await
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    #[serde(with = "claim_serde")]
    pub iat: NaiveDateTime,
    #[serde(with = "claim_serde")]
    pub exp: NaiveDateTime,
}

mod claim_serde {
    use chrono::NaiveDateTime;
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(date.timestamp())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        NaiveDateTime::from_timestamp_opt(i64::deserialize(deserializer)?, 0)
            .ok_or_else(|| de::Error::custom("Invalid unix timestamp value"))
    }
}

pub async fn authenticate_user(
    db: &DatabaseConnection,
    user_data: SignInBody,
    secret: &String,
) -> Option<String> {
    let user = match get_one_by_username(db, user_data.username).await.unwrap() {
        Some(user) => user,
        None => return None,
    };
    match argon2::verify_encoded(&user.password, user_data.password.as_bytes()) {
        Err(_) | Ok(false) => return None,
        _ => (),
    };
    let claims = Claims {
        sub: user.id,
        iat: chrono::Utc::now().naive_utc(),
        exp: chrono::Utc::now()
            .naive_utc()
            .add(chrono::Duration::days(7)),
    };
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();
    Some(token)
}

pub async fn validate_jwt(token: String, secret: &String) -> Option<String> {
    debug!("VALIDATING with {}, {}", token, secret);
    let decode_attempt = jsonwebtoken::decode::<Claims>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
        &jsonwebtoken::Validation::default(),
    );
    debug!("decoding result {:?}", decode_attempt);
    match decode_attempt {
        Ok(token) => Some(token.claims.sub),
        Err(_) => None,
    }
}
