mod controllers;
mod database;
mod models;
mod routes;

use actix_web::{App, HttpServer, web};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = database::db::create_pool().await;
    println!("Servidor rodando em http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
