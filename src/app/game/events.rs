use app::game::errors;
use app::game::logic::EventContainer;
use app::game::logic::EventsList;
use app::game::logic::{GameObject, InfoObject};
use app::game::map::GameField;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub const SYSTEM: usize = 0;

pub struct Broker {
    channels: HashMap<usize, Vec<Rc<RefCell<GameObject>>>>,
    units: HashMap<usize, Rc<RefCell<GameObject>>>,
    counter: usize,
}

impl Broker {
    pub fn tick(&mut self, map: &mut GameField) -> errors::LogicResult<EventsList> {
        let mut events = Vec::new();
        // TODO: тут тоже конструкция не нравится
        for (_unit, gobj) in self.units.clone().iter() {
            let evs = gobj.borrow_mut().tick(self, map);
            match evs {
                Ok(some) => events.append(&mut some.clone()),
                Err(some) => return Err(some),
            }
        }
        Ok(events)
    }

    // а вот тут вот следовало бы поправить. скорее всего не будет реакции на
    pub fn pass_direct(
        &mut self,
        key: usize,
        evnt: EventContainer,
        map: &mut GameField,
    ) -> errors::LogicResult<EventsList> {
        let mut unit = match self.units.get_mut(&key) {
            Some(some) => some.clone(),
            None => {
                return Err(errors::GameLogicError {
                    info: "No such unit".to_string(),
                })
            }
        };
        let mut un = RefCell::borrow_mut(&mut unit);
        let ev = un.process(self, map, evnt)?;
        let mut rsp = Vec::new();
        for i in ev {
            let mut buf = self.pass_message(map, i)?;
            rsp.append(&mut buf);
        }
        Ok(rsp)
    }

    // TODO: Реаллизовать паттерн комманда и enum-ами передавать нужные параметры для спауна объекта внутри
    // системы и возвращать Rс на объект
    pub fn add_system(&mut self, gobjo: Rc<RefCell<GameObject>>) -> errors::LogicResult<()> {
        let gobj = gobjo.borrow();
        self.channels.entry(gobj.key()).or_insert(Vec::new());
        self.units.insert(gobj.key(), Rc::clone(&gobjo));
        Ok(())
    }

    pub fn subscribe(&mut self, key: usize, subscriber: usize) -> errors::LogicResult<()> {
        let gobk = match self.units.get(&subscriber) {
            Some(some) => some.clone(),
            None => {
                return Err(errors::GameLogicError {
                    info: "No such unit".to_owned(),
                })
            }
        };
        self.channels.insert(key, vec![gobk]);
        Ok(())
    }

    pub fn pass_message(
        &mut self,
        map: &mut GameField,
        evnt: EventContainer,
    ) -> errors::LogicResult<EventsList> {
        let mut events = vec![evnt];
        let mut ind = 0;
        while events.len() < ind {
            let evnt = events.get(ind).unwrap().clone();
            ind += 1;
            let mut subs = match self.channels.get_mut(&evnt.unit.clone()) {
                Some(some) => some.clone(),
                None => {
                    return Err(errors::GameLogicError {
                        info: "No such channel".to_string(),
                    })
                }
            };
            for i in &mut subs.iter_mut() {
                let mut gobj = RefCell::borrow_mut(i);
                match gobj.process(self, map, evnt.clone()) {
                    Ok(evs) => {
                        events.append(&mut evs.clone());
                    }
                    Err(err) => return Err(err),
                }
            }
        }
        Ok(events)
    }

    pub fn new() -> Broker {
        let brok = Broker {
            units: HashMap::new(),
            channels: HashMap::new(),
            counter: SYSTEM,
        };
        return brok;
    }

    pub fn produce_key(&mut self) -> usize {
        self.counter += 1;
        self.counter.clone()
    }

    pub fn collect_info(&self) -> errors::LogicResult<Vec<Box<InfoObject>>> {
        let mut evs = Vec::new();
        for (ref _i, ref j) in self.units.iter() {
            let mut e2 = RefCell::borrow(j).get_info()?;
            evs.push(e2);
        }
        return Ok(evs);
    }
}
