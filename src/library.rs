#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    is_available: bool,
}
#[derive(Debug)]
struct Library {
    name: String,
    address: String,
    book: Option<Book>,
}
#[derive(Debug)]
enum LibraryError {
    BookNotAvailable,
    BookNotFound,
    AlreadyBorrowed,
    BookAlreadyExists,
}

impl Book {
    fn borrow(&mut self) -> Result<&mut Book, LibraryError> {
        if self.is_available == true {
            self.is_available = false;
            Ok(self)
        } else {
            Err(LibraryError::AlreadyBorrowed)
        }
    }
    fn return_book(&mut self) {
        self.is_available = true;
    }
}

impl Library {
    fn add_book(&mut self, book: Book) -> Result<(), LibraryError> {
        if self.book.is_none() {
            self.book = Some(book);
            Ok(())
        } else {
            Err(LibraryError::BookAlreadyExists)
        }
    }

    fn borrow_book(&mut self) -> Result<&mut Book, LibraryError> {
        if let Some(book) = self.book.as_mut() {
            match book.borrow() {
                Ok(borrowed_book) => Ok(borrowed_book),
                Err(err) => Err(err),
            }
        } else {
            Err(LibraryError::BookNotFound)
        }
    }
    fn return_book(&mut self) -> Result<(), LibraryError> {
        if let Some(book) = self.book.as_mut() {
            book.return_book();
            Ok(())
        } else {
            Err(LibraryError::BookNotFound)
        }
    }
}

fn main() {
    let book = Book {
        title: String::from("The Rust Book"),
        author: String::from("Steve Klabnik"),
        is_available: true,
    };
    let mut library = Library {
        name: String::from("City Library"),
        address: String::from("123 Library Lane"),
        book: None, //assuming that there is only one book of The Rust Book in library
    };

    match library.add_book(book) {
        Ok(_) => {
            println!("Book added");
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }

    //Attempt to borrow the book and handle the result.

    match library.borrow_book() {
        Ok(result) => {
            println!("Borrowed book:{:?}", result);
        }
        Err(err) => {
            println!("Borrowed book:{:?}", err);
        }
    }

    //Try borrowing it again to simulate an error (book already borrowed).

    match library.borrow_book() {
        Ok(result) => {
            println!("Borrowed book:{:?}", result);
        }
        Err(err) => {
            println!("Borrowed book:{:?}", err);
        }
    }
    //Return the book and then borrow it again successfully.
    match library.return_book() {
        Ok(_) => {
            println!("Book Returned Successfully!");
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }

    match library.borrow_book() {
        Ok(result) => {
            println!("Borrowed book:{:?}", result);
        }
        Err(err) => {
            println!("Borrowed book:{:?}", err);
        }
    }
}
