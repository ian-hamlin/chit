extern crate reqwest;
use std::io::Read;

pub mod crates;
pub mod github;

pub fn get(url: String) -> Option<serde_json::value::Value> {
    let req = reqwest::get(&url);

    match req {
        Ok(mut res) => {
            if res.status() == 200 {
                let mut res_body = String::new();
                res.read_to_string(&mut res_body)
                    .expect("fail to read crate res_body");

                Some(serde_json::from_str(&res_body).expect("fail to serde parse res_body"))
            } else {
                None
            }
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
}
