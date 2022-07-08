use std::fmt;

#[derive(Debug)]
pub enum SafariError {
    Path,
    SqliteParse,
    BadSQL,
    NoHistory,
    Plist,
    Bookmark,
}

impl std::error::Error for SafariError {}

impl fmt::Display for SafariError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SafariError::Path => write!(f, "Failed to get user history file"),
            SafariError::NoHistory => write!(f, "No history data"),
            SafariError::BadSQL => write!(f, "Could not compose sqlite query"),
            SafariError::Plist => write!(f, "Could not parse PLIST file"),
            SafariError::Bookmark => write!(f, "Could not parse PLIST bookmark data"),
            SafariError::SqliteParse => {
                write!(f, "Failed to parse SQLITE History file")
            }
        }
    }
}
