use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Users {
    Users,
    Id,
    ManagerId,
    Name,
    Password,
}

#[derive(DeriveIden)]
enum Managers {
    Managers,
    Id,
    UserId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Users)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::ManagerId).integer())
                    .col(ColumnDef::new(Users::Name).string().unique_key().not_null())
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Managers::Managers)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Managers::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Managers::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-managers-user_id")
                            .from(Managers::Managers, Managers::UserId)
                            .to(Users::Users, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Managers::Managers)
                    .name("fk-managers-user_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Managers::Managers).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Users).to_owned())
            .await
    }
}
