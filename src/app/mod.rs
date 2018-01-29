mod handler;
mod message_manager;
mod room_manager;
mod logic;
pub mod errors;
pub mod game;


use std::sync::mpsc::channel;
use std::result::Result;

pub fn start(addres: String) -> Result<(),errors::FailedToStart> {
    let tx = logic::start();
    let sender = message_manager::start(tx);
    try!(handler::start(addres, sender));
    Ok(())
}