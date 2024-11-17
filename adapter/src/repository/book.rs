use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use kernel::model::value_object::ValueObject;
use kernel::{
    model::book::{event::CreateBook, Book, BookId},
    repository::book::BookRepository,
};

use crate::database::model::book::BookRow;
use crate::database::ConnectionPool;

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn create(&self, event: CreateBook) -> Result<()> {
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
        .await?;
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Book>> {
        let rows = sqlx::query_as!(
            BookRow,
            r#"SELECT book_id, title, author, isbn, description FROM books ORDER BY created_at DESC"#
        )
        .fetch_all(self.db.inner_ref())
        .await?;
        Ok(rows.into_iter().map(|row| row.into()).collect())
    }

    async fn find_by_id(&self, book_id: &BookId) -> Result<Option<Book>> {
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
        .await?;
        Ok(row.map(|row| row.into()))
    }
}

#[cfg(test)]
mod tests {
    use kernel::model::book::{Author, Description, Isbn, Title};

    use super::*;

    #[sqlx::test]
    async fn test_register_book(pool: sqlx::PgPool) -> Result<()> {
        let repo = BookRepositoryImpl::new(ConnectionPool::new(pool));
        let book = CreateBook::new(
            Title::new("test title".to_string()),
            Author::new("test author".to_string()),
            Isbn::new("test isbn".to_string()),
            Description::new("test description".to_string()),
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
        } = res.unwrap();

        assert_eq!(book_id, &id);
        assert_eq!(title.inner_ref(), "test title");
        assert_eq!(author.inner_ref(), "test author");
        assert_eq!(isbn.inner_ref(), "test isbn");
        assert_eq!(description.inner_ref(), "test description");

        Ok(())
    }
}
