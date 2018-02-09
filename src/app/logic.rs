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
}

pub enum Content {
    Message(game_logic::Message),
    Close,
    Start(WsSender),
}

pub struct MessageContainer {
    pub meta: MessageMeta, 
    pub message: Content,
}

struct GameContainer{
    pub channel :Receiver<MessageContainer>,
    pub game : game_logic::Game,
    pub users : Vec<String>,
}

struct LogicWorker {
    rec : Receiver<Receiver<MessageContainer>>,
    games : Vec<GameContainer>,
    resp : Sender<(Box<Responce>, WsSender)>,
    clients: HashMap<String, WsSender>,
}

impl LogicWorker {
    fn worker(&mut self) {
        loop{
            for ll in self.rec.try_iter(){
                let cont = GameContainer{
                    channel :ll,
                    game    :game_logic::Game::new(),
                    users   :Vec::new(),
                };
                self.games.push(cont);
                println!("\n\n\n +++++++++++++++++ New room  ++++++++++++++++++++ \n \n \n ");
            };
            
            for ref mut game in &mut self.games{
                for msg in game.channel.try_iter(){
                    match msg.message{
                        Content::Message(mg) => {
                            let wssender = self.clients.get(&msg.meta.name).unwrap();
                            let mcnt = game_logic::MessageContainer{msg: mg, meta : game_logic::Meta{user_name:msg.meta.name.clone()}};
                            match game.game.process_message(mcnt){
                                Ok(some) =>  {
                                    for i in some.resp{
                                        self.resp.send((Box::new(i), wssender.clone())).unwrap();
                                    };
                                },
                                Err(some) => println!(" Some error in logic process {:?}", some),
                            };
                        }
                        Content::Close => {
                            println!("\n\n\n +++++++++++++++++ close  ++++++++++++++++++++ \n \n \n ");
                            game.game.remove_player(msg.meta.name.clone());
                            self.clients.remove(&msg.meta.name);
                        }  
                        Content::Start(wssock) => {
                            println!("\n\n\n +++++++++++++++++ client  ++++++++++++++++++++ \n \n \n ");
                            game.game.add_player(msg.meta.name.clone());
                            self.clients.insert(msg.meta.name, wssock);
                            
                        }
                    };
                };
            };
            println!("\n \n TICK FINISHED \n \n ");
            thread::sleep(time::Duration::from_secs(10));
        };
    }
}

pub fn start(resp : Sender<(Box<Responce>, WsSender)>) -> Sender<Receiver<logic::MessageContainer>>{
    let (sender, reciever) = channel();
    thread::spawn(move ||{
        let mut lw = LogicWorker{rec: reciever, games: Vec::new(), resp: resp, clients: HashMap::new()};
        lw.worker();
    });
    return sender;
}