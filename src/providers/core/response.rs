#[derive(Debug)]
pub struct Response {
    status: u16,
    body: String,
}

impl Response {
    pub fn new(status: u16, body: String) -> Response {
        Response { status, body }
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status() {
        let response = Response::new(200, "".to_string());
        assert_eq!(response.status(), 200);
    }

    #[test]
    fn test_body() {
        let response = Response::new(200, "Hello, world!".to_string());
        assert_eq!(response.body(), "Hello, world!");
    }
}
