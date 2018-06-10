use app::errors::MessageHandlerError;
use app::logic as inf_logic;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use std::sync::RwLock;

pub struct Room {
    pub messages: Mutex<Sender<inf_logic::MessageContainer>>,
}

pub struct RoomsManager {
    pub rooms: RwLock<HashMap<String, Room>>,
    pub out: Mutex<Sender<Receiver<inf_logic::MessageContainer>>>,
}

impl RoomsManager {
    pub fn pass_mesage(&self, msg: inf_logic::MessageContainer) -> Result<(), MessageHandlerError> {
        let mut rooms = self.rooms.write().unwrap();
        let room = match rooms.entry(msg.meta.room.clone()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(self.produce_room()),
        };
        room.messages.lock().unwrap().send(msg)?;
        Ok(())
    }

    pub fn add_player(&self, msg: inf_logic::MessageContainer) -> Result<(), MessageHandlerError> {
        let mut rooms = self.rooms.write().unwrap();
        let room = match rooms.entry(msg.meta.room.clone()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(self.produce_room()),
        };
        room.messages.lock().unwrap().send(msg)?;
        Ok(())
    }

    pub fn remove_player(
        &self,
        msg: inf_logic::MessageContainer,
    ) -> Result<(), MessageHandlerError> {
        let mut rooms = self.rooms.write().unwrap();
        let room = match rooms.entry(msg.meta.room.clone()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(self.produce_room()),
        };
        room.messages.lock().unwrap().send(msg)?;
        Ok(())
    }

    fn produce_room(&self) -> Room {
        let (tx, rc) = channel();
        self.out.lock().unwrap().send(rc).expect("produsing room sensor error");
        Room {
            messages: Mutex::new(tx.clone()),
        }
    }
}
