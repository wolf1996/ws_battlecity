use std::error;
use std::fmt;

pub type LogicResult<T> = Result<T, GameLogicError>;

#[derive(Debug, Clone)]
pub struct GameLogicError {
    pub info: String,
}

impl fmt::Display for GameLogicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while command processing {}", self.info)
    }
}

impl error::Error for GameLogicError {
    fn description(&self) -> &str {
        &self.info[..]
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
