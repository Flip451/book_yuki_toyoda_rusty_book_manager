use chrono::{DateTime, Utc};
use derive_new::new;
use garde::Validate;
use kernel::model::{
    book::{
        event::{CreateBook, UpdateBook},
        Author, AuthorError, Book, BookId, BookListOptions, Checkout, Description,
        DescriptionError, Isbn, IsbnError, Title, TitleError,
    },
    list::PaginatedList,
    user::{CheckoutUser, UserId},
    value_object::ValueObject,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use super::user::BookOwner;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookRequest {
    #[garde(length(min = 1))]
    pub title: String,
    #[garde(length(min = 1))]
    pub author: String,
    #[garde(length(min = 1))]
    pub isbn: String,
    #[garde(skip)]
    pub description: String,
}

impl TryFrom<CreateBookRequest> for CreateBook {
    type Error = CreateBookRequestError;

    fn try_from(request: CreateBookRequest) -> Result<Self, Self::Error> {
        Ok(CreateBook {
            title: Title::try_from(request.title)?,
            author: Author::try_from(request.author)?,
            isbn: Isbn::try_from(request.isbn)?,
            description: Description::try_from(request.description)?,
        })
    }
}

#[derive(Debug, Error)]
pub enum CreateBookRequestError {
    #[error("invalid title: {0}")]
    InvalidTitle(#[from] TitleError),

    #[error("invalid author: {0}")]
    InvalidAuthor(#[from] AuthorError),

    #[error("invalid isbn: {0}")]
    InvalidIsbn(#[from] IsbnError),

    #[error("invalid description: {0}")]
    InvalidDescription(#[from] DescriptionError),
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBookRequest {
    #[garde(length(min = 1))]
    pub title: String,
    #[garde(length(min = 1))]
    pub author: String,
    #[garde(length(min = 1))]
    pub isbn: String,
    #[garde(skip)]
    pub description: String,
}

#[derive(new)]
pub struct UpdateBookRequestWithIds(BookId, UserId, UpdateBookRequest);

impl TryFrom<UpdateBookRequestWithIds> for UpdateBook {
    type Error = UpdateBookRequestError;

    fn try_from(value: UpdateBookRequestWithIds) -> Result<Self, Self::Error> {
        let UpdateBookRequestWithIds(
            book_id,
            user_id,
            UpdateBookRequest {
                title,
                author,
                isbn,
                description,
            },
        ) = value;
        Ok(UpdateBook {
            book_id,
            title: title.try_into()?,
            author: author.try_into()?,
            isbn: isbn.try_into()?,
            description: description.try_into()?,
            requested_by: user_id,
        })
    }
}

#[derive(Debug, Error)]
pub enum UpdateBookRequestError {
    #[error("invalid title: {0}")]
    InvalidTitle(#[from] TitleError),

    #[error("invalid author: {0}")]
    InvalidAuthor(#[from] AuthorError),

    #[error("invalid isbn: {0}")]
    InvalidIsbn(#[from] IsbnError),

    #[error("invalid description: {0}")]
    InvalidDescription(#[from] DescriptionError),
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BookListQuery {
    #[garde(range(min = 1))]
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[garde(range(min = 0))]
    #[serde(default)]
    pub offset: i64,
}

const DEFAULT_LIMIT: i64 = 20;
const fn default_limit() -> i64 {
    DEFAULT_LIMIT
}

impl From<BookListQuery> for BookListOptions {
    fn from(BookListQuery { limit, offset }: BookListQuery) -> Self {
        BookListOptions { limit, offset }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookResponse {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owner: BookOwner,
    pub checkout: Option<BookCheckoutResponse>,
}

impl From<Book> for BookResponse {
    fn from(book: Book) -> Self {
        let (book_id, title, author, isbn, description, owner, checkout) = book.dissolve();

        BookResponse {
            id: book_id.into_inner(),
            title: title.into_inner(),
            author: author.into_inner(),
            isbn: isbn.into_inner(),
            description: description.into_inner(),
            owner: owner.into(),
            checkout: checkout.map(BookCheckoutResponse::from),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedBookResponse {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub items: Vec<BookResponse>,
}

impl From<PaginatedList<Book>> for PaginatedBookResponse {
    fn from(paginated_book: PaginatedList<Book>) -> Self {
        let PaginatedList {
            total,
            limit,
            offset,
            items,
        } = paginated_book;

        PaginatedBookResponse {
            total,
            limit,
            offset,
            items: items.into_iter().map(BookResponse::from).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookCheckoutResponse {
    pub id: Uuid,
    pub checked_out_by: CheckoutUserResponse,
    pub checked_out_at: DateTime<Utc>,
}

impl From<Checkout> for BookCheckoutResponse {
    fn from(checkout: Checkout) -> Self {
        let (checkout_id, checkout_user, checked_out_at) = checkout.dissolve();

        BookCheckoutResponse {
            id: checkout_id.into_inner(),
            checked_out_by: checkout_user.into(),
            checked_out_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutUserResponse {
    pub id: Uuid,
    pub name: String,
}

impl From<CheckoutUser> for CheckoutUserResponse {
    fn from(checkout_user: CheckoutUser) -> Self {
        let CheckoutUser { user_id, user_name } = checkout_user;

        CheckoutUserResponse {
            id: user_id.into_inner(),
            name: user_name.into_inner(),
        }
    }
}
