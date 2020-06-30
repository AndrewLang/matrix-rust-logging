#[derive(Debug)]
pub struct Post {
    name: String,
    content: String,
}

pub fn new() -> Post {
    Post {name: String::from(""), content: String::from("")}
}

impl Post {    
    pub fn is_valid(&self) -> bool {
        true    
    }
}

