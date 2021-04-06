use std::fmt;

#[derive(Debug, Clone)]
pub struct ExecutionError;

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to execute module command")
    }
}
