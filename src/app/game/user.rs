use app::game::tank;
use app::game::logic;
use app::game::errors;
use app::game::errors::GameLogicError;
use std::rc::Rc;
use app::game::events;
use std::boxed::Box;
use app::game::logic::Position;
use app::game::logic::Events;
use app::game::logic::GameObject;
use std::borrow::Borrow;
use app::game::events::Broker;
use std::cell::RefCell;
use app::game::logic::EventContainer;
use app::game::map::GameField;

pub struct User {
    id         : usize,
    healpoints : i8,
    units      : Vec<Rc<RefCell<GameObject>>>,
}

impl User {
    pub fn new(id: usize) -> User{
        User{id: id, healpoints: 3, units: Vec::new()}
    }

    pub fn spawn_tank(&mut self, stm: &mut events::Broker,  map: &mut GameField) -> errors::LogicResult<logic::Events>{
        let key = stm.produce_key();
        let tankref = Rc::new(RefCell::new(tank::Tank::new(key, self.id.clone(), map)));
        self.units.push(tankref.clone());
        stm.add_system(tankref.clone());
        stm.subscribe((*tankref).borrow().key(), self.id.clone());
        let tkcopy = (*tankref).borrow().clone();
        Ok(logic::Events::Spawned(logic::Unit::Tank(tkcopy)))
    }
}

impl GameObject for User {
    fn process(&mut self, brok: &mut events::Broker, map: &mut GameField, msg : EventContainer) ->  errors::LogicResult<EventContainer>{
        match msg {
            _ => unimplemented!(),
        }
    }
    
    fn tick(&mut self, brok: &mut events::Broker, map: &mut GameField) -> errors::LogicResult<EventContainer>{
        println!("tick processed");
        if self.units.len() < 1 {
            let ev = self.spawn_tank(brok, map)?;
            return Ok(EventContainer{
                unit: self.id.clone(),
                evs : vec![ev,]
            });
        }
        Ok(EventContainer{
            unit: self.id.clone(),
            evs : vec![],
        })
    }

    fn key(&self) -> usize {
        self.id.clone()
    }
}