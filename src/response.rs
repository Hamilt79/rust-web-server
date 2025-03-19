pub struct Response {
    http_header: String,
    status_code: u16,
    status_code_message: String,
    content_length: usize,
    content_type: String,
    data: String,
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ret_str = format!("{} {} {}\n\rcontent-length: {}\n\rcontent-type: {}\n\r\n\r{}",
            self.http_header,
            self.status_code,
            self.status_code_message,
            self.content_length,
            self.content_type,
            self.data);
        write!(f, "{ret_str}")
    }
}

impl Response {
    pub fn new(data: String, status_code: u16, status_code_message: Option<String>) -> Response {
        let data_len = data.as_bytes().len();
        let status_code_str = match status_code_message {
                Some(val)=> { val },
                None=> {
                    match status_code {
                        200=>"OK".to_owned(),
                        301=>"IDK".to_owned(),
                        _=>"Unknown".to_owned(),
                    }
                }};
        Response { http_header: "HTTP/1.1".to_owned(), status_code, status_code_message: status_code_str, content_length: data_len, content_type: "text/plain; charset=utf-8".to_owned(), data }
    }
}
