use actix_web::{ App, HttpServer, web };
use actix_web::dev::Server;
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::{ greet, health_check, subscribe };


pub fn run(
    listener: TcpListener,
    db_pool: PgPool
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
        })
        .listen(listener)?
        .run();
    
    Ok(server)
}
