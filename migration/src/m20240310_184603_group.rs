use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Group::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Group::GroupName).string().not_null())
                    .col(ColumnDef::new(Group::Owner).string().not_null())
                    .col(ColumnDef::new(Group::Title).string())
                    .col(ColumnDef::new(Group::PingMessage).string())
                    .col(ColumnDef::new(Group::CreateBy).string().not_null())
                    .col(
                        ColumnDef::new(Group::CreateTime)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Group::UpdateTime)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Group::IsDelete).boolean().default(true))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Group {
    Table,
    Id,
    GroupName,
    Owner,
    Title,
    PingMessage,
    CreateTime,
    CreateBy,
    UpdateTime,
    IsDelete,
}
