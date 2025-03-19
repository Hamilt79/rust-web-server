use std::collections::HashMap;

pub struct Header {
    data: String,
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
}

impl Header {
    pub fn new(data: Option<String>, headers: Option<HashMap<String, String>>, method: Option<String>, path: Option<String>) -> Header {
        let new_data = match data {
            Some(temp) => { temp },
            None => { "".to_owned() }
        };
        let new_headers = match headers {
            Some(temp)=>{ temp },
            None=>{ HashMap::new() }
        };
        let new_method = match method {
            Some(temp)=>{ temp },
            None=>{ "".to_owned() }
        };
        let new_path = match path {
            Some(temp)=>{ temp },
            None=>{ "".to_owned() }
        };
        Header {data: new_data, headers: new_headers, method: new_method, path: new_path}
    }

    pub fn fill_from_vec(&mut self, vec: &Vec<String>) {
        let mut first = true;
        for line in vec {
            if first {
                first = false;
                let mut line_split = line.split(" ");
                let method = line_split.next().unwrap_or("");
                let path = line_split.next().unwrap_or("").replace("%20", " ");
                self.path = path.to_owned();
                self.method = method.to_owned();
                continue;
            }
            let line_split = line.split_once(": ");        
            match line_split {
                Some(val)=>{ self.headers.insert(val.0.to_owned(), val.1.to_owned()); },
                None=>{ continue; }
            }
        }
    }

    pub fn get_header(&self, header_name: &str) -> Option<&String> {
        let value = self.headers.get(header_name); 
        Some(value?)
    }

    pub fn set_header(&mut self, header_name: String, header_value: String) {
        self.headers.insert(header_name, header_value);
    }
}

