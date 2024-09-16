use super::{id::BookId, user::BookOwner};

pub mod event;

#[derive(Debug)]
pub struct Book {
    pub id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owner: BookOwner,
}

// ページネーションの範囲を指定するための設定値を格納する型
#[derive(Debug)]
pub struct BookListOptions {
    pub limit: i64,
    pub offset: i64,
}
