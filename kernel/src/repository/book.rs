use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::model::book::{event::CreateBook, Book};

#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn create(&self, event: CreateBook) -> Result<()>;
    async fn find_all(&self) -> Result<Vec<Book>>;
    async fn find_by_id(&self, book_id: Uuid) -> Result<Option<Book>>;
}
