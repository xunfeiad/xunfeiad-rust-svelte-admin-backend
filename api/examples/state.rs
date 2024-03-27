use actix_web::{get, web, App, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
async fn index(
    data: web::Data<AppState>,
    counter: web::Data<AppStateWithCounter>,
) -> impl Responder {
    let mut counter = counter.counter.lock().unwrap();
    *counter += 1;
    format!(
        "Hello, World, {}, accessed times: {:?}",
        data.app_name, counter
    )
}

#[derive(Deserialize, Serialize)]
struct MyInfo {
    id: usize,
    username: String,
}

#[get("/users/{user_id}/{friend}")]
async fn userinfo(path: web::Path<(String, String)>, json: web::Json<MyInfo>) -> impl Responder {
    let path = path.into_inner();
    format!("{} {} {} {}", path.0, path.1, json.id, json.username)
}
struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder.set_private_key_file("key.pem",SslFiletype::PEM)
    //     .unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();

    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new().service(
            web::scope("/app")
                .app_data(web::Data::new(AppState {
                    app_name: "Actix_web".to_string(),
                }))
                .app_data(counter.clone())
                .route("/index.html", web::get().to(index))
                .service(userinfo),
        )
    })
    // .bind_openssl("127.0.0.1:8082",builder)?
    .bind("127.0.0.1:8082")?
    .run()
    .await
}
