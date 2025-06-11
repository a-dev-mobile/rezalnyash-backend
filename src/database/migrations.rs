pub async fn run_migrations(pool: &PgPool) -> MigrationResult<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}