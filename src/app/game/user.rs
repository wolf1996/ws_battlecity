use app::game::errors;
use app::game::events;
use app::game::logic;
use app::game::logic::GameObject;
use app::game::logic::{EventContainer, EventsList, InfoObject};
use app::game::map::GameField;
use app::game::tank;
use std::boxed::Box;
use std::cell::RefCell;
use std::rc::Rc;

// TODO: привести поля в однообразный вид
pub struct User {
    id: usize,
    healpoints: i8,
    units: Vec<Rc<RefCell<GameObject>>>,
}

// TODO: попробовать разобраться с полями, возможно - макросы
#[derive(Clone, Debug, Serialize)]
pub struct UserInfo {
    item: String,
    id: usize,
    hp: i8,
}

impl InfoObject for UserInfo {}

impl User {
    pub fn new(id: usize) -> User {
        User {
            id: id,
            healpoints: 3,
            units: Vec::new(),
        }
    }

    pub fn spawn_tank(
        &mut self,
        stm: &mut events::Broker,
        map: &mut GameField,
    ) -> errors::LogicResult<logic::Events> {
        let key = stm.produce_key();
        let tankref = Rc::new(RefCell::new(tank::Tank::new(key, self.id.clone(), map)));
        self.units.push(tankref.clone());
        stm.add_system(tankref.clone())?;
        stm.subscribe((*tankref).borrow().key(), self.id.clone())?;
        let tkcopy = (*tankref).borrow().clone();
        Ok(logic::Events::Spawned(logic::Unit::Tank(tkcopy)))
    }
}

impl GameObject for User {
    fn process(
        &mut self,
        _brok: &mut events::Broker,
        _map: &mut GameField,
        msg: EventContainer,
    ) -> errors::LogicResult<EventsList> {
        match msg {
            _ => unimplemented!(),
        }
    }

    fn tick(
        &mut self,
        brok: &mut events::Broker,
        map: &mut GameField,
    ) -> errors::LogicResult<EventsList> {
        println!("tick processed");
        if self.units.len() < 1 {
            let ev = self.spawn_tank(brok, map)?;
            return Ok(vec![EventContainer {
                unit: self.id.clone(),
                evs: ev,
            }]);
        }
        Ok(vec![])
    }

    fn key(&self) -> usize {
        self.id.clone()
    }

    fn get_info(&self) -> errors::LogicResult<Box<InfoObject>> {
        let uif = UserInfo {
            id: self.id.clone(),
            item: "User".to_owned(),
            hp: self.healpoints,
        };
        Ok(Box::new(uif))
    }
}
