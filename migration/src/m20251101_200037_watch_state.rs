use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(WatchState::Table)
                    .if_not_exists()
                    .col(string(WatchState::WatchStateId).unique_key().not_null().primary_key().string_len(255))
                    .col(string(WatchState::UserId).not_null().string_len(255))
                    .col(string(WatchState::Source).not_null().string_len(255))
                    .col(string(WatchState::Id).not_null().string_len(255))
                    .col(integer(WatchState::SeasonIndex))
                    .col(integer(WatchState::EpisodeIndex))
                    .col(double(WatchState::CurrentTime))
                    .col(big_integer(WatchState::Timestamp))
                    .index(
                        Index::create()
                            .name("userid_source_id_seasonindex_episodeindex")
                            .col(WatchState::UserId)
                            .col(WatchState::Source)
                            .col(WatchState::Id)
                            .col(WatchState::SeasonIndex)
                            .col(WatchState::EpisodeIndex)
                            .unique()

                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WatchState::Table, WatchState::UserId).to(User::Table, User::Id)

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
enum WatchState {
    Table,
    WatchStateId,
    UserId,
    Source,
    Id,
    SeasonIndex,
    EpisodeIndex,
    CurrentTime,
    Timestamp,
}
