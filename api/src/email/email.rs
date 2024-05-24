use actix_web::{
    get,
    web::{Data, Path},
    Error, HttpResponse, Result,
};
use anyhow::anyhow;
use entity::task::{EmailContent, Extra};
use log::info;
use pkg::{AppState, WebError, WebResult};
use sea_orm::DatabaseConnection;
use service::task::email::Service;
use std::sync;
use sync::Arc;

#[get("/{id}")]
pub async fn send_email(id: Path<i32>, data: Data<AppState>) -> Result<HttpResponse> {
    // let model = Service::get_by_id(&data.db, id.into_inner()).await?;
    let mut t = &mut data.tera.clone();
    let ctx = tera::Context::new();
    t.add_template_file("api/templates/email.html.tera", Some("email.html.tera"))
        .unwrap();
    let output = t.render("email.html.tera", &ctx);
    match output {
        Err(e) => Err(Error::from(WebError::InternalError { msg: e.to_string() })),
        _ => {
            tokio::spawn(executed_background_task(
                Arc::new(data.db.clone()),
                Arc::new(output.unwrap()),
                Arc::new(Extra {
                    user_id: Some(id.into_inner()),
                    email_detail: Some(EmailContent {
                        from: None,
                        to: "".to_string(),
                        subject: "".to_string(),
                        body: "".to_string(),
                    }),
                }),
            ));
            Ok(HttpResponse::Ok().content_type("text/html").body(""))
        }
    }
}

pub async fn executed_background_task(
    db: Arc<DatabaseConnection>,
    template: Arc<String>,
    extra: Arc<Extra>,
) -> WebResult<()> {
    let user_id = &extra.user_id;
    let email_detail = &extra.email_detail;
    let email: WebResult<String> = if let Some(user_id) = user_id {
        let user = service::auth::user::Service::get_by_id(db.as_ref(), user_id.clone()).await?;
        Ok(user.email)
    } else {
        let email_detail = email_detail
            .clone()
            .ok_or(anyhow!("Please provide the email detail to send."))?;
        let email = email_detail
            .from
            .ok_or(anyhow!("Please provide the email detail to send."))?;
        Ok(email)
    };
    let task_id = Service::create_task(db.as_ref(), extra.as_ref()).await?;
    info!("find task>>>>:{task_id}");
    let sender = service::task::email::EmailCrypt {
        server: "smtp.qq.com",
        username: "549351256",
        password: "hhxpydyfkshzbcij",
        email: "549351256@qq.com",
    };
    match Service::send_email(
        sender.email.to_string(),
        email?,
        "无锡光大证劵所".to_string(),
        template,
        sender,
    ) {
        Ok(_) => {
            Service::record_task(db.as_ref(), task_id, "Success".to_string(), None).await?;
        }
        Err(e) => {
            Service::record_task(
                db.as_ref(),
                task_id,
                "Failed".to_string(),
                Some(e.to_string()),
            )
            .await?;
        }
    }

    Ok(())
}
