use anyhow::anyhow;
use entity::prelude::Access;
use entity::user::Entity;
use entity::{role, role_access, user, user_role};
use pkg::{WebError, WebResult};
use sea_orm::prelude::DateTime;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr::Custom, EntityTrait,
    FromQueryResult, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Select,
};
use sea_query::{Expr, JoinType};
use serde::{Deserialize, Serialize};

use super::PostUserRole;

pub struct Service;

#[derive(Serialize, Deserialize, FromQueryResult, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserRoles {
    role_id: i32,
    username: String,
    nick_name: Option<String>,
    role_name: String,
    create_time: DateTime,
    update_time: DateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRole {
    user_id: i32,
    role_id: i32,
}

impl Service {
    pub async fn get_user_roles(
        db: &DatabaseConnection,
        page: u64,
        page_size: u64,
        name: Option<String>,
    ) -> WebResult<(Vec<UserRoles>, u64)> {
        let paginator = Self::get_select_entity(name.clone())
            .await
            .into_model()
            .paginate(db, page_size);

        let count = Self::get_select_entity(name).await.count(db).await?;

        let user_roles: Vec<UserRoles> = paginator.fetch_page(page - 1).await?;
        Ok((user_roles, count))
    }

    pub async fn get_select_entity(name: Option<String>) -> Select<Entity> {
        let entity = user::Entity::find().filter(user::Column::IsDelete.eq(false));
        let entity = match name {
            Some(name) => entity.filter(role::Column::RoleName.contains(name)),
            None => entity,
        };
        entity
            .column_as(Expr::col((role::Entity, role::Column::Id)), "role_id")
            .columns([
                role::Column::RoleName,
                role::Column::UpdateTime,
                role::Column::CreateTime,
            ])
            .join(JoinType::Join, user_role::Relation::User.def().rev())
            .join(JoinType::Join, user_role::Relation::Role.def())
            .order_by_asc(role::Column::Id)
    }

    pub async fn get_permission(db: &DatabaseConnection) -> WebResult<()> {
        let count = user::Entity::find()
            .filter(user::Column::IsDelete.eq(false))
            .join(JoinType::Join, user_role::Relation::User.def().rev())
            .join(JoinType::Join, user_role::Relation::Role.def())
            .join(JoinType::Join, role_access::Relation::Role.def().rev())
            .join(JoinType::Join, role_access::Relation::Access.def())
            .count(db)
            .await?;
        if count.gt(&0) {
            Ok(())
        } else {
            Err(WebError::InternalError {
                msg: "No access.".to_string(),
            })
        }
    }

    pub async fn update_user_role(
        db: &DatabaseConnection,
        id: i32,
        updated_model: &PostUserRole,
    ) -> WebResult<()> {
        let mut model: user_role::ActiveModel = Self::get_by_id(db, id).await?.into();
        model.role_id = ActiveValue::Set(updated_model.role_id);
        model.user_id = ActiveValue::Set(updated_model.user_id);
        model.save(db).await?;
        Ok(())
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> WebResult<user_role::Model> {
        let model = user_role::Entity::find_by_id(id)
            .filter(user_role::Column::IsDelete.eq(false))
            .one(db)
            .await?
            .ok_or(Custom("User role not found.".to_owned()))?;
        Ok(model)
    }
    pub async fn delete(db: &DatabaseConnection, id: i32) -> WebResult<()> {
        let mut model: user_role::ActiveModel = Self::get_by_id(db, id).await?.into();
        model.is_delete = ActiveValue::Set(Some(true));
        model.save(db).await?;
        Ok(())
    }

    pub async fn create(db: &DatabaseConnection, schema: &CreateUserRole) -> WebResult<()> {
        let model = user_role::Entity::find()
            .filter(user_role::Column::IsDelete.eq(false))
            .filter(user_role::Column::RoleId.eq(schema.role_id))
            .filter(user_role::Column::UserId.eq(schema.user_id))
            .one(db)
            .await?;
        if model.is_some() {
            Err(WebError::InternalError {
                msg: "This user already configurated the role.".to_owned(),
            })
        } else {
            user_role::ActiveModel {
                user_id: ActiveValue::Set(schema.user_id),
                role_id: ActiveValue::Set(schema.role_id),
                ..Default::default()
            }
            .save(db)
            .await?;
            Ok(())
        }
    }
}
