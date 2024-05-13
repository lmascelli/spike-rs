#[derive(Debug)]
pub enum ErrorType {
    FileOpen,
    PListCreate,
    PListCopy,
    Other,
}

#[derive(Debug)]
pub struct Error {
    etype: ErrorType,
    content: Option<String>,
}

impl Error {
    pub fn new(etype: ErrorType, content: Option<String>) -> Self {
        Self {
            etype,
            content,
        }
    }
}
