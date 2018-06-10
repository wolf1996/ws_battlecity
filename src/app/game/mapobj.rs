use app::game::errors::LogicResult;
use app::game::events;
use app::game::logic::{EventContainer, EventsList, GameObject, InfoObject};
use app::game::map::GameField;

#[derive(Clone, Debug, Serialize)]
pub struct WallInfo {
    item: String,
    id: usize,
}

impl WallInfo {
    fn new(id: usize) -> WallInfo {
        WallInfo {
            item: "Wall".to_owned(),
            id: id,
        }
    }
}

impl InfoObject for WallInfo {}

pub struct WallObj {
    pub key: usize,
}

impl GameObject for WallObj {
    fn process(
        &mut self,
        _brok: &mut events::Broker,
        _map: &mut GameField,
        _msg: EventContainer,
    ) -> LogicResult<EventsList> {
        Ok(Vec::new())
    }
    fn tick(&mut self, _brok: &mut events::Broker, _map: &mut GameField) -> LogicResult<EventsList> {
        Ok(Vec::new())
    }
    fn key(&self) -> usize {
        self.key.clone()
    }
    fn get_info(&self) -> LogicResult<Box<InfoObject>> {
        Ok(Box::new(WallInfo::new(self.key.clone())))
    }
}
