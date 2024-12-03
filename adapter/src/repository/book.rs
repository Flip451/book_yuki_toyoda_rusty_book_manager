use async_trait::async_trait;
use derive_new::new;
use kernel::model::value_object::ValueObject;
use kernel::repository::book::{BookRepositoryError, BookRepositoryResult};
use kernel::{
    model::book::{event::CreateBook, Book, BookId},
    repository::book::BookRepository,
};

use crate::database::model::book::{BookRow, BookRowError};
use crate::database::ConnectionPool;

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn create(&self, event: CreateBook) -> BookRepositoryResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO books (title, author, isbn, description)
            VALUES ($1, $2, $3, $4)
            "#,
            event.title.inner_ref(),
            event.author.inner_ref(),
            event.isbn.inner_ref(),
            event.description.inner_ref(),
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(|e| BookRepositoryError::Unexpected(Box::new(e)))?;
        Ok(())
    }

    async fn find_all(&self) -> BookRepositoryResult<Vec<Book>> {
        let rows = sqlx::query_as!(
            BookRow,
            r#"SELECT book_id, title, author, isbn, description FROM books ORDER BY created_at DESC"#
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| BookRepositoryError::Unexpected(Box::new(e)))?;

        rows.into_iter()
            .map(|row: BookRow| row.try_into())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e: BookRowError| BookRepositoryError::InvalidSavedEntity(e.into()))
    }

    async fn find_by_id(&self, book_id: &BookId) -> BookRepositoryResult<Option<Book>> {
        let row = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    book_id,
                    title,
                    author,
                    isbn,
                    description
                FROM books
                WHERE book_id = $1
            "#,
            book_id.inner_ref(),
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(|e| BookRepositoryError::Unexpected(Box::new(e)))?;

        row.map(|row| row.try_into())
            .transpose()
            .map_err(|e: BookRowError| BookRepositoryError::InvalidSavedEntity(e.into()))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use kernel::model::book::{Author, Description, Isbn, Title};

    use super::*;

    #[sqlx::test]
    #[ignore]
    async fn test_register_book(pool: sqlx::PgPool) -> Result<()> {
        let repo = BookRepositoryImpl::new(ConnectionPool::new(pool));
        let book = CreateBook::new(
            Title::try_from("test title".to_string())?,
            Author::try_from("test author".to_string())?,
            Isbn::try_from("test isbn".to_string())?,
            Description::try_from("test description".to_string())?,
        );

        repo.create(book).await?;

        let res = repo.find_all().await?;
        assert_eq!(res.len(), 1);

        let book_id = &res[0].book_id;

        let res = repo.find_by_id(book_id).await?;
        assert!(res.is_some());

        let Book {
            book_id: id,
            title,
            author,
            isbn,
            description,
        } = res.ok_or(anyhow::anyhow!("book not found"))?;

        assert_eq!(book_id.inner_ref(), id.inner_ref());
        assert_eq!(title.inner_ref(), "test title");
        assert_eq!(author.inner_ref(), "test author");
        assert_eq!(isbn.inner_ref(), "test isbn");
        assert_eq!(description.inner_ref(), "test description");

        Ok(())
    }
}
