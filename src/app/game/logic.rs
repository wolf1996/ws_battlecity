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

const NUM_PLAYERS : usize = 1;

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

// Вговнокодим. Но надо бы добавить человеческий роутинг
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
    fn process (&mut self, brok: &mut events::Broker, msg :EventContainer) ->  LogicResult<EventContainer>;
    fn tick (&mut self, brok: &mut events::Broker) ->  LogicResult<EventContainer>;
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
    //TODO: State machin бы, но очень долго делать.
    pub fn process_message(&mut self, msg :MessageContainer) -> LogicResult<EventsList>{
        if !self.users.len() < NUM_PLAYERS {
            return Ok(vec![EventContainer{unit: 0,  evs: vec![Events::Error("not enouth players".to_owned())]}] as EventsList);
        }
        let evc = EventContainer{
            unit: SYSTEM,
            evs: vec![Events::Command(msg.clone())],
        };
        let evs = match self.logic.system.borrow_mut().pass_direct(msg.msg.unit ,evc){
            Ok(some) => some,
            Err(er) => return Err(er),
        };
        Ok(evs)
    }

    pub fn add_player(&mut self, user :String) -> LogicResult<EventsList>{
        if self.users.len() >= NUM_PLAYERS {
            return Ok(vec![EventContainer{unit: 0, evs: vec![Events::Error("lobbi is full".to_owned())]}] as EventsList);
        }
        let key = RefCell::borrow_mut(&mut self.logic.system).produce_key().clone();
        let mut us = User::new(key);
        let refu = Rc::new(RefCell::new(us));
        self.users.insert(user.clone(), refu.clone());
        RefCell::borrow_mut(&mut self.logic.system).add_system(refu.clone());
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