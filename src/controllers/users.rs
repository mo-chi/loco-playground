#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    models::_entities::users::{ActiveModel, Entity, Model},
    views::user::UserResponse,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: String,
}

impl Params {
    fn update(&self, user: &mut ActiveModel) {
        user.name = Set(self.name.clone());
    }
}

async fn load_user(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(_auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let users = Entity::find().all(&ctx.db).await?;

    format::json(UserResponse::news(users))
}

#[debug_handler]
pub async fn get_user(
    _auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = load_user(&ctx, id).await?;

    format::json(UserResponse::new(&user))
}

#[debug_handler]
pub async fn update_user(
    _auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let user = load_user(&ctx, id).await?;
    let mut user = user.into_active_model();

    params.update(&mut user);
    user.update(&ctx.db).await?;

    format::empty()
}

#[debug_handler]
pub async fn delete_user(
    _auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = load_user(&ctx, id).await?;

    user.delete(&ctx.db).await?;

    format::empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("users")
        .add("/", get(list))
        .add("/:id", get(get_user))
        .add("/:id", post(update_user))
        .add("/:id", delete(delete_user))
}
