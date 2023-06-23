use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Posts::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Posts::Title).string().not_null())
                    .col(ColumnDef::new(Posts::ShortDescription).string().not_null())
                    .col(ColumnDef::new(Posts::DetailedDescription).string().not_null())
                    .col(ColumnDef::new(Posts::Contacts).array(ColumnType::String(None)).not_null())
                    .col(ColumnDef::new(Posts::UserId).big_integer().not_null())
                    .col(ColumnDef::new(Posts::Category).small_unsigned().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Posts {
    Table,
    Id,
    Title,
    ShortDescription,
    DetailedDescription,
    Contacts,
    UserId,
    Category,
}
