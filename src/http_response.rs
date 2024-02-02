pub struct HttpResponse {
    pub status_line: String,
    pub headers: Vec<String>,
    pub body: String,
}

impl HttpResponse {
    pub fn serialize(&self) -> String {
        let headers_str = self.headers.join("\r\n");
        format!("{}\r\n{}\r\n{}", self.status_line, headers_str, self.body)
    }
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            status_line: "".to_string(),
            headers: vec![],
            body: "".to_string(),
        }
    }
}
