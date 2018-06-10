pub mod errors;
pub mod game;
mod handler;
mod logic;
mod message_manager;
mod responce_manager;
mod room_manager;

use std::result::Result;

pub fn start(addres: String) -> Result<(), errors::FailedToStart> {
    let resp_ch = responce_manager::start();
    let tx = logic::start(resp_ch);
    let sender = message_manager::start(tx);
    try!(handler::start(addres, sender));
    Ok(())
}
