#[derive(Debug)]
pub enum ErrorType {
    Unknown,
    Io(std::io::Error),
    Json(serde_json::Error),
    Reqwest(reqwest::Error),
}

impl std::string::ToString for ErrorType {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("Unknown"),
            Self::Io(_) => String::from("Io"),
            Self::Json(_) => String::from("Json"),
            Self::Reqwest(_) => String::from("Reqwest"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub kind: String,
}

impl Error {
    pub fn new(child: ErrorType) -> Self {
        match child {
            ErrorType::Io(e) => Self {
                message: e.to_string(),
                kind: e.kind().to_string(),
            },
            ErrorType::Json(e) => Self {
                message: e.to_string(),
                kind: match e.classify() {
                    serde_json::error::Category::Data => String::from("Data"),
                    serde_json::error::Category::Eof => String::from("Eof"),
                    serde_json::error::Category::Io => String::from("Io"),
                    serde_json::error::Category::Syntax => String::from("Syntax"),
                },
            },
            ErrorType::Reqwest(e) => Self {
                message: e.to_string(),
                kind: if e.is_body() {
                    String::from("Body")
                } else if e.is_builder() {
                    String::from("Builder")
                } else if e.is_connect() {
                    String::from("Connect")
                } else if e.is_decode() {
                    String::from("Decode")
                } else if e.is_redirect() {
                    String::from("Redirect")
                } else if e.is_request() {
                    String::from("Request")
                } else if e.is_status() {
                    String::from("Status")
                } else if e.is_timeout() {
                    String::from("Timeout")
                } else {
                    String::from("Unknown")
                },
            },
            _ => Self {
                message: String::from("Unknown"),
                kind: ErrorType::Unknown.to_string(),
            },
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: {}", self.kind, self.message)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::new(ErrorType::Io(e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::new(ErrorType::Json(e))
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::new(ErrorType::Reqwest(e))
    }
}

#[cfg(test)]
mod tests {
    use serde::ser::Error;

    #[test]
    fn unknown() {
        let error = super::Error::new(super::ErrorType::Unknown);
        assert_eq!(error.to_string(), "[Unknown]: Unknown");
    }

    #[test]
    fn io() {
        let error = super::Error::new(super::ErrorType::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "test",
        )));
        assert_eq!(error.to_string(), "[other error]: test");
    }

    #[test]
    fn json() {
        let error = super::Error::new(super::ErrorType::Json(serde_json::Error::custom("test")));
        assert_eq!(error.to_string(), "[Data]: test");
    }

    #[cfg(feature = "sync")]
    #[test]
    fn reqwest() {
        let error = super::Error::new(super::ErrorType::Reqwest(
            reqwest::blocking::get("test").unwrap_err(),
        ));
        assert_eq!(error.to_string(), "[Builder]: builder error");
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn reqwest() {
        let error = super::Error::new(super::ErrorType::Reqwest(
            reqwest::get("test").await.unwrap_err(),
        ));
        assert_eq!(error.to_string(), "[Builder]: builder error");
    }
}
