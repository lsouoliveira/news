use crate::providers::core::response::Response;

pub enum Error {
    Request(Response),
    Client(String)
}

impl Error {
    pub fn build_request_error(status: u16, body: String) -> Error {
        Error::Request(Response::new(status, body))
    }

    pub fn build_client_error(message: &str) -> Error {
        Error::Client(message.to_string())
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Request(response) => write!(f, "Request error: {:?}", response),
            Error::Client(message) => write!(f, "Client error: {}", message),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_build_request_error() {
        let error = super::Error::build_request_error(404, "Not found".to_string());
        match error {
            super::Error::Request(response) => {
                assert_eq!(response.status(), 404);
                assert_eq!(response.body(), "Not found");
            },
            _ => panic!("Expected Request error")
        } 
    }

    #[test]
    fn test_build_client_error() {
        let error = super::Error::build_client_error("Failed to send request");
        match error {
            super::Error::Client(message) => {
                assert_eq!(message, "Failed to send request");
            },
            _ => panic!("Expected Client error")
        }
    }
}
