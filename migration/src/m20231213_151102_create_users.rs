use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Users {
    Users,
    Id,
    ManagerId,
    FirstName,
    LastName,
    UserName,
    Password,
    AccessLevel,
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
                    .col(ColumnDef::new(Users::ManagerId).integer().not_null())
                    .col(ColumnDef::new(Users::FirstName).string().not_null())
                    .col(ColumnDef::new(Users::LastName).string().not_null())
                    .col(
                        ColumnDef::new(Users::UserName)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .col(
                        ColumnDef::new(Users::AccessLevel)
                            .small_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-users-manager_id")
                            .from(Users::Users, Users::ManagerId)
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
                    .table(Users::Users)
                    .name("fk-users-manager_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Users).to_owned())
            .await
    }
}
