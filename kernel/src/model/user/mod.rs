// kernel/src/model/user/mod.rs
use crate::model::{id::UserId, role::Role};
pub mod event;

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub role: Role,
}

#[derive(Debug)]
pub struct BookOwner {
    pub id: UserId,
    pub name: String,
}

#[derive(Debug)]
pub struct CheckoutUser {
    pub id: UserId,
    pub name: String,
}
