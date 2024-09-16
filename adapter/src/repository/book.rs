use async_trait::async_trait;
use derive_new::new;
// 新たに定義した型を追加で use する
use crate::database::model::book::BookCheckoutRow;
use crate::database::model::book::{BookRow, PaginatedBookRow};
use crate::database::ConnectionPool;
use kernel::model::book::Checkout;
use kernel::model::{
    id::{BookId, UserId},
    {book::event::DeleteBook, list::PaginatedList},
};
use kernel::{
    model::book::{
        event::{CreateBook, UpdateBook},
        Book, BookListOptions,
    },
    repository::book::BookRepository,
};
use shared::error::{AppError, AppResult};
use std::collections::HashMap;

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn create(&self, event: CreateBook, user_id: UserId) -> AppResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO books (title, author, isbn, description, user_id)
                VALUES ($1, $2, $3, $4, $5)
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description,
            user_id as _
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;
        Ok(())
    }

    // find_all はページネーションを実装することにしたので内容は以前より複雑化している。
    // ステップとしては、
    // 1. 指定した limit と offset の範囲に該当する蔵書 ID のリストと総件数を取得する
    // 2. 対象の蔵書 ID から蔵書のレコードデータを取得する
    // 3． 取得したデータを戻り値の型に合うよう整えて返す
    //
    // 総件数（total）を取得するとき、1 つ目のクエリのレコードのカラムに総件数が含まれる
    // ような実装であるため、このクエリ結果のレコードが 0 件のときは総件数も取得できない。
    // そのときは、総件数も 0 件として返す仕様としている。
    async fn find_all(&self, options: BookListOptions) -> AppResult<PaginatedList<Book>> {
        let BookListOptions { limit, offset } = options;
        let rows: Vec<PaginatedBookRow> = sqlx::query_as!(
            PaginatedBookRow,
            r#"
                SELECT
                COUNT(*) OVER() AS "total!",
                b.book_id AS id
                FROM books AS b
                ORDER BY b.created_at DESC
                LIMIT $1
                OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        let total = rows.first().map(|r| r.total).unwrap_or_default(); // レコードが 1 つもないときは total も 0 にする
        let book_ids = rows.into_iter().map(|r| r.id).collect::<Vec<BookId>>();

        let rows: Vec<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    b.book_id AS book_id,
                    b.title AS title,
                    b.author AS author,
                    b.isbn AS isbn,
                    b.description AS description,
                    u.user_id AS owned_by,
                    u.name AS owner_name
                FROM books AS b
                INNER JOIN users AS u USING(user_id)
                WHERE b.book_id IN (SELECT * FROM UNNEST($1::uuid[]))
                ORDER BY b.created_at DESC
            "#,
            &book_ids as _
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        let book_ids = rows.iter().map(|book| book.book_id).collect::<Vec<_>>();
        let mut checkouts = self.find_checkouts(&book_ids).await?;
        let items = rows
            .into_iter()
            .map(|row| {
                let checkout = checkouts.remove(&row.book_id);
                row.into_book(checkout)
            })
            .collect();

        Ok(PaginatedList {
            total,
            limit,
            offset,
            items,
        })
    }

    // find_by_id は、コードのシグネチャ自体は変わっていないが、
    // BookRow のフィールドに所有者の情報を含むようになったので
    // SQL のクエリを変更している。
    async fn find_by_id(&self, book_id: BookId) -> AppResult<Option<Book>> {
        let row: Option<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    b.book_id AS book_id,
                    b.title AS title,
                    b.author AS author,
                    b.isbn AS isbn,
                    b.description AS description,
                    u.user_id AS owned_by,
                    u.name AS owner_name
                FROM books AS b
                INNER JOIN users AS u USING(user_id)
                WHERE b.book_id = $1
            "#,
            book_id as _
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        match row {
            Some(r) => {
                let checkout = self.find_checkouts(&[r.book_id]).await?.remove(&r.book_id);
                Ok(Some(r.into_book(checkout)))
            }
            None => Ok(None),
        }
    }

    // update は SQL の UPDATE 文に当てはめているだけであるが、
    // 内容を変更できるのは所有者のみとするため、
    // SQL クエリの WHERE 条件を book_id と user_id の複合条件としている。
    async fn update(&self, event: UpdateBook) -> AppResult<()> {
        let res = sqlx::query!(
            r#"
                UPDATE books
                SET
                    title = $1,
                    author = $2,
                    isbn = $3,
                    description = $4
                WHERE book_id = $5
                AND user_id = $6
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description,
            event.book_id as _,
            event.requested_user as _
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;
        if res.rows_affected() < 1 {
            return Err(AppError::EntityNotFound("specified book not found".into()));
        }

        Ok(())
    }

    // update と同様に、delete も所有者のみが行えるよう、
    // SQL クエリの WHERE の条件には book_id と user_id の複合条件にしている。
    async fn delete(&self, event: DeleteBook) -> AppResult<()> {
        let res = sqlx::query!(
            r#"
                DELETE FROM books
                WHERE book_id = $1
                AND user_id = $2
            "#,
            event.book_id as _,
            event.requested_user as _
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::EntityNotFound("specified book not found".into()));
        }

        Ok(())
    }
}

