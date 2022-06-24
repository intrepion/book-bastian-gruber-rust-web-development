fn main() {
    struct Book {
        title: String,
        isbn: Option<String>,
    }
 
    let book = Book { 
        title: "Great book".to_string(), 
        isbn: Some(String::from("1-123-456")) 
    };
    
    match book.isbn {
        Some(i) => println!("The ISBN of the book: {} is: {}", book.title, i),
        None => println!("We don’t know the ISBN of the book"),
    }
}
