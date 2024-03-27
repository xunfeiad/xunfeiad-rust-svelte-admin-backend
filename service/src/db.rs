use anyhow::{anyhow, Result};
use dotenvy::dotenv;
use log::LevelFilter;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct DBConnection {
    pub url: String,
    pub schema: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
    pub sqlx_logging: bool,
    pub sqlx_logging_level: LevelFilter,
    pub connection: Option<DatabaseConnection>,
}

impl DBConnection {
    #[allow(clippy::too_many_arguments)]
    pub fn new() -> Self {
        dotenv().unwrap();
        let url = env::var("DATABASE_URL").unwrap();
        let schema = env::var("DATABASE_SCHEMA").unwrap();
        Self {
            url,
            schema,
            max_connections: 1000,
            min_connections: 5,
            connect_timeout: 8,
            acquire_timeout: 8,
            idle_timeout: 8,
            max_lifetime: 8,
            sqlx_logging: true,
            sqlx_logging_level: LevelFilter::Info,
            connection: None,
        }
    }
    // todo!
    pub async fn get_db(mut self) -> Result<Self> {
        let mut opt = ConnectOptions::new(self.url.as_str());
        opt.max_connections(self.max_connections)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(self.connect_timeout))
            .acquire_timeout(Duration::from_secs(self.acquire_timeout))
            .idle_timeout(Duration::from_secs(self.idle_timeout))
            .max_lifetime(Duration::from_secs(self.max_lifetime))
            .sqlx_logging(self.sqlx_logging)
            .sqlx_logging_level(self.sqlx_logging_level)
            .set_schema_search_path(self.schema.as_str());

        self.connection = Some(Database::connect(opt).await?);
        Ok(self)
    }

    pub async fn ping(&self) -> Result<()> {
        match &self.connection {
            Some(connection) => {
                if connection.ping().await.is_ok() {
                    Ok(())
                } else {
                    Err(anyhow!("Connection has not connected..."))
                }
            }
            _ => Err(anyhow!("Connection has not connected...")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn ping() {
        dotenv().unwrap();
        let db = DBConnection::new();
        let db = db.get_db().await.unwrap();
        println!("{:?}", &db);
        db.ping().await.unwrap()
    }
}
