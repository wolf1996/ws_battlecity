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
use app::game::map::GameField;

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
    Move{
        direction :Direction,
    },
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
        dir :Direction,
    },
    ChangeDirection {
        dir :Direction,
    },
    Fire {
        pos :Position,
        dir :Direction,
    },
    Spawned(Unit),
    UserConnected{
        user_name: String,
    },
    Collision {
        fst :usize,
        scd :usize,
    },
    Error {err: String, user: String},
}


// TODO: Было бы норм добавить контекст и пихать все брокеры и map туда
pub trait GameObject {
    fn process (&mut self, brok: &mut events::Broker,  map: &mut GameField, msg :EventContainer) ->  LogicResult<EventsList>;
    fn tick (&mut self, brok: &mut events::Broker,   map: &mut GameField) ->  LogicResult<EventsList>;
    fn key(&self) -> usize;
}

pub struct Game {
    pub users : HashMap<String, Rc<RefCell<User>>>,
    pub logic : Logic,
}

pub struct Logic {
    pub system : Rc<RefCell<events::Broker>>,
    pub map    : GameField,
}

impl Game {
    //TODO: State machin бы, но очень долго делать.
    pub fn process_message(&mut self, msg :MessageContainer) -> LogicResult<EventsList>{
        if !self.users.len() < NUM_PLAYERS {
            return Ok(vec![EventContainer{unit: 0,  evs: Events::Error{err: "not enouth players".to_owned(), user: msg.meta.user_name.clone()}}] as EventsList);
        }
        let evc = EventContainer{
            unit: SYSTEM,
            evs: Events::Command(msg.clone()),
        };
        let evs = match self.logic.system.borrow_mut().pass_direct(msg.msg.unit ,evc, &mut self.logic.map){
            Ok(some) => some,
            Err(er) => return Err(er),
        };
        Ok(evs)
    }

    pub fn add_player(&mut self, user :String) -> LogicResult<EventsList>{
        if self.users.len() >= NUM_PLAYERS {
            return Ok(vec![EventContainer{unit: 0, evs: Events::Error{err:"lobbi is full".to_owned(), user:user}}] as EventsList);
        }
        let key = RefCell::borrow_mut(&mut self.logic.system).produce_key().clone();
        let mut us = User::new(key);
        let refu = Rc::new(RefCell::new(us));
        self.users.insert(user.clone(), refu.clone());
        RefCell::borrow_mut(&mut self.logic.system).add_system(refu.clone());
        return Ok(vec![EventContainer{unit: SYSTEM, evs: Events::UserConnected{user_name: user}}] as EventsList);
    }

    pub fn tick(&mut self) ->  LogicResult<EventsList>{
        let mut system =  RefCell::borrow_mut(&mut self.logic.system);
        let evs = match system.tick(&mut self.logic.map){
            Ok(some) => some,
            Err(er) => return Err(er),
        };
        Ok(evs)
    }

    pub fn new() -> Game{
        let mut map = GameField::new();
        let mut brok = Rc::new(RefCell::new(events::Broker::new()));
        {
            let mut bt = RefCell::borrow_mut(&mut brok);
            map.generate_map(&mut bt);
        }
        Game{logic: Logic{system: brok, map: map}, users: HashMap::new()}
    }
}