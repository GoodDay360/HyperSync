use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(string(User::Id).unique_key().not_null().primary_key())
                    .col(string(User::Email).unique_key().not_null())
                    .col(string(User::Username).not_null())
                    .col(string(User::Password).not_null())
                    .col(string(User::Token).unique_key().not_null())
                    .col(integer(User::Timestamp).unique_key().not_null())
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Email,
    Username,
    Password,
    Token,
    Timestamp,
}
