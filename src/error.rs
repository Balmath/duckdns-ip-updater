#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    ConfigFolderAccess,
    DisplayVersion,
    DisplayHelp,
    InvalidArgument,
    InvalidConfigFile,
    InvalidRequest,
}

#[derive(Debug, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}
