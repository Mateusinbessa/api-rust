use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use dotenv::dotenv;
use std::env;
use tokio_postgres::NoTls;

pub async fn create_pool() -> deadpool_postgres::Pool {
    dotenv().ok();

    let mut cfg = Config::new();

    cfg.host = Some(env::var("PG_HOST").expect("PG_HOST não definido"));
    cfg.port = Some(env::var("PG_PORT").expect("PG_PORT não definido").parse().unwrap());
    cfg.user = Some(env::var("PG_USER").expect("PG_USER não definido"));
    cfg.password = Some(env::var("PG_PASSWORD").expect("PG_PASSWORD não definido"));
    cfg.dbname = Some(env::var("PG_DBNAME").expect("PG_DBNAME não definido"));

    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("Falha ao criar pool de conexões")
}
