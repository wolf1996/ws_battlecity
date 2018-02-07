use app::game::errors;
use app::game::logic::{Position, GameObject, MessageContainer, Events};
use std::rc::Rc;
use app::game::events;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tank {
    pos :Position,
    key :String,
    owner :String,
}

impl Tank {
    fn process_message_container (&mut self, msg :MessageContainer) ->  errors::LogicResult<Events>{
        println!("message processed {:?}", msg);
        Ok(Events::ChangePosition{pos: Position{x: 0., y: 0.}})
    }

    pub fn new(key: String, owner: String) -> Tank{
        Tank{pos: Position{x: 0.0, y:0.0}, key: key, owner: owner}
    }
}

impl GameObject for Tank {
    fn process(&mut self, msg : Events) ->  errors::LogicResult<Events>{
        match msg {
            Events::Command(sm) => return self.process_message_container(sm),
            _ => unimplemented!(),
        }
    }
    
    fn tick(&mut self) -> errors::LogicResult<Events>{
        println!("tick processed");
        Ok(Events::ChangePosition{pos: Position{x: 0., y: 0.}})
    }

    fn key(&self) -> String {
        self.key.clone()
    }
}