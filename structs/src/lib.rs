pub struct Person {
    // Define fields here
    pub name: String,
    pub age: u8
    // Read the description
}

pub fn is_adult(p: &Person) -> bool {
    return p.age >= 18;
} // Finish the function
