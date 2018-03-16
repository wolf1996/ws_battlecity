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
use serde::Serializer;
use erased_serde::Serialize as ESerialize;
use std::fmt::Debug;
use serde::ser::Serialize;

const NUM_PLAYERS : usize = 1;

pub trait InfoObject: ESerialize + Send + Debug + InfoObjectClone{
}

trait InfoObjectClone {
    fn clone_box(&self) -> Box<InfoObject>;
}

impl<T> InfoObjectClone for T where T: 'static + InfoObject + Clone {
    fn clone_box(&self) -> Box<InfoObject> {
        Box::new(self.clone())
    }
}

impl Clone for Box<InfoObject> {
    fn clone(&self) -> Box<InfoObject> {
        self.clone_box()
    }
}

serialize_trait_object!(InfoObject);

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
#[derive(Debug, Serialize, Clone)]
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

#[derive(Debug, Serialize, Clone)]
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
    Error {
        err: String,
        user: String,
    },
    #[serde(serialize_with = "info_object_serializer")]
    GameInfo (Box<InfoObject>),
}

fn info_object_serializer<S>(to_serialize :&Box<InfoObject>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    to_serialize.serialize(serializer)
}


// TODO: Было бы норм добавить контекст и пихать все брокеры и map туда
pub trait GameObject {
    fn process (&mut self, brok: &mut events::Broker,  map: &mut GameField, msg :EventContainer) ->  LogicResult<EventsList>;
    fn tick (&mut self, brok: &mut events::Broker,   map: &mut GameField) ->  LogicResult<EventsList>;
    fn key(&self) -> usize;
    fn get_info(&self) -> LogicResult<EventsList>;
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
        let mut res = vec![EventContainer{unit: SYSTEM, evs: Events::UserConnected{user_name: user}}];
        let mut info = self.collect_info()?;
        res.append(&mut info);
        return Ok(res as EventsList);
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

    pub fn collect_info(&self) -> errors::LogicResult<EventsList> {
        RefCell::borrow(& self.logic.system).collect_info()
    }
}