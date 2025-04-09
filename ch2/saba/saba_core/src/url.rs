use alloc::string::{String, ToString};

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self {
            url: "".to_string(),
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }
    pub fn parse(&mut self) -> Result<Self, String> {
        unimplemented!()
    }
}
