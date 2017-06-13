
extern crate hyper;
extern crate regex;

use hyper::{Client, Url};
use hyper::method::Method;
use regex::bytes::Regex;
use std::io::Read;

#[derive(Debug)]
pub struct PoeSite {
    pub change_id: String,
    pub body: Vec<u8>,
}

#[derive(Debug)]
pub struct PoeFetcher {
    url: Url,
    client: Client,
    next_id: String,
}


impl PoeFetcher {
    pub fn new(start_url: Url) -> Self {
        Self {
            url: start_url,
            client: Client::new(),
            next_id: "".into(),
        }
    }
}

impl Iterator for PoeFetcher {
    type Item = PoeSite;
    fn next(&mut self) -> Option<Self::Item> {

        self.url
            .query_pairs_mut()
            .clear()
            .append_pair("id", &self.next_id);

        let response = self.client.request(Method::Get, self.url.clone()).send();



        match response {
            Ok(mut res) => {

                let mut result = Vec::with_capacity(5_000_000);
                let old_id = self.next_id.clone();

                let size = res.read_to_end(&mut result);

                match size {
                    Ok(x) if x == 0 => return None,
                    Err(_) => return None,
                    _ => {}
                }

                self.next_id = match extract_next_id(&result) {
                    Ok( s ) => s,
                    _ => return None
                };

                Some(PoeSite {
                         change_id: old_id,
                         body: result,
                     })
            }

            _ => None,
        }
    }
}

fn extract_next_id(s: &[u8]) -> Result<String, String> {
    let re = Regex::new("\\{\"next_change_id\":\"((?:\\d|-)+)\",").unwrap();
    let mat = match re.captures(s).and_then(|cap| cap.get(1)) {
        Some(x) => x,
        None => return Err("no changeid found in body".into()),
    };
    Ok(String::from_utf8(mat.as_bytes().to_vec()).map_err(|_| "error not utf8 string".to_string())?)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
