# Assignment: Library Management System

Objective:
Build a simple library management system where you manage books, authors, and their availability using Structs and Enums.
You will also use Result and Option to handle errors and optional values.

Requirements:
Define Structs:

# Create a struct for Book with the following fields:

title: String
author: String
is_available: bool

# Create a struct for Library with the following fields:

name: String
address: String
book: Option (Assume the library manages only one book for simplicity).

# Define Enums:

# Create an enum called LibraryError with the following variants:

BookNotAvailable
BookNotFound
AlreadyBorrowed

# Implement Methods:

# For Book:

A method borrow that sets is_available to false. If the book is already borrowed, return LibraryError::AlreadyBorrowed wrapped in a Result.
A method return_book that sets is_available to true.
For Library:

A method add_book to add a new book to the library.
A method borrow_book that allows borrowing the book if it's available. If the book is missing, return LibraryError::BookNotFound.
A method return_book to return a borrowed book. If the book is missing, return LibraryError::BookNotFound.

# Main Function Logic:

Create a library named "City Library" at "123 Library Lane".
Add a book titled "The Rust Book" by "Steve Klabnik" to the library.
Attempt to borrow the book and handle the result.
Try borrowing it again to simulate an error (book already borrowed).
Return the book and then borrow it again successfully.
Handle and print appropriate error messages for each case using match.

# Example Output:

Added book: "The Rust Book" by Steve Klabnik
Borrowed: "The Rust Book"
Error: Book is already borrowed.
Returned: "The Rust Book"
Borrowed: "The Rust Book"

# Tips:

Use Result to handle methods that may fail (e.g., borrowing a book that's already borrowed).
Use Option to represent the absence or presence of a book in the library.
Use match for error handling and optional value handling
