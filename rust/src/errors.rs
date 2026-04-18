use std::fmt;

#[derive(Debug)]
pub enum FileOpsError {
    FileNotFound(String),
    OutOfBounds(usize, usize),
    ExternalChange { expected: String, actual: String },
    InvalidRegex(String),
    RateLimitExceeded,
    SchemaValidation(String),
    Encoding(String),
    InvalidPath(String),
    PermissionDenied(String),
    Other(String),
}

impl fmt::Display for FileOpsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileOpsError::FileNotFound(path) => write!(f, "File not found: {}", path),
            FileOpsError::OutOfBounds(line, total) => {
                write!(f, "Out of bounds: line {} in {}-line file", line, total)
            }
            FileOpsError::ExternalChange { expected, actual } => {
                write!(f, "External change detected: hash {} != {}", actual, expected)
            }
            FileOpsError::InvalidRegex(err) => write!(f, "Invalid regex: {}", err),
            FileOpsError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            FileOpsError::SchemaValidation(err) => write!(f, "Schema validation failed: {}", err),
            FileOpsError::Encoding(err) => write!(f, "Encoding error: {}", err),
            FileOpsError::InvalidPath(path) => write!(f, "Invalid path: {}", path),
            FileOpsError::PermissionDenied(path) => write!(f, "Permission denied: {}", path),
            FileOpsError::Other(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for FileOpsError {}

impl FileOpsError {
    pub fn to_json_rpc_error(&self) -> JsonRpcError {
        let (code, message) = match self {
            FileOpsError::FileNotFound(path) => (-32001, format!("File not found: {}", path)),
            FileOpsError::OutOfBounds(line, total) => {
                (-32002, format!("Out of bounds: line {} in {}-line file", line, total))
            }
            FileOpsError::ExternalChange { expected, actual } => (
                -32003,
                format!("External change detected: hash {} != {}", actual, expected),
            ),
            FileOpsError::InvalidRegex(err) => (-32004, format!("Invalid regex: {}", err)),
            FileOpsError::RateLimitExceeded => (-32005, "Rate limit exceeded".to_string()),
            FileOpsError::SchemaValidation(err) => (-32006, format!("Schema validation failed: {}", err)),
            FileOpsError::Encoding(err) => (-32009, format!("Encoding error: {}", err)),
            FileOpsError::InvalidPath(path) => (-32010, format!("Invalid path: {}", path)),
            FileOpsError::PermissionDenied(path) => (-32011, format!("Permission denied: {}", path)),
            FileOpsError::Other(err) => (-32999, err.clone()),
        };

        JsonRpcError {
            code,
            message,
            data: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

