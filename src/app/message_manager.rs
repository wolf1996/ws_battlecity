extern crate ws;

use self::ws::Sender as WsSender;
use serde_json;
use std::sync::mpsc::channel;
use std::thread;
use std::sync::mpsc;
use app::game;
use app::room_manager::RoomsManager;
use std::sync::RwLock;
use std::collections::HashMap;
use app::room_manager::Room;
use app::game::logic;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{Sender, Receiver};
use app::logic as inf_logic;

pub use app::logic::MessageMeta;

pub struct MessageContainer {
    pub meta: MessageMeta, 
    pub message: String,
}

fn message_manager_worker(rec: mpsc::Receiver<MessageContainer>, rms_arc: &mut Arc<RoomsManager>) {
    let mut rms = rms_arc.as_ref();
    for i in rec {
        // TODO: добравить обработку ошибки
        let mut msg : logic::Message = serde_json::from_str(&i.message[..]).unwrap();
        match rms.pass_mesage(inf_logic::MessageContainer{meta: i.meta, message: msg}){
            Ok(some) => some ,
            Err(err) => println!("{:?}",err),
        }
    };
}

pub fn start(out :Sender<Receiver<inf_logic::MessageContainer>>) -> mpsc::Sender<MessageContainer>{
    let (sender, reciever) = channel();
    let mut rmgr = Arc::new( RoomsManager{rooms: RwLock::new(HashMap::new()), out: Mutex::new(out.clone())});
    thread::spawn(move ||{message_manager_worker(reciever, &mut rmgr);});
    return sender;
}