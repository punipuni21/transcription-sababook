extern crate alloc;

use alloc::string::String;
use noli::net::lookup_host;
use saba_core::error::Error;
use saba_core::http::HttpResponse;

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
      let ips = match lookup_host(&"example.com") {
        Ok(ips) => ips,
        Err(_) => return Err(Error::Network("Failed to find IP address").to_string());
      }
    }
}
