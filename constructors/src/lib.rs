pub struct Book {
    // 1. Define the fields of the struct
    // Make all of them public with `pub`
    // Read the description for the fields
    pub title: String,
    pub author: String,
    pub year: i32,
    pub likes: i32
}

impl Book {
    // 2. Define the `new` associated function
    pub fn new(title: &str, autor: &str, year: i32) -> Book {
        return Book { title: String::from(title), author: String::from(autor), year: year, likes: 0 };
    }
}
