use crate::models::user::User;
use actix_web::{HttpResponse, Responder, web};
use deadpool_postgres::Pool;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationParams {
    page: i64,
    items_per_page: i64,
}

pub async fn get_all_users(pool: web::Data<Pool>) -> impl Responder {
    match User::get_all(&pool).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_all_users_paginated(
    pool: web::Data<Pool>,
    query: web::Query<PaginationParams>, // Usamos a estrutura PaginationParams
) -> impl Responder {
    let params = query.into_inner();

    // Validar que os parâmetros são válidos
    if params.page < 1 || params.items_per_page < 1 {
        return HttpResponse::BadRequest()
            .body("Os parâmetros 'page' e 'items_per_page' devem ser maiores que zero.");
    }

    // Chamar o método get_all_paginated
    match User::get_all_paginated(&pool, params.page, params.items_per_page).await {
        Ok(pagination) => HttpResponse::Ok().json(pagination),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_one(pool: web::Data<Pool>, user_id: web::Path<i32>) -> impl Responder {
    match User::get_one(&pool, user_id.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("Usuário não encontrado"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
