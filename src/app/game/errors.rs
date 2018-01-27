use std::fmt;
use std::error;

pub type LogicResult<T> = Result<T, GameLogicMesssage>;

#[derive(Debug, Clone)]
pub struct GameLogicMesssage{
    pub info :String,
}

impl fmt::Display for GameLogicMesssage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Error while command processing {}", self.info)
    }
}

impl error::Error for GameLogicMesssage {
    fn description(&self) -> &str {
        &self.info[..]
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
