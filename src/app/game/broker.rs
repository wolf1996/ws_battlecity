use app::game::errors;
use app::game::events::{EventContainer, EventsList, AddresableEventsList, AddresableContainer, AddresType, Events, SpawneReq, Unit};
use app::game::logic::{GameObject, InfoObject};
use app::game::map::GameField;
use app::game::tank::{Tank, TankInfo};
use app::game::maptank::TankMapObj;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use app::game::map_fabric::MapObjectsFabric;
use std::any::Any;

pub const SYSTEM: usize = 0;

pub struct Broker {
    channels: HashMap<usize, Vec<Rc<RefCell<GameObject>>>>,
    units: HashMap<usize, Rc<RefCell<GameObject>>>,
    task_queue: Vec<EventContainer>,
    counter: usize,
    map: Rc<RefCell<GameField>>,
    map_objects_fabric: MapObjectsFabric,
}

impl Broker {
    pub fn tick(&mut self) -> errors::LogicResult<EventsList> {
        let mut events = self.process_system_queue()?;
        for i in events.clone() {
            self.pass_message_addresable(i)?;
        }
        // TODO: тут тоже конструкция не нравится
        for (_unit, gobj) in self.units.clone().iter() {
            let mut evs = gobj.borrow_mut().tick()?;
            events.append(&mut evs.clone());
            for i in evs {
                self.pass_message_addresable(i)?;
            }
        }
        let mut result = Vec::new();
        for mut i in events {
            result.append(&mut i.events)
        }
        Ok(result)
    }



    // а вот тут вот следовало бы поправить. скорее всего не будет реакции на
    // заиниченные сообщения
    pub fn pass_direct_unit(
        &mut self,
        key: usize,
        evnt: EventContainer,
    ) -> errors::LogicResult<()> {
        let mut unit = match self.units.get_mut(&key) {
            Some(some) => some.clone(),
            None => {
                return Err(errors::GameLogicError{
                    info: "No such unit".to_string(),
                })
            }
        };
        let mut un = RefCell::borrow_mut(&mut unit);
        let ev = un.process(evnt)?;
        Ok(())
    }

    pub fn pass_addresable(
        &mut self,
        addres: AddresType,
        evnt: EventContainer,
    ) -> errors::LogicResult<()> {
        match addres {
            AddresType::Direct(some) => {
                return self.pass_direct_unit(some, evnt);
            },
            AddresType::System => {
                return self.pass_system(evnt)
            },
            AddresType::Broadcast => {
                // тут передаётся всё на подписки
                return self.pass_broadcast(evnt);
            }
        }
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

    pub fn pass_message_addresable(
        &mut self,
        event: AddresableContainer,
    ) -> errors::LogicResult<()> {
        for i in & event.events {
            for addr in & event.addres {
                self.pass_addresable(addr.clone(), i.clone())?;
            }
        }
        Ok(())
    }

    pub fn process_system_msg(&mut self, evnt: EventContainer) -> errors::LogicResult<AddresableEventsList> {
        let mut res = vec![];
        match evnt.evs {
            Events::SpawneRequest(req) => {
                res = self.spawn_handler(evnt.unit, req)?;
            },
            Events::Spawned{..} => {

            },
            _ => unimplemented!(),
        };
        Ok(res)
    }

    pub fn process_system_queue(&mut self) -> errors::LogicResult<AddresableEventsList> {
        let mut res = vec![];
        for i in self.task_queue.clone(){
            // TODO: придумать, что делать с некоторыми ошибками
            let mut sub_res = self.process_system_msg(i)?;
            res.append(&mut sub_res);
        }
        self.task_queue.clear();
        Ok(res)
    }

    pub fn pass_system(
        &mut self,
        evnt: EventContainer,
    ) -> errors::LogicResult<()>{
        self.task_queue.push(evnt);
        Ok(())
    }

    pub fn spawn_tank(&mut self, 
        user: usize 
    ) -> errors::LogicResult<AddresableEventsList> {
        let key = self.produce_key();
        let mapobj = self.map_objects_fabric.spawn_tank(key);
        let tank = Tank::new(key, user, mapobj);
        let tank_rc = Rc::new(RefCell::new(tank.clone()));
        self.add_system(tank_rc.clone())?;
        self.subscribe(key, user)?;
        // TODO: отправку реквеста до юзера о
        // заспауненном танке
        let inf_box = tank.get_info()?;
        let inf = inf_box.as_any().downcast_ref::<TankInfo>().unwrap();
        let event = Events::Spawned{owner: user, unit: Unit::Tank((*inf).clone())};
        let ev = EventContainer{
            unit: user,
            evs: event,
        };
        let res_event = AddresableContainer{
            addres: vec![AddresType::System, AddresType::Direct(user)],
            events: vec![ev,]
        };
        Ok(vec![res_event])
    }

    pub fn spawn_handler(
        &mut self,
        user :usize, 
        req  :SpawneReq,
    ) -> errors::LogicResult<AddresableEventsList>{
        // TODO: добавить обработку реквеста
        // желательно через "комманду"
        match req {
            SpawneReq::Tank => return self.spawn_tank(user),
            _ => unimplemented!(),
        }
    }

    pub fn pass_broadcast(
        &mut self,
        evnt: EventContainer,
    ) -> errors::LogicResult<()> {
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
            gobj.process(evnt.clone())?;
        }
        Ok(())
    }

    pub fn new(map:  Rc<RefCell<GameField>>) -> Broker {

        let brok = Broker {
            units: HashMap::new(),
            channels: HashMap::new(),
            task_queue: Vec::new(),
            counter: SYSTEM,
            map: map.clone(),
            map_objects_fabric: MapObjectsFabric::new(map),
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
