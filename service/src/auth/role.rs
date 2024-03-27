use crate::WebResult;
use entity::role::{self, ActiveModel, Entity as Role, Model};
use sea_orm::DbErr::Custom;
use sea_orm::Set;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder,
};

pub struct Service;

impl Service {
    pub async fn create(db: &DatabaseConnection, model: Model) -> WebResult<()> {
        ActiveModel {
            role_name: Set(model.role_name),
            ..Default::default()
        }
        .save(db)
        .await?;
        Ok(())
    }

    pub async fn get_multi(
        db: &DatabaseConnection,
        page: u64,
        page_size: u64,
    ) -> WebResult<Vec<Model>> {
        let paginator = Role::find()
            .filter(role::Column::IsDelete.eq(false))
            .order_by_asc(role::Column::Id)
            .paginate(db, page_size);
        let models = paginator.fetch_page(page - 1).await?;
        Ok(models)
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> WebResult<Model> {
        let user = Role::find_by_id(id)
            .filter(role::Column::IsDelete.eq(false))
            .one(db)
            .await?
            .ok_or(Custom("Role not found.".to_owned()))?;
        Ok(user)
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        updated_model: Model,
    ) -> WebResult<Model> {
        let role: ActiveModel = Self::get_by_id(db, id).await.map(Into::into)?;
        let updated_role = ActiveModel {
            id: role.id,
            role_name: Set(updated_model.role_name),
            ..Default::default()
        }
        .update(db)
        .await?;
        Ok(updated_role)
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> WebResult<()> {
        let mut role: ActiveModel = Self::get_by_id(db, id).await.map(Into::into)?;
        role.is_delete = ActiveValue::Set(Some(true));
        role.update(db).await?;
        Ok(())
    }
}
