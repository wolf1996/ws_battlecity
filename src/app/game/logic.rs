use app::game::errors;
use app::game::errors::LogicResult;
use app::game::events::{AddresableEventsList, MessageContainer};
use app::game::events::{EventsList, Events, EventContainer};
use app::game::broker::SYSTEM;
use app::game::broker;
use app::game::map::GameField;
use app::game::user::User;
use erased_serde::Serialize as ESerialize;
use serde::ser::{SerializeSeq, Serializer};
use std::boxed::Box;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

const NUM_PLAYERS: usize = 1;

pub trait InfoObject: ESerialize + Send + Debug + InfoObjectClone{
}

trait InfoObjectClone {
    fn clone_box(&self) -> Box<InfoObject>;
}

impl<T> InfoObjectClone for T
where
    T: 'static + InfoObject + Clone,
{
    fn clone_box(&self) -> Box<InfoObject> {
        Box::new(self.clone())
    }
}

impl Clone for Box<InfoObject> {
    fn clone(&self) -> Box<InfoObject> {
        self.clone_box()
    }
}

serialize_trait_object!(InfoObject);

pub fn info_object_serializer<S>(
    to_serialize: &Vec<Box<InfoObject>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(to_serialize.len()))?;
    for e in to_serialize {
        seq.serialize_element(e)?;
    }
    seq.end()
}

// TODO: Было бы норм добавить контекст и пихать все брокеры и map туда
pub trait GameObject {
    fn process(
        &mut self,
        msg: EventContainer,
    )-> errors::LogicResult<()>;
    fn tick(&mut self) -> LogicResult<AddresableEventsList>;
    fn key(&self) -> usize;
    fn get_info(&self) -> LogicResult<Box<InfoObject>>;
}

pub struct Game {
    pub users: HashMap<String, Rc<RefCell<User>>>,
    pub logic: Logic,
}

pub struct Logic {
    pub system: Rc<RefCell<broker::Broker>>,
    pub map: Rc<RefCell<GameField>>,
}

impl Game {
    //TODO: State machin бы, но очень долго делать.
    pub fn process_message(&mut self, msg: MessageContainer) -> LogicResult<()> {
        if !self.users.len() < NUM_PLAYERS {
            return Err(errors::GameLogicError{ 
                info :"Not enought players".to_string(),
            });
        }
        let evc = EventContainer {
            unit: SYSTEM,
            evs: Events::Command(msg.clone()),
        };
        self.logic.system.borrow_mut().pass_direct_unit(
            msg.msg.unit,
            evc,
        )?;
        Ok(())
    }

    pub fn add_player(&mut self, user: String) -> LogicResult<EventsList> {
        if self.users.len() >= NUM_PLAYERS {
            return Ok(vec![EventContainer {
                unit: 0,
                evs: Events::Error {
                    err: "lobbi is full".to_owned(),
                    user: user,
                },
            }] as EventsList);
        }
        let key = RefCell::borrow_mut(&mut self.logic.system)
            .produce_key()
            .clone();
        let us = User::new(key);
        let refu = Rc::new(RefCell::new(us));
        self.users.insert(user.clone(), refu.clone());
        RefCell::borrow_mut(&mut self.logic.system).add_system(refu.clone())?;
        let mut res = vec![EventContainer {
            unit: SYSTEM,
            evs: Events::UserConnected { user_name: user },
        }];
        let info = self.collect_info()?;
        let infoevent = EventContainer {
            unit: SYSTEM,
            evs: Events::GameInfo(info),
        };
        res.push(infoevent);
        return Ok(res as EventsList);
    }

    pub fn tick(&mut self) -> LogicResult<EventsList> {
        let mut system = RefCell::borrow_mut(&mut self.logic.system);
        let evs = match system.tick() {
            Ok(some) => some,
            Err(er) => return Err(er),
        };
        Ok(evs)
    }

    pub fn new() -> Game {
        let map = Rc::new(RefCell::new(GameField::new()));
        let mut brok = Rc::new(RefCell::new(broker::Broker::new(map.clone())));
        {
            let mut bt = RefCell::borrow_mut(&mut brok);
            map.borrow_mut().generate_map(&mut bt);
        }
        Game {
            logic: Logic {
                system: brok,
                map: map,
            },
            users: HashMap::new(),
        }
    }

    pub fn collect_info(&self) -> errors::LogicResult<Vec<Box<InfoObject>>> {
        let mapinfo = Box::new(self.logic.map.borrow().clone());
        let mut inf = RefCell::borrow(&self.logic.system).collect_info()?;
        inf.push(mapinfo);
        return Ok(inf);
    }
}
