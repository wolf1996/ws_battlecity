use app::game::tank;
use app::game::logic;
use app::game::errors;


trait Role {
    fn process_as(&mut self, logic::MessageContainer) -> errors::LogicResult<logic::Responce>;
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
    fn process_as(&mut self, mc :logic::MessageContainer) -> errors::LogicResult<logic::Responce> {
        Ok(logic::Responce{unit: 1 ,evs: logic::Events::ChangePosition{pos: logic::Position{x:0.0, y:0.0}}})
    }
}
