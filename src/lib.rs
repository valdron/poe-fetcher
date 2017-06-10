
extern crate hyper;

use hyper::{Client, Url};

#[derive(Debug)]
struct PoeFetcher {
    url: Url,
    client: Client,
}


impl PoeFetcher {
    pub fn new(start_url: Url) -> Self {
        Self {
            url: start_url,
            client: Client::new(),
        }
    }
}

impl Iterator for PoeFetcher {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!();
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
