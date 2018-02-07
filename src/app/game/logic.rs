use app::game::tank;
use app::game::errors;
use std::boxed::Box;
use std::collections::HashMap;
use app::game::user::User;
use app::game::errors::{GameLogicError, LogicResult};
use app::game::user::Role;
use app::game::events;
use std::rc::Rc;

const MAX_PLAYERS : usize = 1;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub user_name : String,
}

#[derive(Debug, Serialize, Deserialize, Clone)] 
pub struct MessageContainer {
    pub msg :Message,
    pub meta :Meta,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub unit: usize,
    pub cmd: Commands,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponceContainer {
    pub meta :Meta,
    pub resp :Vec<Responce>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Responce {
    pub unit: usize,
    pub evs : Events,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub x : f32,
    pub y : f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Commands {
    Move,
    ChangeDirection{
        newdir :Direction,
    },
    Fire, 
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Unit {
    Tank(tank::Tank),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Events {
    Command(MessageContainer),
    ChangePosition {
        pos :Position,
    },
    Fire {
        pos :Position,
        dir :Direction,
    },
    Spawned(Unit),
}

pub trait GameObject {
    fn process (&mut self, msg :Events) ->  LogicResult<Events>;
    fn tick (&mut self) ->  LogicResult<Events>;
    fn key(&self) -> String;
}

pub struct Game {
    pub users : HashMap<String, User>,
    pub logic : Logic,
}

pub struct Logic {
    pub system : Rc<events::Broker>,
}

impl Game {
    pub fn process_message(&mut self, msg :MessageContainer) -> LogicResult<ResponceContainer>{
        let mut system =  Rc::get_mut(&mut self.logic.system).unwrap();
        let evs = match system.pass_message("1".to_owned(),Events::Command(msg.clone())){
            Ok(some) => some,
            Err(er) => return Err(er),
        };
        let events = evs.into_iter().map(|i|{
            let (_, ev) = i;
            //TODO: разобраться с идентификаторами
            Responce{unit: 1, evs:ev } 
        }).collect();
        Ok(ResponceContainer{meta: msg.meta, resp: events})
    }

    pub fn add_player(&mut self, user :String) -> LogicResult<()>{
        if self.users.len() >= MAX_PLAYERS {
            return Err(GameLogicError{info: "lobby is full".to_string()});
        }
        let key =  Rc::get_mut(&mut self.logic.system).unwrap().produceKey().clone();
        User::new(key,self.logic.system.clone());
        return Ok(());
    }

    pub fn remove_player(&mut self, user :String) -> LogicResult<()>{
        if self.users.len() == 0 {
            return Err(GameLogicError{info: "lobby is full".to_string()});
        }
        self.users.remove(&user);
        Ok(())
    }

    pub fn tick(&mut self) ->  Vec<LogicResult<Responce>>{
        let mut vc: Vec<LogicResult<Responce>> = Vec::new();
        // for (ref ind, ref mut i) in &mut self.logic.obj_list.iter_mut().enumerate() {
        //     match i.tick() {
        //         Ok(some) => vc.push(Ok(Responce{unit: *ind, evs: vec![some, ]})),
        //         Err(err) => vc.push(Err(err)),
        //     };
            
        // }
        return vc;
    }

    pub fn new() -> Game{
        Game{logic: Logic{system: Rc::new(events::Broker::new())}, users: HashMap::new()}
    }
}