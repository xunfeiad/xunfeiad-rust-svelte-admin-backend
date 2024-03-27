use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RoleAccess::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RoleAccess::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RoleAccess::RoleId).integer().not_null())
                    .col(ColumnDef::new(RoleAccess::AccessId).integer().not_null())
                    .col(
                        ColumnDef::new(RoleAccess::CreateTime)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(RoleAccess::UpdateTime)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(RoleAccess::IsDelete).boolean().default(true))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RoleAccess::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RoleAccess {
    Table,
    Id,
    RoleId,
    AccessId,
    CreateTime,
    UpdateTime,
    IsDelete,
}
