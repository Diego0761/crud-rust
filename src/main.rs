use std::error::Error;
use sqlx::{Row, SqlitePool};

struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String
}

async fn create_book(book: &Book, pool: &sqlx::SqlitePool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

async fn delete_book(isbn: &str, pool: &sqlx::SqlitePool) -> Result<(), Box<dyn Error>> {
    let query = "DELETE FROM book WHERE isbn = $1";

    sqlx::query(query).bind(&isbn).execute(pool).await?;

    Ok(())
}

async fn update_book(book: &Book, isbn: &str, pool: &sqlx::SqlitePool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE book SET title = $1, author = $2 WHERE isbn = $3";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

async fn get_books(conn: &sqlx::SqlitePool) -> Result<Vec<Book>, Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";

    let query = sqlx::query(q);

    // single book
    // let row = query.fetch_one(conn).await?;
    //
    // let book = Book{
    //     title: row.get("title"),
    //     author: row.get("author"),
    //     isbn: row.get("isbn")
    // };

    let rows = query.fetch_all(conn).await?;

    let books = rows.iter().map(|row| {
        Book{
            title: row.get("title"),
            author: row.get("author"),
            isbn: row.get("isbn"),
        }
    }).collect();

    Ok(books)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    println!("Hello, world!");

    let mut config = sqlx::sqlite::SqliteConnectOptions::new();
    config = config.filename( "./data.db" );



    let pool = SqlitePool::connect_with(config).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let book = Book{
        title: "big book".to_string(),
        author: "big book author".to_string(),
        isbn: "1".to_string()
    };

    delete_book(&book.isbn, &pool).await?;

    create_book(&book, &pool).await?;

    let books = get_books(&pool).await?;

    for book in books {
        println!("Title: {}, Author: {}, ISBN: {}", book.title, book.author, book.isbn);
    }


    Ok(())
}
