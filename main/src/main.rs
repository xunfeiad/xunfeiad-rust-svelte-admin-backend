mod authority;
pub mod middleware;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use pkg::AppState;
use service::get_db;
use std::{env, io};
mod websocket;
use websocket::index;

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
    // let subscriber = FmtSubscriber::builder()
    //     .with_max_level(Level::TRACE)
    //     .finish();
    //
    // tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    env_logger::init();
    let conn = get_db().await.unwrap();
    let tera = tera::Tera::new("api/templates/*/**").unwrap();
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(Data::new(AppState {
                db: conn.clone(),
                tera: tera.clone(),
            }))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors)
            .configure(api::route::scoped_user_config)
            .route("/ws/", web::get().to(index))
    })
    .bind("127.0.0.1:33333")?
    .workers(2)
    .run()
    .await
}
