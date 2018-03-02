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
    pos :Position,
    dir :Direction,
    id :usize,
    owner :usize,
    state: Status,
}

impl Tank {
    fn process_message_container (&mut self, msg :MessageContainer) ->  errors::LogicResult<Vec<Events>>{
        match msg.msg.cmd {
            Commands::Move{direction:dir} =>{
                return self.moving_command_process(dir);
            },
            Commands::ChangeDirection{newdir: dir} => {
                return self.change_direction_command_process(dir);
            },
            _ => return Ok(vec![Events::Error{err: "invalid command".to_owned(), user: msg.meta.user_name.clone()},])
        }
        Ok(vec![Events::ChangePosition{pos: self.pos.clone(), dir:self.dir.clone() },],)
    }

    fn change_direction_command_process(&mut self, dir: Direction) -> errors::LogicResult<Vec<Events>>{
        self.state = Status::Standing;
        self.dir = dir;
        return Ok(vec![Events::ChangePosition{pos: self.pos.clone(), dir: self.dir.clone()},],); 
    }

    fn moving_command_process(&mut self, dir: Direction) -> errors::LogicResult<Vec<Events>>{
        self.state = Status::Moving{
            delta: 1,
        };
        self.dir = dir;
        return Ok(vec![Events::ChangePosition{pos: self.pos.clone(), dir: self.dir.clone()},],); 
    }

    fn moving_tick(&mut self, del: usize) -> errors::LogicResult<Vec<Events>>{
        match self.dir {
            Direction::Down => {
                self.pos.y -= 1.;
            },
            Direction::Left => {
                self.pos.x -= 1.;
            },
            Direction::Up => {
                self.pos.y += 1.;
            },
            Direction::Right => {
                self.pos.x += 1.;
            },
        }
        return Ok(vec![Events::ChangePosition{pos: self.pos.clone(), dir: self.dir.clone()},],); 
    }

    pub fn new(id: usize, owner: usize,  map :&mut GameField,) -> Tank{
        map.add_new(id);
        Tank{pos: Position{x: 0.0, y:0.0}, dir:Direction::Up, id: id, owner: owner, state: Status::Standing,}
    }
}

impl GameObject for Tank {
    fn process(&mut self,brok: &mut events::Broker, map: &mut GameField, msg : EventContainer) ->  errors::LogicResult<EventContainer>{
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
    
    fn tick(&mut self, brok: &mut events::Broker,  map: &mut GameField) -> errors::LogicResult<EventContainer>{
        match self.state.clone() {
            Status::Moving{ delta: delta} => {
                let evs = self.moving_tick(delta.clone())?;
                Ok(EventContainer{
                    unit: self.id.clone(),
                    evs : evs,
                })
            },
            Status::Standing => {
                Ok(EventContainer{
                    unit: self.id.clone(),
                    evs : vec![Events::ChangePosition{pos: self.pos.clone(), dir: self.dir.clone()},]
                })
            },
        }
    }

    fn key(&self) -> usize {
        self.id.clone()
    }
}