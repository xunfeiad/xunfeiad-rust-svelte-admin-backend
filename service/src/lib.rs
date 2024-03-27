use actix_web::Result;
use pkg::WebResult;
use sea_orm::DatabaseConnection;
use serde::Serialize;

pub mod auth;
pub mod db;

// db connection trait
trait DBConnection {
    async fn get_db(&mut self) -> DatabaseConnection;

    async fn ping(&self) -> Result<()>;
}

pub async fn get_db() -> WebResult<DatabaseConnection> {
    let db = db::DBConnection::new()
        .get_db()
        .await
        .expect("Fail to connect to postgresql.");
    Ok(db.connection.unwrap())
}
