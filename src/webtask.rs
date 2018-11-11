use curl::easy::Easy;
use std::io::Read;
use std::str;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Webtask {
    pub url: String,
    pub name: String,
    pub description: String,
    pub repository: Option<String>,
    pub private: bool,
    pub author: String,
}

impl Webtask {
    pub fn call(&self, data: String) -> String {
        let content = Vec::new();
        let mut easy = Easy::new();
        let mut bytes = data.as_bytes();
        let out = Arc::new(Mutex::new(content));

        easy.url(self.url.as_ref()).unwrap();
        easy.post(true).unwrap();
        easy.post_field_size(bytes.len() as u64).unwrap();

        let mut transfer = easy.transfer();
        transfer
            .read_function(|buf| Ok(bytes.read(buf).unwrap_or(0)))
            .unwrap();

        transfer
            .write_function(|data| {
                let mut c = out.lock().unwrap();

                c.extend_from_slice(data);
                // let s = str::from_utf8(data).unwrap();
                // println!("{}", String::from(s).replace("\\n", "\n").replace("\"", ""));
                // stdout().write_all(data).unwrap();
                Ok(data.len())
            })
            .unwrap();

        transfer.perform().unwrap();

        // println!("{:?}", str::from_utf8(&out.lock().unwrap()).unwrap());
        let str_val = out.lock().unwrap();
        let response = str::from_utf8(&str_val).unwrap();

        response.to_owned()
    }
}
