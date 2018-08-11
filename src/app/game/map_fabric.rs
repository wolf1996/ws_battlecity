use app::game::maptank::TankMapObj;
use app::game::mapobj::MovementState;
use app::game::events::{Position, Direction};
use std::rc::Rc;
use std::cell::RefCell;
use app::game::map::GameField;

pub struct MapObjectsFabric {
    map: Rc<RefCell<GameField>>,
}

impl MapObjectsFabric {
    pub fn new(map: Rc<RefCell<GameField>>) -> MapObjectsFabric {
        return MapObjectsFabric{
            map: map
        }
    }

    pub fn spawn_tank(&mut self, key :usize) -> Rc<RefCell<TankMapObj>> {
        let maptank_obj = Rc::new(RefCell::new(TankMapObj{
            key: key,
            // TODO: добавить вот тут spawn position generation
            pos: Position{x:0., y: 0.},
            state: MovementState::Stay{dir: Direction::Up}
        }));
        self.map.borrow_mut().add_new(maptank_obj.clone());
        return maptank_obj;
    }
}