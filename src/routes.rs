extern crate diesel;

use crate::controllers::create_user;
use crate::controllers::index;
use actix_web::{web};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    name: String,
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index::index));
    cfg.route("/", web::post().to(create_user::create));
}