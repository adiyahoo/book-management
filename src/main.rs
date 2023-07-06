use mysql::*;
use mysql::prelude::*;
use std::env;
use std::io;

const HOST: &str = "localhost";
const USERNAME: &str = "root"; 
const PASSWORD: &str = ""; 
const DATABASE: &str = "aplikasi_buku"; 

mod my_table {
    use mysql::prelude::Queryable;

    pub fn create_table(conn: &mut mysql::PooledConn) {
        let book_query = "
            CREATE TABLE book (
                id INT AUTO_INCREMENT PRIMARY KEY,
                name_book VARCHAR(255) NOT NULL, 
                genre VARCHAR(255) NOT NULL, 
                author VARCHAR(255) NOT NULL
            )   
        ";

        match conn.query_drop(book_query) {
            Ok(_) => println!("Berhasil membuat table"),

            Err(err) => println!("Terjadi kesalahan saya membuat table: {}", err),
        }
    }
}

struct Book {
    name_book: String,
    genre: String,
    author: String,
}

impl Book {
    fn new(name_book: String, genre: String, author: String) -> Self {
        Book {
            name_book,
            genre,
            author,
        }
    }

    pub fn input_book(conn: &mut mysql::PooledConn) {
        let mut name_book = String::new();
        let mut genre_book = String::new();
        let mut author_book = String::new();

        loop {
            println!("Input name book: ");
            io::stdin()
                .read_line(&mut name_book)
                .expect("Error in input name book");

            if name_book.trim().is_empty() {
                println!("Name book can`t empty");
            } else {
                break;
            }
        }

        loop {
            println!("Input genre book: ");
            io::stdin()
                .read_line(&mut genre_book)
                .expect("Error in input genre book");

            if genre_book.trim().is_empty() {
                println!("Genre book can`t empty");
            } else {
                break;
            }
        }

        loop {
            println!("Input author book: ");
            io::stdin()
                .read_line(&mut author_book)
                .expect("Error in input author book");

            if author_book.trim().is_empty() {
                println!("Author book can`t empty");
            } else {
                break;
            }
        }

        let data = Book::new(name_book, genre_book, author_book);

        data.add_book(conn);
    }

    fn add_book(&self, conn: &mut mysql::PooledConn) {
        let insert_query =
            "INSERT INTO book (name_book, genre, author) VALUES(:name_book, :genre, :author)";

        let params = params! {
            "name_book" => &self.name_book.trim(),
            "genre" => &self.genre.trim(),
            "author" => &self.author.trim()
        };

        let result = conn.exec_drop(insert_query, params);

        match result {
            Ok(_) => {
                println!("Book added successfully");
            }
            Err(err) => {
                println!("There was an error adding Book data: {}", err);
            }
        }
    }

    pub fn delete_book(conn: &mut mysql::PooledConn) {
        let mut input_value = String::new();

        loop {
            println!("Input id column to delete: ");

            io::stdin()
                .read_line(&mut input_value)
                .expect("Error in input id delete book");

            if input_value.trim().is_empty() {
                continue;
            } else {
                break;
            }
        }

        let result_parse = input_value
            .trim()
            .parse::<i32>()
            .expect("Error in parse input_value to int");

        let delete_query = "DELETE FROM book WHERE id = :id";

        let params = params! {
            "id" => result_parse
        };

        let result = conn.exec_drop(delete_query, params);

        match result {
            Ok(_) => println!("Successfully deleted data by id: {}", result_parse),

            Err(err) => println!("{}", err),
        }
    }

    fn select_books(conn: &mut mysql::PooledConn) {
        let select_query = "SELECT id, name_book, genre, author FROM book";

        match conn.query_iter(select_query) {
            Ok(mut result) => {
                if let Some(row) = result.next() {
                    match row {
                        Ok(row) => {
                            let id: u32 = row.get("id").unwrap();
                            let name: String = row.get("name_book").unwrap();
                            let genre: String = row.get("genre").unwrap();
                            let author: String = row.get("author").unwrap();

                            println!(
                                "ID: {}, Name: {}, Genre: {}, Author: {}",
                                id, name, genre, author
                            );
                        }
                        Err(err) => {
                            eprintln!("Error reading row: {}", err);
                        }
                    }
                } else {
                    println!("No books found.");
                }
            }
            Err(err) => {
                eprintln!("Error executing query: {}", err);
            }
        }
    }
}

fn help_command() { 
    println!("Using: book add, for add data");
    println!("Using: book delete. for delete data");
    println!("Using: book select, for select data"); 
    println!("Using: create_table to create table"); 
}

fn match_command(args: &[String], conn: &mut mysql::PooledConn) {
    match args.get(1) {
        Some(cmd) => match cmd.as_str() {
            "add" => Book::input_book(conn),
            "delete" => Book::delete_book(conn),
            "select" => Book::select_books(conn),
            "create_table" => my_table::create_table(conn),
            "help" => help_command(),
            _ => println!("Unknown command, using > book help"),
        },

        None => println!("Unknown command, using > book help"),
    }
}

fn main() {
    println!("Management Book Application");
    let url = format!("mysql://{}:{}@{}:3306/{}", USERNAME, PASSWORD, HOST, DATABASE);
    let pool = Pool::new(url.as_str()).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let command: Vec<String> = env::args().collect();

    match_command(&command, &mut conn);
}
