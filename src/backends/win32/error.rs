#[derive(Debug)]
pub enum Win32Error {
    String(String),
}

impl From<&str> for Win32Error {
    fn from(error: &str) -> Self {
        Win32Error::String(error.to_owned())
    }
}

pub type Win32Result<T = ()> = Result<T, Win32Error>;
