extern crate ws;

use serde_json;
use std::boxed::Box;
use self::ws::Sender as WsSender;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use app::game::logic::Responce; 

fn worker(rec: Receiver<(Box<Responce>, WsSender)>){
    for ( i, j) in &mut rec.iter() {
        let msg = *i;
        let msg_str = serde_json::to_string(&msg).unwrap();
        j.send(msg_str);
    }
}

pub fn start() ->  Sender<(Box<Responce>, WsSender)>{
    let (lt, rt) = channel();
    thread::spawn(move ||{worker(rt);});
    return lt
}