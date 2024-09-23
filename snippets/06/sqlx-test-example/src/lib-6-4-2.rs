#[cfg(test)]
mod tests {
    #[sqlx::test(fixtures("common"))]
    async fn it_works(pool: sqlx::PgPool) {
        let row = sqlx::query!("SELECT author FROM books WHERE title = 'Test Book 1'")
            .fetch_one(&pool)
            .await
            .unwrap();
        let result = row.author;
        assert_eq!(result, "Test Author 1".to_string());
    }
}
