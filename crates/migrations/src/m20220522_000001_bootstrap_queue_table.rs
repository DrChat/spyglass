use entities::sea_orm::{ConnectionTrait, Statement};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220522_000001_bootstrap_queue_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let new_table = r#"
            CREATE TABLE IF NOT EXISTS "bootstrap_queue" (
                "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,
                "seed_url" text NOT NULL UNIQUE,
                "count" integer NOT NULL DEFAULT 0,
                "created_at" text NOT NULL,
                "updated_at" text NOT NULL);"#;

        // Create lens table
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                new_table.to_owned().to_string(),
            ))
            .await?;
        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
