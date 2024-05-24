use entity::prelude::Access;
use entity::user::Entity;
use entity::{role, role_access, user, user_role};
use pkg::{WebError, WebResult};
use sea_orm::prelude::DateTime;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Select,
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

impl Service {
    pub async fn get_user_roles(
        db: &DatabaseConnection,
        page: u64,
        page_size: u64,
    ) -> WebResult<(Vec<UserRoles>, u64)> {
        let paginator = Self::get_select_entity()
            .await
            .into_model()
            .paginate(db, page_size);

        let count = Self::get_select_entity().await.count(db).await?;

        let user_roles: Vec<UserRoles> = paginator.fetch_page(page - 1).await?;
        Ok((user_roles, count))
    }

    pub async fn get_select_entity() -> Select<Entity> {
        user::Entity::find()
            .filter(user::Column::IsDelete.eq(false))
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
        user_role: &PostUserRole,
    ) -> WebResult<()> {
        let user_role = user_role::ActiveModel {
            id: ActiveValue::Set(user_role.id),
            user_id: ActiveValue::Set(user_role.user_id),
            role_id: ActiveValue::Set(user_role.role_id),
            ..Default::default()
        };

        user_role.update(db).await?;
        Ok(())
    }
}
