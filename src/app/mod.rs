mod handler;
mod message_manager;
mod room_manager;
pub mod errors;
pub mod game;

use std::result::Result;

pub fn start(addres: String) -> Result<(),errors::FailedToStart> {
    let sender = message_manager::start();
    try!(handler::start(addres, sender));
    Ok(())
}