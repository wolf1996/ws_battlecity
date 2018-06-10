extern crate ws;

use self::ws::Sender as WsSender;
use app::game::logic::EventContainer;
use serde_json;
use std::boxed::Box;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

fn worker(rec: Receiver<(Box<EventContainer>, WsSender)>) {
    for (i, j) in &mut rec.iter() {
        let msg = *i;
        println!("processing responce {:?}", msg);
        let msg_str = serde_json::to_string(&msg).unwrap();
        j.send(msg_str).expect("error sending responce");
    }
}

pub fn start() -> Sender<(Box<EventContainer>, WsSender)> {
    let (lt, rt) = channel();
    thread::spawn(move || {
        worker(rt);
    });
    return lt;
}
