use rstest::rstest;

use std::sync::Arc;

use tower::util::ServiceExt;

use kernel::{
    model::{
        book::{Author, Book, BookId, Description, Isbn, Title},
        list::PaginatedList,
        user::{BookOwner, UserId, UserName},
    },
    repository::book::MockBookRepository,
};
use uuid::Uuid;

use api::model::book::PaginatedBookResponse;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use crate::{
    deserialize_json,
    helper::{fixture, make_router, v1, TestRequestExt},
};

#[rstest]
#[case("/books", 20, 0)]
#[case("/books?limit=50", 50, 0)]
#[case("/books?limit=50&offset=20", 50, 20)]
#[case("/books?offset=20", 20, 20)]
#[tokio::test]
async fn show_book_list_with_query_200(
    // fixture として mock オブジェクトを渡す
    mut fixture: registry::MockAppRegistryExt,
    #[case] path: &str,
    #[case] expected_limit: i64,
    #[case] expected_offset: i64,
) -> anyhow::Result<()> {
    let book_id = BookId::new(Uuid::new_v4());

    // モックの挙動を設定
    fixture.expect_book_repository().returning(move || {
        let mut mock = MockBookRepository::new();
        let book_id = book_id.clone();

        mock.expect_find_all().returning(move |opt| {
            let book_id = book_id.clone();
            let items = vec![Book {
                book_id,
                title: Title::new("RustによるWebアプリケーション開発".to_string()),
                author: Author::new("Yuki Toyoda".to_string()),
                isbn: Isbn::new("978-4-06-536957-9".to_string()),
                description: Description::new("RustによるWebアプリケーション開発".to_string()),
                owner: BookOwner {
                    user_id: UserId::new(Uuid::new_v4()),
                    user_name: UserName::new("Yuki Toyoda".to_string()),
                },
                checkout: None,
            }];
            Ok(PaginatedList {
                total: 1,
                limit: opt.limit,
                offset: opt.offset,
                items,
            })
        });

        Arc::new(mock)
    });

    // ルーターを作成
    let app: axum::Router = make_router(fixture);

    // リクエストを作成・送信し、レスポンスのステータスコードを検証する
    let req = Request::get(&v1(path)).bearer().body(Body::empty())?;
    let res = app.oneshot(req).await?;
    assert_eq!(res.status(), StatusCode::OK);

    // レスポンスの値を検証する
    let result = deserialize_json!(res, PaginatedBookResponse);
    assert_eq!(result.limit, expected_limit);
    assert_eq!(result.offset, expected_offset);

    Ok(())
}

#[rstest]
#[case("/books?limit=-1")]
#[case("/books?offset=aaa")]
#[tokio::test]
async fn show_book_list_with_query_400(
    mut fixture: registry::MockAppRegistryExt,
    #[case] path: &str,
) -> anyhow::Result<()> {
    // ルーターを作成
    let app: axum::Router = make_router(fixture);

    // リクエストを作成・送信し、レスポンスのステータスコードを検証する
    let req = Request::get(&v1(path)).bearer().body(Body::empty())?;
    let res = app.oneshot(req).await?;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    Ok(())
}
