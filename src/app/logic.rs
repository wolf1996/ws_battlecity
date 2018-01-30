extern crate ws;

use self::ws::Sender as WsSender;
use app::game::logic as game_logic;
use app::game::logic::Responce;
use std::sync::mpsc::{Sender, Receiver};
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc::channel;
use std::time;
use app::logic;

struct ChannelWrapper {
    log :game_logic::Logic,
}

pub struct MessageMeta {
    pub name: String,
    pub room: String,
    pub chan: WsSender,
}

pub struct MessageContainer {
    pub meta: MessageMeta, 
    pub message: game_logic::Message,
}

struct LogicWorker {
    rec : Receiver<Receiver<MessageContainer>>,
    games : Vec<(Receiver<MessageContainer>, game_logic::Logic)>,
    resp : Sender<(Box<Responce>, WsSender)>,
}

impl LogicWorker {
    fn worker(&mut self) {
        loop{
            for ll in self.rec.try_iter(){
                self.games.push((ll, game_logic::Logic::new()));
                println!("\n\n\n New room \n \n \n ++++++++++++++++++++ \n \n \n ");
            };
            
            for &mut(ref i, ref mut j) in &mut self.games{
                for msg in i.try_iter(){
                    let mcnt = game_logic::MessageContainer{msg: msg.message, meta : game_logic::Meta{user_name:msg.meta.name.clone()}};
                    match j.process_message(mcnt){
                        Ok(some) =>  self.resp.send((Box::new(some.resp), msg.meta.chan)).unwrap(),
                        Err(some) => println!(" Some error in logic process {:?}", some),
                    };
                };
            };
            println!("\n \n TICK FINISHED \n \n ");
            thread::sleep(time::Duration::from_secs(1));
        };
    }
}

pub fn start(resp : Sender<(Box<Responce>, WsSender)>) -> Sender<Receiver<logic::MessageContainer>>{
    let (sender, reciever) = channel();
    thread::spawn(move ||{
        let mut lw = LogicWorker{rec: reciever, games: Vec::new(), resp: resp};
        lw.worker();
    });
    return sender;
}