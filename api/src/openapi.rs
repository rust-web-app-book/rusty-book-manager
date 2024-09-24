use crate::{handler, model};

#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        title = "Book App - 書籍『Rust による Web アプリケーション開発』向けのサンプルアプリケーション。",
        contact(
            name = "Rust による Web アプリケーション開発",
            url = "todo",
            email = "todo"
        ),
        description = r#"
## 概要
書籍『Rust による Web アプリケーション開発』向けに実装されたサンプルアプリケーションです。axum を基軸に、実務で Rust を利用するエンジニアが
どのように Rust を使ってアプリケーションを実装しているのかを示すことを目的とした本です。本番環境へのリリースはもちろんのこと、保守運用まで含めた実装を
Rust でしたいと思った際に、どのようなクレートや実装を利用できるのかまでを示すことを目標としています。
"#,
    ),
    paths(
        handler::health::health_check,
        handler::health::health_check_db,
        handler::book::show_book_list,
        handler::book::show_book,
        handler::book::register_book,
        handler::book::update_book,
        handler::book::delete_book,
        handler::checkout::checkout_book,
        handler::checkout::return_book,
        handler::checkout::checkout_history,
        handler::user::get_current_user,
        handler::auth::login,
        handler::auth::logout,
    ),
    components(schemas(
        model::book::CreateBookRequest,
        model::book::UpdateBookRequest,
        model::book::BookResponse,
        model::book::PaginatedBookResponse,
        model::book::BookCheckoutResponse,
        model::checkout::CheckoutsResponse,
        model::checkout::CheckoutResponse,
        model::checkout::CheckoutBookResponse,
        model::user::BookOwner,
        model::user::CheckoutUser,
        model::auth::LoginRequest,
        model::auth::AccessTokenResponse,
        kernel::model::id::BookId,
        kernel::model::id::UserId,
        kernel::model::id::CheckoutId,
    ))
)]
pub struct ApiDoc;
