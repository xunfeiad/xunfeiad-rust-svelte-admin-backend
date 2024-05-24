use anyhow::Result;
use entity::task::{ActiveModel, Column, Entity as Task, Extra, Model};
use lettre::message::header::ContentType;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use pkg::WebResult;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use std::os::macos::raw::stat;
use std::sync::Arc;

pub enum TaskStatus {
    Pending,
    Success,
    Failed,
}

pub struct Service;

impl Service {
    pub async fn create_task(db: &DatabaseConnection, extra: &Extra) -> WebResult<i32> {
        let model = ActiveModel {
            id: NotSet,
            task_cate: Set("Email".to_owned()),
            status: Set("Pending".to_owned()),
            extra: Set(Some(extra.clone())),
            ..Default::default()
        }
        .save(db)
        .await?;

        Ok(model.id.unwrap())
    }

    pub async fn record_task(
        db: &DatabaseConnection,
        id: i32,
        status: String,
        traceback: Option<String>,
    ) -> WebResult<()> {
        let mut active_model: ActiveModel = Self::get_by_id(db, id).await.map(Into::into)?;
        active_model.status = Set(status);
        active_model.traceback = Set(traceback);
        active_model.save(db).await?;
        Ok(())
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> WebResult<Model> {
        let model = Task::find_by_id(id)
            .filter(Column::IsDelete.eq(false))
            .one(db)
            .await?
            .ok_or("task not found.".to_owned())?;
        Ok(model)
    }

    pub fn send_email(
        from: String,
        to: String,
        subject: String,
        body: Arc<String>,
        email_crypt: EmailCrypt,
    ) -> Result<()> {
        let email = Message::builder()
            .from(from.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body.to_string())?;

        let creds = Credentials::new(
            email_crypt.username.to_string(),
            email_crypt.password.to_string(),
        );

        let mailer = SmtpTransport::relay(email_crypt.server)?
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully"),
            Err(e) => eprintln!("Could not send the email: {:?}", e),
        }

        Ok(())
    }
}
pub struct EmailCrypt {
    pub server: &'static str,
    pub username: &'static str,
    pub password: &'static str,
    pub email: &'static str,
}
