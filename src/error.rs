pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidPath,
    InvalidParent,
    CommandNotRecognized(&'static str),
    NotFound(&'static str),
    InvalidName(&'static str),
    FolderNotEmpty(String),
    ArgsNotRecognized(String),
}
