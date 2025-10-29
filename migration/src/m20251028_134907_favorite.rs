use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Favorite::Table)
                    .if_not_exists()
                    .col(string(Favorite::UserId).not_null())
                    .col(string(Favorite::Source).not_null())
                    .col(string(Favorite::Id).not_null())
                    .col(json(Favorite::Tags))
                    .col(integer(Favorite::CurrentWatchSeasonIndex))
                    .col(integer(Favorite::CurrentWatchEpisodeIndex))
                    .col(big_integer(Favorite::Timestamp))
                    .primary_key(Index::create().col(Favorite::Source).col(Favorite::Id))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Favorite::Table, Favorite::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Favorite {
    Table,
    UserId,
    Source,
    Id,
    Tags,
    CurrentWatchSeasonIndex,
    CurrentWatchEpisodeIndex,
    Timestamp,

}
