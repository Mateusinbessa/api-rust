use crate::controllers::user::{get_all_users, get_all_users_paginated, get_one};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/usuarios").route(web::get().to(get_all_users)))
        .service(web::resource("/usuarios/paginated").route(web::get().to(get_all_users_paginated)))
        .service(web::resource("/usuarios/{id}").route(web::get().to(get_one)));
}
