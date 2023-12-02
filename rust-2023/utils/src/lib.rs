pub type ProcessResult = Result<String, ProcessError>;

#[derive(Debug, Clone)]
pub enum ProcessError {
    ParsingError
}
