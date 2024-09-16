use async_trait::async_trait;
use shared::error::AppResult;

use crate::model::{
    book::{event::CreateBook, Book},
    id::BookId,
};

#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn create(&self, event: CreateBook) -> AppResult<()>;
    async fn find_all(&self) -> AppResult<Vec<Book>>;
    async fn find_by_id(&self, book_id: BookId) -> AppResult<Option<Book>>;
}
