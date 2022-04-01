use crate::routes::{
    add_user, delete_user, health_check, sign_in, update_password, update_permissions,
};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool); // Wrap database in smart pointer
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/add_user", web::post().to(add_user))
            .route("/sign_in", web::post().to(sign_in))
            .route("/update_permissions", web::post().to(update_permissions))
            .route("/update_password", web::post().to(update_password))
            .route("/delete_user", web::post().to(delete_user))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
