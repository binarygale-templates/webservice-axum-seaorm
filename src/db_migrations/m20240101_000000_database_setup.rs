use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(r#"create extension if not exists "uuid-ossp";"#)
            .await?;

        // SeaORM does not yet support automatically updating an updated_at
        // column, so these two functions are used to do that without having
        // to do manual work all the time.
        db.execute_unprepared(r#"
            create or replace function manage_updated_at(_tbl regclass) returns void as $$
            begin
            execute format(
                'create trigger set_updated_at before update on %s
                for each row execute procedure set_updated_at()',
                _tbl
            );
            end;
            $$ language plpgsql;

            create or replace function set_updated_at() returns trigger as $$
            begin
            if (new is distinct from old and new.updated_at is not distinct from old.updated_at) then
                new.updated_at := current_timestamp;
            end if;
            return new;
            end;
            $$ language plpgsql;
        "#).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(r#"drop extension if exists "uuid-ossp";"#)
            .await?;

        db.execute_unprepared(r#"drop function if exists manage_updated_at;"#)
            .await?;

        db.execute_unprepared(r#"drop function if exists set_updated_at;"#)
            .await?;

        Ok(())
    }
}
