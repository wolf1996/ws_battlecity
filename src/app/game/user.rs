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

#[derive(Debug)]
enum State {
    Created,
    Finished,
    Alive,
}

pub struct User {
    id: usize,
    healpoints: i8,
    system    : Rc<RefCell<Broker>>,
}

impl User {
    pub fn new(id: usize, system: Rc<RefCell<Broker>>) -> User{
        User{id: id, healpoints: 3, system: system}
    }

    pub fn spawn_tank(&mut self) -> errors::LogicResult<logic::Events>{
        let mut stm = self.system.borrow_mut();
        let key = stm.produceKey();
        let tank = tank::Tank::new(key, self.id.clone());
        stm.add_system(Rc::new(RefCell::new(tank.clone())));
        stm.subscribe(tank.key(), self.id.clone());
        Ok(logic::Events::Spawned(logic::Unit::Tank(tank)))
    }
}

impl GameObject for User {
    fn process(&mut self, msg : Events) ->  errors::LogicResult<Events>{
        match msg {
            _ => unimplemented!(),
        }
    }
    
    fn tick(&mut self) -> errors::LogicResult<Events>{
        println!("tick processed");
        Ok(logic::Events::ChangePosition{pos: Position{x: 0., y: 0.}})
    }

    fn key(&self) -> usize {
        self.id.clone()
    }
}