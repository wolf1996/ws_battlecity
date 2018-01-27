use app::game::errors;
use app::game::logic::{Position, GameObject, MessageContainer};

struct Tank {
    pos :Position,
}

impl GameObject for Tank {
    fn process (&mut self, msg :MessageContainer) ->  errors::LogicResult<()>{
        println!("message processed {:?}", msg);
        Ok(())
    }
}