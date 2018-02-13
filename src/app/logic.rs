extern crate ws;

use self::ws::Sender as WsSender;
use app::game::logic as game_logic;
use app::game::logic::{EventContainer, Game};
use std::sync::mpsc::{Sender, Receiver};
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc::channel;
use std::time;
use app::logic;
use std::rc::Rc;

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
    pub channel : Receiver<MessageContainer>,
    pub resp    : Rc<Sender<(Box<EventContainer>, WsSender)>>,
    pub game    : Game,
    pub users   : HashMap<String, WsSender>,
}

impl GameContainer {
    pub fn broadcast(&self, mut rsp :Box<EventContainer>) {
        for (_, j) in &self.users{
            self.resp.send((Box::clone(&mut rsp), j.clone())).unwrap();
        };
    }
}

struct LogicWorker {
    rec : Receiver<Receiver<MessageContainer>>,
    games : Vec<GameContainer>,
    resp : Rc<Sender<(Box<EventContainer>, WsSender)>>,
}

impl LogicWorker {
    fn worker(&mut self) {
        loop{
            for ll in self.rec.try_iter(){
                let cont = GameContainer{
                    channel :ll,
                    resp    :self.resp.clone(),
                    game    :game_logic::Game::new(),
                    users   :HashMap::new(),
                };
                self.games.push(cont);
                println!("\n\n\n +++++++++++++++++ New room  ++++++++++++++++++++ \n \n \n ");
            };
            
            for ref mut game in &mut self.games{
                for mut msg in &mut game.channel.try_iter(){
                    match msg.message{
                        Content::Message(mg) => {
                            let mcnt = game_logic::MessageContainer{msg: mg, meta : game_logic::Meta{user_name:msg.meta.name.clone()}};
                            match game.game.process_message(mcnt){
                                Ok(some) =>  {
                                    for i in some{
                                        game.broadcast(Box::new(i));
                                    };
                                },
                                Err(some) => println!(" Some error in logic process {:?}", some),
                            };
                        }
                        Content::Close => {
                            println!("\n\n\n +++++++++++++++++ close  ++++++++++++++++++++ \n \n \n ");
                            game.users.remove(&msg.meta.name);
                        }  
                        Content::Start(wssock) => {
                            println!("\n\n\n +++++++++++++++++ client  ++++++++++++++++++++ \n \n \n ");
                            game.users.insert(msg.meta.name.clone(), wssock);
                            game.game.add_player(msg.meta.name.clone());
                        }
                    };
                };
            };
            for ref mut game in &mut self.games{
                match game.game.tick(){
                    Ok(some) =>  {
                        for i in some{
                            game.broadcast(Box::new(i));
                        };
                    },
                    Err(some) => println!(" Some error in logic tick {:?}", some),
                };
            };
            println!("\n \n TICK FINISHED \n \n ");
            thread::sleep(time::Duration::from_secs(20));
        };
    }
}

pub fn start(resp : Sender<(Box<EventContainer>, WsSender)>) -> Sender<Receiver<logic::MessageContainer>>{
    let (sender, reciever) = channel();
    thread::spawn(move ||{
        let mut lw = LogicWorker{rec: reciever, games: Vec::new(), resp: Rc::new(resp)};
        lw.worker();
    });
    return sender;
}