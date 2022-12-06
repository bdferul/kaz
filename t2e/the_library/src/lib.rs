use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserData {
    id: usize,
    name: String,
    country: String,
}

impl UserData {
    pub fn new(id: usize, name: &str, country: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}