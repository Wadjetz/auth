#![allow(clippy::module_inception)]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
embed_migrations!("./migrations");

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod config;
mod domain;
mod errors;
mod jwt;
mod password;
mod repositories;
mod routes;
mod templates;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use log::info;

use crate::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let config = Config::new().expect("Config Error");
    let address = config.address();

    info!("Server at {}", &address);

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
            .data(templates::create_templates().expect("Templates errors"))
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.1.0"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(routes::create_app::create_application_route)
            .service(routes::login::login_form_route)
            .service(routes::login::login_route)
            .service(routes::signup::signup_form_route)
            .service(routes::signup::signup_route)
            .service(routes::authorize::authorize_form_route)
            .service(routes::token::token_route)
    })
    .bind(&address)?
    .run()
    .await
}
