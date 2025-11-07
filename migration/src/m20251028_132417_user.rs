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
                    .col(string(User::Id).unique_key().not_null().primary_key().string_len(255))
                    .col(string(User::Email).unique_key().not_null().string_len(255))
                    .col(string(User::Username).not_null().string_len(255))
                    .col(string(User::Password).not_null().string_len(255))
                    .col(string(User::Token).unique_key().not_null().string_len(255))
                    .col(big_integer(User::Timestamp).unique_key().not_null())
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
