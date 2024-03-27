use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserGroup::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserGroup::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserGroup::UserId).integer().not_null())
                    .col(ColumnDef::new(UserGroup::GroupId).integer().not_null())
                    .col(
                        ColumnDef::new(UserGroup::CreateTime)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserGroup::UpdateTime)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(UserGroup::IsDelete).boolean().default(true))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserGroup::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserGroup {
    Table,
    Id,
    UserId,
    GroupId,
    CreateTime,
    UpdateTime,
    IsDelete,
}
