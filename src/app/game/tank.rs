use app::game::errors;
use app::game::logic::{Position, GameObject, MessageContainer, Events};

pub struct Tank {
    pos :Position,
}

impl Tank{
    pub fn new() -> Tank{
        Tank{pos: Position{x: 0.0, y:0.0}}
    }
}

impl GameObject for Tank {
    fn process (&mut self, msg :MessageContainer) ->  errors::LogicResult<Events>{
        println!("message processed {:?}", msg);
        Ok(Events::ChangePosition{pos: Position{x: 0., y: 0.}})
    }
}