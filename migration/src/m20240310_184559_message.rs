use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Message::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Message::Context).string().not_null())
                    .col(ColumnDef::new(Message::UserId).integer().not_null())
                    .col(ColumnDef::new(Message::GroupId).integer().not_null())
                    .col(ColumnDef::new(Message::IsRead).boolean().default(false))
                    .col(
                        ColumnDef::new(Message::CreateTime)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Message::UpdateTime)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Message::IsDelete).boolean().default(true))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Message {
    Table,
    Id,
    Context,
    UserId,
    GroupId,
    IsRead,
    CreateTime,
    UpdateTime,
    IsDelete,
}
