#![allow(clippy::module_inception)]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
embed_migrations!("../migrations");

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod admin;
mod config;
mod errors;
mod routes;
mod templates;
mod utils;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use log::info;
use std::sync::Arc;

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

    let admin_app = admin::admin_init::init_admin_application(sqlx_pool.clone(), &config)
        .await
        .expect("creating admin application error");
    let admin_app = Arc::new(admin_app);

    HttpServer::new(move || {
        App::new()
            .data(sqlx_pool.clone())
            .data(config.clone())
            .data(admin_app.clone())
            .data(templates::create_templates().expect("Templates errors"))
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.1.0"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(routes::application_routes::create_application_route)
            .service(routes::application_routes::get_applications_route)
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
