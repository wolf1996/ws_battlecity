use app::game::errors;
use app::game::logic::{Position, GameObject, MessageContainer, Events, Commands};
use std::rc::Rc;
use app::game::events;
use app::game::logic::Direction;
use app::game::logic::EventContainer;
use app::game::map::GameField;

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Status {
    Moving{
        delta : usize,
    },
    Standing,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tank {
    dir :Direction,
    id :usize,
    owner :usize,
    state: Status,
}

impl Tank {
    fn process_message_container (&mut self, brok: &mut events::Broker, map: &mut GameField, msg :MessageContainer) ->  errors::LogicResult<Vec<Events>>{
        match msg.msg.cmd {
            Commands::Move{direction:dir} =>{
                return self.moving_command_process(brok, map, dir);
            },
            Commands::ChangeDirection{newdir: dir} => {
                return self.change_direction_command_process(dir);
            },
            _ => return Ok(vec![Events::Error{err: "invalid command".to_owned(), user: msg.meta.user_name.clone()},])
        }
        Ok(vec![Events::ChangeDirection{dir:self.dir.clone() },],)
    }

    fn change_direction_command_process(&mut self, dir: Direction) -> errors::LogicResult<Vec<Events>>{
        self.state = Status::Standing;
        self.dir = dir;
        return Ok(vec![Events::ChangeDirection{dir:self.dir.clone() },],); 
    }

    fn moving_command_process(&mut self,brok: &mut events::Broker, map: &mut GameField,  dir: Direction) -> errors::LogicResult<Vec<Events>>{
        self.state = Status::Moving{
            delta: 1,
        };
        self.dir = dir;
        return Ok(vec![Events::ChangeDirection{dir:self.dir.clone() }],); 
    }

    fn moving_tick(&mut self, brok: &mut events::Broker, map: &mut GameField, del: usize) -> errors::LogicResult<Vec<Events>>{
        return map.move_unit(brok, self.id, self.dir.clone(), del); 
    }

    pub fn new(id: usize, owner: usize,  map :&mut GameField,) -> Tank{
        map.add_new(id);
        Tank{ dir:Direction::Up, id: id, owner: owner, state: Status::Standing,}
    }
}

impl GameObject for Tank {
    fn process(&mut self,brok: &mut events::Broker, map: &mut GameField, msg : EventContainer) ->  errors::LogicResult<EventContainer>{
        let mut evs = Vec::new();
        for i in  msg.evs {
            match i {
                Events::Command(sm) => {
                    match self.process_message_container(brok, map, sm){
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
    
    fn tick(&mut self, brok: &mut events::Broker,  map: &mut GameField) -> errors::LogicResult<EventContainer>{
        match self.state.clone() {
            Status::Moving{ delta: delta} => {
                let evs = self.moving_tick(brok, map, delta.clone())?;
                Ok(EventContainer{
                    unit: self.id.clone(),
                    evs : evs,
                })
            },
            Status::Standing => {
                Ok(EventContainer{
                    unit: self.id.clone(),
                    evs : vec![]
                })
            },
        }
    }

    fn key(&self) -> usize {
        self.id.clone()
    }
}