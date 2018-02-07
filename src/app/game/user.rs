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

pub trait Role {
    fn process_as(&mut self, &logic::MessageContainer, &mut logic::Logic) -> errors::LogicResult<logic::Responce>;
}


pub struct User {
    id: String,
    healpoints: i8,
    system    : Rc<Broker>,
}

impl User {
    pub fn new(id: String, system: Rc<Broker>) -> User{
        User{id: id, healpoints: 3, system: system}
    }

    pub fn spawn_tank(&mut self) -> errors::LogicResult<logic::Events>{
        let mut stm : &mut Broker = Rc::get_mut(&mut self.system).unwrap();
        let key: String = stm.produceKey();
        let tank = tank::Tank::new(key, self.id.clone());
        stm.add_system(Rc::new(tank.clone()));
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

    fn key(&self) -> String {
        self.id.clone()
    }
}