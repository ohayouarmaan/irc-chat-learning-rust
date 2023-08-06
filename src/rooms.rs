
#[derive(Debug)]
pub struct room {
    name: String
}

impl room {
    pub fn new(name: &str) -> Self {
        return Self {
            name: String::from(name)
        };
    }
}