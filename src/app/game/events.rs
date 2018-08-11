use app::game::tank;
use app::game::logic::InfoObject;
use app::game::logic::info_object_serializer;
use std::boxed::Box;
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::Send;

const NUM_PLAYERS: usize = 1;

pub type AddresableEventsList = Vec<AddresableContainer>;

#[derive(Debug, Serialize, Clone)]
pub enum AddresType {
    Broadcast,
    Direct(usize),
    System,
}

#[derive(Debug, Serialize, Clone)]
pub struct AddresableContainer {
    pub addres: Vec<AddresType>,
    pub events: EventsList,
}

pub type EventsList = Vec<EventContainer>;

// Вговнокодим. Но надо бы добавить человеческий роутинг
#[derive(Debug, Serialize, Clone)]
pub struct EventContainer {
    pub unit: usize,
    pub evs: Events,
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
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Commands {
    Move { direction: Direction },
    ChangeDirection { newdir: Direction },
    Fire,
}

#[derive(Debug, Serialize, Clone)]
pub enum Unit {
    Tank(tank::TankInfo),
    SomeDefaultUnit,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageContainer {
    pub msg: Message,
    pub meta: Meta,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub user_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub unit: usize,
    pub cmd: Commands,
}

#[derive(Debug, Serialize, Clone)]
pub enum SpawneReq {
    Tank,
}

#[derive(Debug, Serialize, Clone)]
pub enum Events {
    Command(MessageContainer),
    ChangePosition {
        pos: Position,
        dir: Direction,
    },
    ChangeDirection {
        dir: Direction,
    },
    Fire {
        pos: Position,
        dir: Direction,
    },
    Spawned{
        owner: usize,
        unit:  Unit,
    },
    SpawneRequest(SpawneReq),
    UserConnected {
        user_name: String,
    },
    Collision {
        fst: usize,
        scd: usize,
    },
    Error {
        err: String,
        user: String,
    },
    #[serde(serialize_with = "info_object_serializer")]
    GameInfo(Vec<Box<InfoObject>>),
}
