use app::game::tank;
use app::game;
use app::game::errors;
use std::boxed::Box;
use std::marker::Copy;
use std::marker::Send;
use std::marker::Sync;

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

pub struct Logic {
    pub obj_list :  Vec<Box<GameObject + Send + Sync>>,
}

impl Logic {
    pub fn process_message(&mut self, msg :MessageContainer) -> errors::LogicResult<ResponceContainer>{
        let unit = msg.msg.unit;
        if unit >= self.obj_list.len() {
            return Err(errors::GameLogicError{info: "Invalid unit".to_string()});
        }
        let ev = self.obj_list[unit].process(msg.clone())?;
        Ok(ResponceContainer{meta: msg.meta, resp: Responce{unit: msg.msg.unit, evs:ev}})
    }

    pub fn new() -> Logic{
        Logic{obj_list: vec![Box::new(tank::Tank::new()),]}
    }
}