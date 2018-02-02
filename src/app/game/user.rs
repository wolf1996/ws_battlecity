use app::game::tank;
use app::game::logic;
use app::game::errors;
use app::game::errors::GameLogicError;


pub trait Role {
    fn process_as(&mut self, &logic::MessageContainer, &mut logic::Logic) -> errors::LogicResult<logic::Responce>;
}


pub struct User {
    id: String,
    healpoints: i8,
}

impl User {
    pub fn new(id: String) -> User{
        User{id: id, healpoints: 3}
    }
    pub fn spawn_tank(&mut self) -> tank::Tank {
        tank::Tank::new()
    }
}

impl Role for User {
    fn process_as(&mut self, msg : &logic::MessageContainer, logic_cnt:  &mut logic::Logic) -> errors::LogicResult<logic::Responce> {
        let unit = msg.msg.unit;
        if unit >= logic_cnt.obj_list.len() {
            return Err(GameLogicError{info: "Invalid unit".to_string()});
        }
        let ev = logic_cnt.obj_list[msg.msg.unit].process(msg.clone())?;
        Ok(logic::Responce{unit: msg.msg.unit, evs:ev})
    }
}
