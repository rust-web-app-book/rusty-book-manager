#[cfg(test)]
mod tests {
    #[sqlx::test]
    async fn it_works(pool: sqlx::PgPool) {
        // 接続確認
        let row = sqlx::query!("SELECT 1 + 1 AS result")
            .fetch_one(&pool)
            .await
            .unwrap();
        let result = row.result;
        assert_eq!(result, Some(2));
    }
}
