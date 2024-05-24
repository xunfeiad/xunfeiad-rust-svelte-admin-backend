// use anyhow::Result;
// use dotenvy::dotenv;
// use migration::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::prelude::*;
// use std::env;

#[async_std::main]
async fn main() {
    // dotenv().unwrap();
    // let url = env::var("DATABASE_URL").unwrap();
    // get_db(url,"")
    cli::run_cli(migration::Migrator).await;
}

// async fn get_db(url: &str, schema_name: String) -> Result<DatabaseConnection> {
//     let connect_options = ConnectOptions::new(url)
//         .set_schema_search_path(schema_name)
//         .to_owned();
//     let db = Database::connect(connect_options).await?;
//     Ok(db)
// }
//
