use axum::debug_handler;
use loco_rs::prelude::*;

use crate::{models::_entities::users, views::me::MeResponse};

#[debug_handler]
async fn me(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(MeResponse::new(&user))
}

pub fn routes() -> Routes {
    Routes::new().prefix("me").add("/", get(me))
}
