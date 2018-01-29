use app::game::logic as game_logic;
use std::sync::mpsc::{Sender, Receiver};
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc::channel;
use std::time;

struct ChannelWrapper {
    log :game_logic::Logic,
}

struct LogicWorker {
    rec : Receiver<Receiver<game_logic::MessageContainer>>,
    games : Vec<(Receiver<game_logic::MessageContainer>, game_logic::Logic)>,
}

impl LogicWorker {
    fn worker(&mut self) {
        loop{
            for ll in self.rec.try_iter(){
                self.games.push((ll, game_logic::Logic{ obj_list: Vec::new()}));
                println!("New room \n \n \n ++++++++++++++++++++ \n \n \n ");
            };
            
            for &(ref i, ref j) in &self.games{
                for msg in i.try_iter(){
                    println!("\n \n PROCESSED {:?} \n \n ", msg);
                };
            };
            println!("\n \n TICK FINISHED \n \n ");
            thread::sleep(time::Duration::from_secs(1));
        };
    }
}

pub fn start() -> Sender<Receiver<game_logic::MessageContainer>>{
    let (sender, reciever) = channel();
    thread::spawn(move ||{
        let mut lw = LogicWorker{rec: reciever, games: Vec::new()};
        lw.worker();
    });
    return sender;
}