pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Misc(String),
    #[error("Parser error: {0}")]
    Parser(#[from] ikal::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] attohttpc::Error),
    #[error("{method} {href}: {status}")]
    Request {
        method: String,
        href: String,
        status: attohttpc::StatusCode,
        body: String,
    },
}

impl Error {
    pub fn new(method: crate::Method, href: &str, response: attohttpc::Response) -> Self {
        Self::Request {
            method: method.to_string(),
            href: href.to_string(),
            status: response.status(),
            body: response.text().unwrap_or_default(),
        }
    }
}
