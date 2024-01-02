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
    Name,
    Password,
}

#[derive(DeriveIden)]
enum TimeEntries {
    TimeEntries,
    Id,
    JobNumber,
    JobDescription,
    WorkCode,
    EmployeeId,
    WeekEndDate,
    HoursWorked,
    SubmitStatus,
}

#[derive(Iden, EnumIter)]
pub enum Status {
    Table,
    #[iden = "Initial"]
    Initial,
    #[iden = "Submitted"]
    Submitted,
    #[iden = "Approved"]
    Approved,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Status::Table)
                    .values(Status::iter().skip(1))
                    .to_owned(),
            )
            .await?;
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
                    .col(ColumnDef::new(Users::Name).string().unique_key().not_null())
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(TimeEntries::TimeEntries)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TimeEntries::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TimeEntries::JobNumber).integer().not_null())
                    .col(
                        ColumnDef::new(TimeEntries::JobDescription)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TimeEntries::WorkCode).integer().not_null())
                    .col(ColumnDef::new(TimeEntries::EmployeeId).integer().not_null())
                    .col(ColumnDef::new(TimeEntries::WeekEndDate).date().not_null())
                    .col(ColumnDef::new(TimeEntries::HoursWorked).float().not_null())
                    .col(
                        ColumnDef::new(TimeEntries::SubmitStatus)
                            .enumeration(Status::Table, Status::iter().skip(1))
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-timelogs-user_id")
                            .from(TimeEntries::TimeEntries, TimeEntries::EmployeeId)
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
                    .table(TimeEntries::TimeEntries)
                    .name("fk-timelogs-user_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(TimeEntries::TimeEntries).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Users).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(Status::Table).to_owned())
            .await
    }
}
