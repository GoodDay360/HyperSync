pub use sea_orm_migration::prelude::*;

mod m20251028_132417_user;
mod m20251028_134907_favorite;
mod m20251101_200037_watch_state;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251028_132417_user::Migration),
            Box::new(m20251028_134907_favorite::Migration),
            Box::new(m20251101_200037_watch_state::Migration),
        ]
    }
}

