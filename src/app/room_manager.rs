use std::sync::RwLock;
use std::collections::HashMap;
use app::game;
use std::vec::Vec;
use app::errors::MessageHandlerError;
use std::collections::hash_map::Entry;
use app::game::logic;

#[derive(Debug)]
pub struct Room {
    pub messages: Vec<logic::MessageContainer>,
}

#[derive(Debug)]
pub struct RoomsManager {
    pub rooms: RwLock<HashMap<String, Room>>,
}

impl RoomsManager {
    pub fn pass_mesage(&mut self, msg: logic::MessageContainer) ->  Result<(), MessageHandlerError>{
        let mut mlock = & mut self.rooms;
        let mut rooms = mlock.write().unwrap();
        let room = match rooms.entry(msg.meta.user_name.clone()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Room{messages: Vec::new()}),
        };
        room.messages.push(msg);
        Ok(())
    }
}