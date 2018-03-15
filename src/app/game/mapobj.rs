use app::game::logic::{GameObject,EventContainer,EventsList,Events, InfoObject};
use app::game::events;
use app::game::map::GameField;
use app::game::errors::LogicResult;
use erased_serde;
use serde;

#[derive(Clone, Debug, Serialize)]
pub struct WallInfo{
    item : String,
    id   : usize
}

impl WallInfo{
    fn new(id :usize) -> WallInfo{
        WallInfo{
            item: "Wall".to_owned(),
            id  : id,
        }
    }
}

impl InfoObject for WallInfo {
}

pub struct WallObj{
    pub key :usize,
}

impl GameObject for WallObj {
    fn process (&mut self, brok: &mut events::Broker,  map: &mut GameField, msg :EventContainer) ->  LogicResult<EventsList>{
        Ok(Vec::new())
    }
    fn tick (&mut self, brok: &mut events::Broker,   map: &mut GameField) ->  LogicResult<EventsList>{
        Ok(Vec::new())
    }
    fn key(&self) -> usize{
        self.key.clone()
    }
    fn get_info(&self) -> LogicResult<EventsList>{
        Ok(vec![EventContainer{unit: self.key.clone(), evs :Events::GameInfo(Box::new(WallInfo::new(self.key.clone())))},])
    }
}