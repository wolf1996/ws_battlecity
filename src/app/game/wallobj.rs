use app::game::errors::LogicResult;
use app::game::events::{EventContainer, AddresableEventsList};
use app::game::logic::{GameObject, InfoObject};

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
        _msg: EventContainer,
    ) -> LogicResult<()> {
        Ok(())
    }
    fn tick(&mut self) -> LogicResult<AddresableEventsList> {
        Ok(Vec::new())
    }
    fn key(&self) -> usize {
        self.key.clone()
    }
    fn get_info(&self) -> LogicResult<Box<InfoObject>> {
        Ok(Box::new(WallInfo::new(self.key.clone())))
    }
}
