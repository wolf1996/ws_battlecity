extern crate ws;

use self::ws::Error as WsError;
use std::fmt;
use std::error;

#[derive(Debug, Clone)]
pub struct FailedToStart{
    pub info :String,
}

impl fmt::Display for FailedToStart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Error while starting {}", self.info)
    }
}

impl error::Error for FailedToStart {
    fn description(&self) -> &str {
        &self.info[..]
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl From<WsError> for FailedToStart {
    fn from(errvar: WsError) -> Self {
        FailedToStart{info: errvar.to_string()}
    }
}

#[derive(Debug, Clone)]
pub struct MessageHandlerError{
    pub info :String,
}

impl fmt::Display for MessageHandlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Error while message handling {}", self.info)
    }
}

impl error::Error for MessageHandlerError {
    fn description(&self) -> &str {
        &self.info[..]
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
