use app::game::errors;
use app::game::logic::{Position, GameObject, MessageContainer, Events};
use std::rc::Rc;
use app::game::events;
use app::game::logic::Direction;
use app::game::logic::EventContainer;

#[derive(Debug)]
enum Status {
    Moving{
        dir: Direction,
        delta : usize,
    },
    Standing{
        dir: Direction,
    },
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tank {
    pos :Position,
    id :usize,
    owner :usize,
}

impl Tank {
    fn process_message_container (&mut self, msg :MessageContainer) ->  errors::LogicResult<Vec<Events>>{
        println!("message processed {:?}", msg);
        self.pos.x += 1.0;
        Ok(vec![Events::ChangePosition{pos: self.pos.clone()},],)
    }

    // fn moving_command_process(&mut self, msg :MessageContainer) -> errors::LogicResult<Events>{
    // }

    pub fn new(id: usize, owner: usize) -> Tank{
        Tank{pos: Position{x: 0.0, y:0.0}, id: id, owner: owner}
    }
}

impl GameObject for Tank {
    fn process(&mut self,brok: &mut events::Broker, msg : EventContainer) ->  errors::LogicResult<EventContainer>{
        let mut evs = Vec::new();
        for i in  msg.evs {
            match i {
                Events::Command(sm) => {
                    match self.process_message_container(sm){
                        Ok(res) => {
                            evs = [&evs[..], &res[..]].concat();
                        },
                        Err(err) => return Err(err),
                    };
                },
                _ => unimplemented!(),
            };
        };
        let mut ev = EventContainer{
            unit: self.id.clone(),
            evs : evs,
        };
        Ok(ev)
    }
    
    fn tick(&mut self, brok: &mut events::Broker) -> errors::LogicResult<EventContainer>{
        println!("tick processed");
        Ok(EventContainer{
            unit: self.id.clone(),
            evs : vec![Events::ChangePosition{pos: Position{x: 0., y: 0.}},]
        })
    }

    fn key(&self) -> usize {
        self.id.clone()
    }
}