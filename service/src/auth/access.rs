use crate::get_db;
use crate::WebResult;
use entity::access::{self, Entity as Access, Model as AccessSchema};
use sea_orm::{ActiveModelTrait, EntityTrait, QuerySelect};

pub struct Service;

impl Service {
    pub async fn create(schema: AccessSchema) -> WebResult<AccessSchema> {
        let user: access::ActiveModel = schema.into();
        let res = Access::insert(user).exec(&get_db().await?).await?;
        let user = Access::find_by_id(res.last_insert_id)
            .one(&get_db().await?)
            .await?
            .unwrap();
        Ok(user)
    }

    pub async fn get_multi(limit: u64, offset: u64) -> WebResult<Vec<AccessSchema>> {
        let users = Access::find()
            .limit(limit)
            .offset(offset)
            .all(&get_db().await?)
            .await?;
        Ok(users)
    }

    pub async fn get_by_id(id: i32) -> WebResult<AccessSchema> {
        let user = Access::find_by_id(id).one(&get_db().await?).await?.unwrap();
        Ok(user)
    }

    pub async fn update(id: i32, mut schema: AccessSchema) -> WebResult<AccessSchema> {
        schema.id = id;

        let user_model: access::ActiveModel = schema.into();

        let user: AccessSchema = user_model.update(&get_db().await?).await?.into();
        Ok(user)
    }

    pub async fn delete(id: i32) -> WebResult<()> {
        Access::delete_by_id(id).exec(&get_db().await?).await?;
        Ok(())
    }
}
