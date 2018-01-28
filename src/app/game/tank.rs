use app::game::errors;
use app::game::logic::{Position, GameObject, MessageContainer, Events};

struct Tank {
    pos :Position,
}

impl GameObject for Tank {
    fn process (&mut self, msg :MessageContainer) ->  errors::LogicResult<Events>{
        println!("message processed {:?}", msg);
        Ok(Events::ChangePosition{pos: Position{x: 0., y: 0.}})
    }
}