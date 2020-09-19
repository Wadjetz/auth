extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
embed_migrations!("./migrations");

mod config;
mod errors;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;

use crate::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let config = Config::new().expect("Config Error");
    let address = config.address();

    let database_url = config.database_url.clone();
    actix_web::web::block(move || {
        use diesel::prelude::*;
        let connection = diesel::pg::PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to the database"));
        embedded_migrations::run(&connection)
    })
    .await
    .expect("Migration error");

    let sqlx_pool = sqlx::PgPool::new(&config.database_url)
        .await
        .expect("creating pool error");

    HttpServer::new(move || {
        App::new()
            .data(sqlx_pool.clone())
            .data(config.clone())
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.1.0"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
    })
    .bind(&address)?
    .run()
    .await
}
