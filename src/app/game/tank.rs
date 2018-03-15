use app::game::errors;
use app::game::logic::{Position, GameObject, MessageContainer, Events, Commands, InfoObject};
use std::rc::Rc;
use app::game::events;
use app::game::logic::Direction;
use app::game::logic::{EventContainer, EventsList};
use app::game::map::GameField;

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Status {
    Moving{
        delta : usize,
    },
    Standing,
}

#[derive(Debug, Serialize, Clone)]
pub struct TankInfo{
    dir   :Direction,
    id    :usize,
    item  :String,
    state :Status,
}


impl InfoObject for TankInfo {
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
    fn process(&mut self,brok: &mut events::Broker, map: &mut GameField, msg : EventContainer) ->  errors::LogicResult<EventsList>{
        let mut evs = Vec::new();
        match msg.evs {
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
        let mut res = Vec::new();
        for i in evs {
            let mut ev = EventContainer{
                unit: self.id.clone(),
                evs : i,
            };
            res.push(ev)
        };
        Ok(res)
    }
    
    fn tick(&mut self, brok: &mut events::Broker,  map: &mut GameField) -> errors::LogicResult<EventsList>{
        match self.state.clone() {
            Status::Moving{ delta: delta} => {
                let evs = self.moving_tick(brok, map, delta.clone())?;
                let mut res = Vec::new();
                for i in evs {
                    let ip = EventContainer{
                        unit: self.id.clone(),
                        evs : i,
                    };
                    res.push(ip);
                }
                Ok(res)
            },
            Status::Standing => {
                Ok(vec![])
            },
        }
    }

    fn key(&self) -> usize {
        self.id.clone()
    }

    fn get_info(&self) -> errors::LogicResult<EventsList>{
        let tif = TankInfo{
            dir   :self.dir.clone(),
            id    :self.id.clone(),
            item  :"Tank".to_owned(),
            state :self.state.clone(),
        };
        Ok(vec![EventContainer{unit: self.id.clone(), evs :Events::GameInfo(Box::new(tif))},])
    }
}