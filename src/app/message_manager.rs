extern crate ws;

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

#[derive(Debug)]
pub struct MessageMeta {
    pub name: String,
    pub room: String,
}

#[derive(Debug)]
pub struct MessageContainer {
    pub meta: MessageMeta, 
    pub message: String,
}

fn message_manager_worker(rec: mpsc::Receiver<MessageContainer>, rms_arc: &mut Arc<RoomsManager>) {
    let mut rms = rms_arc.as_ref();
    for i in rec {
        println!("{:?}", i);
        // TODO: добравить обработку ошибки
        let mut msg : logic::Message = serde_json::from_str(&i.message[..]).unwrap();
        match rms.pass_mesage(logic::MessageContainer{meta: logic::Meta{user_name: i.meta.name}, msg: msg}){
            Ok(some) => some ,
            Err(err) => println!("{:?}",err),
        }
    };
}

fn message_manager_worker_resp(rec: mpsc::Receiver<logic::ResponceContainer>) {
    for i in rec {
        println!("{:?}", i);
    };
}

pub fn start() -> mpsc::Sender<MessageContainer>{
    let (sender, reciever) = channel();
    let rms :RwLock<HashMap<String, Room>> = RwLock::new(HashMap::new());
    let mut rmgr = Arc::new(RoomsManager{rooms: rms});
    thread::spawn(move ||{message_manager_worker(reciever, &mut rmgr);});
    return sender;
}