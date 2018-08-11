use app::game::events::{Position,Direction};
use app::game::errors::LogicResult;
use app::game::logic::InfoObject;
use app::game::mapobj::{MapObject, MovementState};


#[derive(Clone, Debug, Serialize)]
pub struct TankMapInfo {
    pub item: String,
    pub id: usize,
    pub pos: Position,
    pub state: MovementState,
}

impl TankMapInfo {
    pub fn new(id: usize, pos: Position, state: MovementState) -> TankMapInfo {
        TankMapInfo{
            id: id,
            pos: pos,
            state: state,
            item: "map_tank".to_owned(),
        }
    }
}

impl InfoObject for TankMapInfo {}

#[derive(Debug, Serialize, Clone)]
pub struct TankMapObj {
    pub key: usize,
    pub pos: Position,
    pub state: MovementState,
    
}

impl MapObject for TankMapObj {
    fn get_position(&self) -> Position{
        self.pos.clone()
    }
    fn set_position(&mut self, pos :Position){
        self.pos = pos
    }
    fn key(&self) -> usize {
        self.key.clone()
    }
    fn get_info(&self) -> LogicResult<Box<InfoObject>> {
        Ok(Box::new(
            TankMapInfo::new(
                self.key.clone(),
                self.pos.clone(),
                self.state.clone(),
        )))
    }

    fn get_movement(&self) -> (Position, MovementState){
        (self.pos.clone(), MovementState::Stay{dir: Direction::Up})
    }

    fn set_movement(&mut self, mstate: MovementState){
        self.state = mstate;
    }
}
