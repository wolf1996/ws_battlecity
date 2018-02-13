use app::game::tank;
use app::game::errors;
use std::boxed::Box;
use std::collections::HashMap;
use app::game::user::User;
use app::game::errors::{GameLogicError, LogicResult};
use app::game::events;
use std::cell::RefCell;
use std::rc::Rc;
use app::game::events::SYSTEM;

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

pub type  EventsList = Vec<EventContainer>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventContainer {
    pub unit: usize,
    pub evs : Vec<Events>,
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
    UserConnected{
        user_name: String,
    },
    Error (String),
}

pub trait GameObject {
    fn process (&mut self, msg :EventContainer) ->  LogicResult<EventContainer>;
    fn tick (&mut self) ->  LogicResult<EventContainer>;
    fn key(&self) -> usize;
}

pub struct Game {
    pub users : HashMap<String, Rc<RefCell<User>>>,
    pub logic : Logic,
}

pub struct Logic {
    pub system : Rc<RefCell<events::Broker>>,
}

impl Game {
    pub fn process_message(&mut self, msg :MessageContainer) -> LogicResult<EventsList>{
        let mut system =  RefCell::borrow_mut(&mut self.logic.system);
        let evc = EventContainer{
            unit: SYSTEM,
            evs: vec![Events::Command(msg.clone())],
        };
        let evs = match system.pass_direct(msg.msg.unit ,evc){
            Ok(some) => some,
            Err(er) => return Err(er),
        };
        Ok(evs)
    }

    pub fn add_player(&mut self, user :String) -> LogicResult<EventsList>{
        if self.users.len() >= MAX_PLAYERS {
            return Ok(vec![EventContainer{unit: 0, evs: vec![Events::Error("lobbi is full".to_owned())]}] as EventsList);
        }
        let key = RefCell::borrow_mut(&mut self.logic.system).produceKey().clone();
        let mut us = User::new(key,Rc::clone(&mut self.logic.system));
        us.spawn_tank();
        self.users.insert(user.clone(), Rc::new(RefCell::new(us)));
        return Ok(vec![EventContainer{unit: SYSTEM, evs: vec![Events::UserConnected{user_name: user}]}] as EventsList);
    }

    pub fn tick(&mut self) ->  LogicResult<EventsList>{
        let mut system =  RefCell::borrow_mut(&mut self.logic.system);
        let evs = match system.tick(){
            Ok(some) => some,
            Err(er) => return Err(er),
        };
        Ok(evs)
    }

    pub fn new() -> Game{
        Game{logic: Logic{system: Rc::new(RefCell::new(events::Broker::new()))}, users: HashMap::new()}
    }
}