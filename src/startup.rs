use crate::routes::*;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool); // Wrap database in smart pointer
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/ingredients", web::get().to(get_all_ingredients))
            .service(
                web::scope("/user")
                    .route("/add_user", web::post().to(add_user))
                    .route("/sign_in", web::post().to(sign_in))
                    .route("/update_permissions", web::post().to(update_permissions))
                    .route("/update_password", web::post().to(update_password))
                    .route("/", web::delete().to(delete_user)),
            )
            .service(
                web::scope("/recipe")
                    .route("add_recipe", web::post().to(add_recipe))
                    .route("/", web::delete().to(delete_recipe))
                    .route("/{name}", web::get().to(get_recipe))
                    .route("/", web::post().to(get_recipes)),
            )
            .service(
                web::scope("/kitchen")
                    .route("/{user_id}", web::get().to(get_kitchen))
                    .route("/", web::post().to(update_kitchen)),
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
