use sea_orm::{ActiveValue, EntityTrait};
use sea_orm_migration::{
    prelude::*,
    schema::{text_null, timestamp_with_time_zone, uuid},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Example::Table)
            .if_not_exists()
            .col(
                uuid(Example::Uuid)
                    .primary_key()
                    .extra("default uuid_generate_v4()"),
            )
            .col(timestamp_with_time_zone(Example::CreatedAt).default(Expr::current_timestamp()))
            .col(timestamp_with_time_zone(Example::UpdatedAt).default(Expr::current_timestamp()))
            .col(text_null(Example::Content))
            .to_owned();

        manager.create_table(table).await?;

        manager
            .get_connection()
            .execute_unprepared(r#"select manage_updated_at('example');"#)
            .await?;

        // The following block is only there to insert a placeholder value
        // into the table so there is something to query.
        use crate::entities::example::ActiveModel as ExampleModel;
        use crate::entities::example::Entity as ExampleEntity;
        let example_entry = ExampleModel {
            content: ActiveValue::Set(Some("Hello, World".to_owned())),
            ..Default::default()
        };
        ExampleEntity::insert(example_entry)
            .exec(manager.get_connection())
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Example::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Example {
    Table,
    CreatedAt,
    UpdatedAt,
    Uuid,
    Content,
}
