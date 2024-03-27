use migration::{
    m20240310_171244_user::User, m20240310_174955_role::Role, m20240310_180627_user_role::UserRole,
};
use sea_orm::sea_query::*;

#[tokio::main]
async fn main() {
    let query = Query::select()
        .columns([User::Id, User::Username, User::NickName])
        .columns([Role::RoleName])
        .from(User::Table)
        .from(UserRole::Table)
        .from(Role::Table)
        .left_join(
            UserRole::Table,
            Expr::col((User::Table, User::Id)).equals((UserRole::Table, UserRole::UserId)),
        )
        .left_join(
            Role::Table,
            Expr::col((Role::Table, Role::Id)).equals((UserRole::Table, UserRole::RoleId)),
        )
        .cond_where(
            Cond::all()
                .add(Expr::col((User::Table, User::IsDelete)).eq(false))
                .add(Expr::col((Role::Table, Role::IsDelete)).eq(false)),
        )
        .to_string(PostgresQueryBuilder);
    println!("{query}")
}
