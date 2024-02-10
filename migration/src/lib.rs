pub use sea_orm_migration::prelude::*;

mod m20231213_151102_create_users;
mod m20231214_151102_create_projects;
mod m20231215_151102_create_timelogs;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231213_151102_create_users::Migration),
            Box::new(m20231214_151102_create_projects::Migration),
            Box::new(m20231215_151102_create_timelogs::Migration),
        ]
    }
}
