use app::game::logic::{GameObject,EventContainer,EventsList};
use app::game::events;
use app::game::map::GameField;
use app::game::errors::LogicResult;


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
}