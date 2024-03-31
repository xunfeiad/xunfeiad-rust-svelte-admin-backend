mod authority;
pub mod middleware;

use crate::middleware::AuthMiddleWare;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{
    http::header,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use env_logger::Env;
use pkg::AppState;
use service::get_db;
use std::{env, io};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let conn = get_db().await.unwrap();
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(Data::new(AppState { db: conn.clone() }))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors)
            .configure(api::route::scoped_user_config)
    })
    .bind("127.0.0.1:33333")?
    .workers(2)
    .run()
    .await
}
