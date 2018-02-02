use app::game::tank;
use app::game::errors;
use std::boxed::Box;
use std::collections::HashMap;
use app::game::user::User;
use app::game::errors::{GameLogicError, LogicResult};
use app::game::user::Role;

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
    pub resp :Responce,
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
pub enum Events {
    ChangePosition {
        pos :Position,
    },
    Fire {
        pos :Position,
        dir :Direction,
    },
}

pub trait GameObject {
    fn process (&mut self, msg :MessageContainer) ->  errors::LogicResult<Events>;
}

pub struct Game {
    pub users : HashMap<String, User>,
    pub logic : Logic,
}

pub struct Logic {
    pub obj_list :  Vec<Box<GameObject>>,
}

impl Game {
    pub fn process_message(&mut self, msg :MessageContainer) -> LogicResult<ResponceContainer>{
        if self.users.len() != 2 {
            return Err(GameLogicError{info: "No players".to_string()});
        }
        let mut user = self.users.get_mut(&msg.meta.user_name).unwrap();
        let rsp = user.process_as(&msg.clone(), &mut self.logic)?;
        Ok(ResponceContainer{meta: msg.meta, resp: rsp})
    }

    pub fn add_player(&mut self, user :String) -> LogicResult<()>{
        if self.users.len() == 2 {
            return Err(GameLogicError{info: "lobby is full".to_string()});
        }
        self.users.insert(user.clone(), User::new(user));
        return Ok(());
    }

    pub fn remove_player(&mut self, user :String) -> LogicResult<()>{
        if self.users.len() == 0 {
            return Err(GameLogicError{info: "lobby is full".to_string()});
        }
        self.users.remove(&user);
        Ok(())
    }

    pub fn new() -> Game{
        Game{logic: Logic{obj_list: vec![Box::new(tank::Tank::new()),]}, users: HashMap::new()}
    }
}