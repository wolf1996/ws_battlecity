use app::game::errors;
use app::game::events;
use app::game::tank;
use app::game::logic::{GameObject, InfoObject};
use app::game::events::{Events, Unit, EventContainer, SpawneReq, AddresableContainer, AddresableEventsList, AddresType};
use std::boxed::Box;
use std::cell::RefCell;
use std::rc::Rc;

// TODO: привести поля в однообразный вид
pub struct User {
    id: usize,
    healpoints: i8,
    units: Vec<usize>,
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
    ) -> errors::LogicResult<events::Events> {
        Ok(events::Events::SpawneRequest(SpawneReq::Tank))
    }
}

impl GameObject for User {
    fn process(
        &mut self,
        msg: EventContainer,
    ) -> errors::LogicResult<()> {
        match msg.evs {
            Events::Spawned{owner: _owner, unit: unit} => {
                if let Unit::Tank(tnk) = unit {
                    self.units.push(tnk.id);
                } else {
                    unimplemented!();
                }
            },
            Events::ChangePosition{pos: pos, dir: dir} => {},
            _ => unimplemented!(),
        }
        Ok(())
    }

    fn tick(
        &mut self,
    ) -> errors::LogicResult<AddresableEventsList> {
        println!("tick processed");
        if self.units.len() < 1 {
            println!("User spawn tank requested");
            let ev = self.spawn_tank()?;
            let addrevlist = vec![
                AddresableContainer{
                    addres: vec![AddresType::System],
                    events: vec![EventContainer {
                            unit: self.id.clone(),
                            evs: ev,
                            }]
                }
            ];
            return Ok(addrevlist);
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
