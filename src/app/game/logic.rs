use app::game::tank;
use app::game;
use app::game::errors;
use std::boxed::Box;


#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub user_name : String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageContainer {
    pub msg :Message,
    pub meta :Meta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub unit: usize,
    pub cmd: Commands,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponceContainer {
    pub meta :Meta,
    pub evs : Events,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    x : f32,
    y : f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Commands {
    Move,
    ChangeDirection{
        newdir :Direction,
    },
    Fire, 
}

#[derive(Debug, Serialize, Deserialize)]
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
    fn process (&mut self, msg :MessageContainer) ->  errors::LogicResult<()>;
}

pub struct Logic {
    obj_list :  Vec<Box<GameObject>>,
}

impl Logic {
    fn process_message(&mut self, msg :MessageContainer) -> errors::LogicResult<()>{
        let unit = msg.msg.unit;
        self.obj_list[unit].process(msg)
    }
}