use app::game::events::Position;
use app::game::logic::InfoObject;
use app::game::errors::LogicResult;

pub trait MapObject: MapObjectClone{
    fn get_position(&self) -> Position;
    fn set_position(&mut self, pos :Position);
    fn key(&self) -> usize;
    fn get_info(&self) -> LogicResult<Box<InfoObject>>;
}

pub trait MapObjectClone {
    fn clone_mobj_box(&self) -> Box<MapObject>;
}

impl<T> MapObjectClone for T
where
    T: 'static + MapObject + Clone,
{
    fn clone_mobj_box(&self) -> Box<MapObject> {
        Box::new(self.clone())
    }
}

impl Clone for Box<MapObject> {
    fn clone(&self) -> Box<MapObject> {
        self.clone_mobj_box()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct WallMapInfo {
    pub item: String,
    pub id: usize,
    pub pos: Position
}

impl WallMapInfo {
    pub fn new(id: usize, pos: Position) -> WallMapInfo {
        WallMapInfo{
            id: id,
            pos: pos,
            item: "map_wall".to_owned(),
        }
    }
}

impl InfoObject for WallMapInfo {}

#[derive(Debug, Serialize, Clone)]
pub struct WallMapObj {
    pub key: usize,
    pub pos: Position
}

impl MapObject for WallMapObj {
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
        Ok(Box::new(WallMapInfo::new(self.key.clone(), self.pos.clone())))
    }
}
