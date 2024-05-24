use base64::prelude::*;
use entity::user_group::ActiveModel;
use sea_orm::ActiveModelTrait;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() {
    use entity::task::ActiveModel;
    use sea_orm::ActiveValue::Set;
    use service::get_db;

    let db = &get_db().await.unwrap();
    let a = ActiveModel {
        task_cate: Set("Scheduler".to_string()),
        status: Set("Failed".to_string()),
        ..Default::default()
    }
    .save(db)
    .await
    .unwrap();
    println!("{:?}", a.id.unwrap())
}
