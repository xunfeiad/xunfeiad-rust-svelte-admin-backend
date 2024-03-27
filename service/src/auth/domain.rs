use crate::get_db;
use crate::WebResult;
use entity::domain::{self, Entity as Domain, Model as DomainSchema};
use sea_orm::{ActiveModelTrait, EntityTrait, QuerySelect};

pub struct Service;

impl Service {
    pub async fn create(schema: DomainSchema) -> WebResult<DomainSchema> {
        let user: domain::ActiveModel = schema.into();
        let res = Domain::insert(user).exec(&get_db().await?).await?;
        let user = Domain::find_by_id(res.last_insert_id)
            .one(&get_db().await?)
            .await?
            .unwrap();
        Ok(user)
    }

    pub async fn get_multi(limit: u64, offset: u64) -> WebResult<Vec<DomainSchema>> {
        let users = Domain::find()
            .limit(limit)
            .offset(offset)
            .all(&get_db().await?)
            .await?;
        Ok(users)
    }

    pub async fn get_by_id(id: i32) -> WebResult<DomainSchema> {
        let user = Domain::find_by_id(id).one(&get_db().await?).await?.unwrap();
        Ok(user)
    }

    pub async fn update(id: i32, mut schema: DomainSchema) -> WebResult<DomainSchema> {
        schema.id = Some(id);

        let user_model: domain::ActiveModel = schema.into();
        let user: DomainSchema = user_model.update(&get_db().await?).await?.into();
        Ok(user)
    }

    pub async fn delete(id: i32) -> WebResult<()> {
        Domain::delete_by_id(id).exec(&get_db().await?).await?;
        Ok(())
    }
}
