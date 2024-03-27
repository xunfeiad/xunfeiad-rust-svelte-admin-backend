use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RoleDomain::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RoleDomain::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RoleDomain::RoleId).integer().not_null())
                    .col(ColumnDef::new(RoleDomain::DomainId).integer().not_null())
                    .col(
                        ColumnDef::new(RoleDomain::CreateTime)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(RoleDomain::UpdateTime)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(RoleDomain::IsDelete).boolean().default(true))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RoleDomain::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RoleDomain {
    Table,
    Id,
    RoleId,
    DomainId,
    CreateTime,
    UpdateTime,
    IsDelete,
}
