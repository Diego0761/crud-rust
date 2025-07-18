CREATE TABLE book (
                      title TEXT NOT NULL,
                      author TEXT NOT NULL,
                      isbn TEXT NOT NULL
);

CREATE UNIQUE INDEX book_isbn_idx ON book (isbn);