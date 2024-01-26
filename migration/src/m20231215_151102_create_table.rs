use crate::m20231214_151102_create_table::Projects;

use super::m20231213_151102_create_table::Users;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum TimeEntries {
    TimeEntries,
    Id,
    JobId,
    WorkCode,
    EmployeeId,
    DateOfWork,
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

#[derive(Iden, EnumIter)]
pub enum WorkCodes {
    Table,
    #[iden = "Meetings"]
    Meetings,
    #[iden = "SoftwareDev"]
    SoftwareDev,
    #[iden = "Checkout"]
    Checkout,
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
            .create_type(
                Type::create()
                    .as_enum(WorkCodes::Table)
                    .values(WorkCodes::iter().skip(1))
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
                    .col(
                        ColumnDef::new(TimeEntries::WorkCode)
                            .enumeration(WorkCodes::Table, Status::iter().skip(1))
                            .not_null(),
                    )
                    .col(ColumnDef::new(TimeEntries::JobId).integer().not_null())
                    .col(ColumnDef::new(TimeEntries::EmployeeId).integer().not_null())
                    .col(ColumnDef::new(TimeEntries::DateOfWork).date().not_null())
                    .col(ColumnDef::new(TimeEntries::HoursWorked).float().not_null())
                    .col(
                        ColumnDef::new(TimeEntries::SubmitStatus)
                            .enumeration(Status::Table, Status::iter().skip(1))
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-timelogs-project_id")
                            .from(TimeEntries::TimeEntries, TimeEntries::JobId)
                            .to(Projects::Projects, Projects::Id),
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
                    .name("fk-timelogs-project_id")
                    .to_owned(),
            )
            .await?;
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
            .drop_type(Type::drop().name(Status::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(WorkCodes::Table).to_owned())
            .await
    }
}
