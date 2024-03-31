use entity::user::Column;
use entity::{role, user, user_role};
use sea_orm::sea_query::*;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::{ColumnTrait, PaginatorTrait};
use sea_orm::{DbBackend, EntityTrait, FromQueryResult, QuerySelect, QueryTrait, RelationTrait};
use serde::{Deserialize, Serialize};
use service::auth::user_role::UserRoles;

#[tokio::main]
async fn main() {
    // let query = Query::select()
    //     .columns([User::Id, User::Username, User::NickName])
    //     .columns([Role::RoleName])
    //     .from(User::Table)
    //     .from(UserRole::Table)
    //     .from(Role::Table)
    //     .left_join(
    //         UserRole::Table,
    //         Expr::col((User::Table, User::Id)).equals((UserRole::Table, UserRole::UserId)),
    //     )
    //     .left_join(
    //         Role::Table,
    //         Expr::col((Role::Table, Role::Id)).equals((UserRole::Table, UserRole::RoleId)),
    //     )
    //     .cond_where(
    //         Cond::all()
    //             .add(Expr::col((User::Table, User::IsDelete)).eq(false))
    //             .add(Expr::col((Role::Table, Role::IsDelete)).eq(false)),
    //     )
    //     .to_string(PostgresQueryBuilder);
    // println!("{query}")
    use service::get_db;
    let db = get_db().await.unwrap();
    let paginator = user::Entity::find()
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
        .count(&db)
        .await
        .unwrap();
    println!("{:?}", paginator)
}

#[derive(Serialize, Deserialize, FromQueryResult, Debug)]
struct AccountDTO {
    id: i32,
    username: String,
    role_name: String,
}