impl BookRepositoryImpl {
    // 指定された book_id が貸出中の場合に貸出情報を返すメソッドを追加する
    async fn find_checkouts(&self, book_ids: &[BookId]) -> AppResult<HashMap<BookId, Checkout>> {
        let res = sqlx::query_as!(
            BookCheckoutRow,
            r#"
                SELECT
                c.checkout_id,
                c.book_id,
                u.user_id,
                u.name AS user_name,
                c.checked_out_at
                FROM checkouts AS c
                INNER JOIN users AS u USING(user_id)
                WHERE book_id = ANY($1)
                ;
            "#,
            book_ids as _
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?
        .into_iter()
        .map(|checkout| (checkout.book_id, Checkout::from(checkout)))
        .collect();

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::user::UserRepositoryImpl;
    use kernel::{model::user::event::CreateUser, repository::user::UserRepository};

    #[sqlx::test]
    async fn test_register_book(pool: sqlx::PgPool) -> anyhow::Result<()> {
        // 蔵書のデータを追加・取得するためにはユーザー情報がないといけないため
        // テストコードのほうでもロールおよびユーザー情報を追加するコードを足した。
        // テストコードで、このようなデータベースにあらかじめデータを追加しておくために
        // fixture という機能が便利であるが、次章で解説するためここでは愚直な実装としておく。
        sqlx::query!(r#"INSERT INTO roles(name) VALUES ('Admin'), ('User');"#)
            .execute(&pool)
            .await?;
        let user_repo = UserRepositoryImpl::new(ConnectionPool::new(pool.clone()));
        let repo = BookRepositoryImpl::new(ConnectionPool::new(pool.clone()));
        let user = user_repo
            .create(CreateUser {
                name: "Test User".into(),
                email: "test@example.com".into(),
                password: "test_password".into(),
            })
            .await?;
        let book = CreateBook {
            title: "Test Title".into(),
            author: "Test Author".into(),
            isbn: "Test ISBN".into(),
            description: "Test Description".into(),
        };
        repo.create(book, user.id).await?;
        // find_all を実行するためには BookListOptions 型の値が必要なので作る。
        let options = BookListOptions {
            limit: 20,
            offset: 0,
        };
        let res = repo.find_all(options).await?;
        assert_eq!(res.items.len(), 1);
        let book_id = res.items[0].id;
        let res = repo.find_by_id(book_id).await?;
        assert!(res.is_some());
        let Book {
            id,
            title,
            author,
            isbn,
            description,
            owner,
            ..
        } = res.unwrap();
        assert_eq!(id, book_id);
        assert_eq!(title, "Test Title");
        assert_eq!(author, "Test Author");
        assert_eq!(isbn, "Test ISBN");
        assert_eq!(description, "Test Description");
        assert_eq!(owner.name, "Test User");
        Ok(())
    }
}
