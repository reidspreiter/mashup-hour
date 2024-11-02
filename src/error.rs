use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    Custom(String),
    CriticalError(String),

    #[from]
    ResponseError {
        status_code: u16,
        message: String,
    },

    #[from]
    IndexError {
        index: usize,
        length: usize,
    },

    #[from]
    ImageError(image::error::ImageError),

    #[from]
    ReqwestError(reqwest::Error),

    #[from]
    SerdeJSONError(serde_json::Error),

    #[from]
    IOError(std::io::Error),
}

impl Error {
    pub fn custom(val: impl std::fmt::Display) -> Self {
        Self::Custom(val.to_string())
    }
}

impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::Custom(val.to_string())
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for Error {}
