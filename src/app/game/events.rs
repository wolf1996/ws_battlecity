use std::collections::HashMap;
use app::game::logic::GameObject;
use app::game::errors;
use app::game::logic::Events as MessageEvents;
use app::game::logic::EventsList;
use app::game::logic::EventContainer;
use std::cell::RefCell;
use std::rc::Rc;

pub const  SYSTEM: usize = 0;

pub struct Broker {
    channels :HashMap<usize, Vec<Rc<RefCell<GameObject>>>>, 
    units    :HashMap<usize, Rc<RefCell<GameObject>>>,
    counter  :usize,
}

impl Broker {

    pub fn tick(&mut self)-> errors::LogicResult<EventsList> {
        let mut events = Vec::new();
        for (ref unit, ref gobj) in &mut self.units.iter_mut() {
            let evs = gobj.borrow_mut().tick();
            match evs {
                Ok(some) => events.push(some),
                Err(some) => return Err(some),
            }
        }
        Ok(events)
    }

    pub fn pass_direct(&mut self, key: usize, evnt: EventContainer) -> errors::LogicResult<EventsList> {
        let mut unit = match self.units.get_mut(&key){
            Some(some) => some,
            None => return Err(errors::GameLogicError{info:"No such unit".to_string()}),
        };
        let mut un = RefCell::borrow_mut(unit);
        let rsp = match un.process(evnt) {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };
        Ok(vec![rsp, ])
    }

    // TODO: Реаллизовать паттерн комманда и enum-ами передавать нужные параметры для спауна объекта внутри
    // системы и возвращать Rс на объект
    pub fn add_system(&mut self, gobjo: Rc<RefCell<GameObject>>) -> errors::LogicResult<()> {
        let gobj = gobjo.borrow();
        self.channels.entry(gobj.key()).or_insert(Vec::new());
        self.units.insert(gobj.key() ,Rc::clone(&gobjo));
        Ok(())
    }

    pub fn subscribe(&mut self, key: usize , subscriber: usize) -> errors::LogicResult<()> {
        let gobk = match self.units.get(&subscriber) {
            Some(some) => some.clone(),
            None => return Err(errors::GameLogicError{info: "No such unit".to_owned()}), 
        };
        self.channels.insert(key, vec![gobk,]);
        Ok(())
    }

    pub fn pass_message(&mut self, evnt: EventContainer) -> errors::LogicResult<EventsList> {
        let mut events = vec![ evnt,];
        let mut ind = 0;
        while events.len() < ind {
            let evnt = events.get(ind).unwrap().clone();
            ind += 1;
            let mut subs = match self.channels.get_mut(&evnt.unit.clone()){
                Some(some) => some,
                None => return Err(errors::GameLogicError{info:"No such channel".to_string()}),
            };
            for i in &mut subs.iter_mut(){
                let mut gobj = RefCell::borrow_mut(i); 
                match gobj.process(evnt.clone()){
                    Ok(evs) =>{
                        events.push(evs);
                    },
                    Err(err) => return Err(err)
                }
            };
        };
        Ok(events)
    }

    pub fn new() -> Broker {
        let mut brok = Broker{units: HashMap::new() ,channels: HashMap::new(), counter: SYSTEM};
        return brok;
    }
    
    pub fn produceKey(&mut self) -> usize {
        self.counter += 1;
        self.counter.clone()
    }
}