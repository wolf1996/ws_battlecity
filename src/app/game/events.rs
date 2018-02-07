use std::collections::HashMap;
use app::game::logic::GameObject;
use app::game::errors;
use app::game::logic::Events as MessageEvents;
use std::rc::Rc;

static ALL_CHAN_NAME:  &'static str = "ALL";

pub struct Broker {
    channels :HashMap<String, Vec<Rc<GameObject>>>, 
    units    :HashMap<String, Rc<GameObject>>,
    counter  :usize,
}

impl Broker {
    pub fn get_direct(key: String) -> String {
        key+"direct"
    }

    //TODO: Хранить массив Rc по ключу и осуществлять подписки через ключ
    pub fn add_system(&mut self, gobjo: Rc<GameObject>) -> errors::LogicResult<()> {
        let mut gobj = gobjo;
        self.channels.entry(gobj.key()).or_insert(Vec::new());
        self.units.insert(gobj.key() ,gobj.clone());
        let mut objs = self.channels.entry(Broker::get_direct(gobj.key())).or_insert(Vec::new());
        objs.push(gobj);
        Ok(())
    }

    pub fn subscribe(&mut self, key: String , subscriber: String) -> errors::LogicResult<()> {
        let gobk = match self.units.get(&subscriber) {
            Some(some) => some.clone(),
            None => return Err(errors::GameLogicError{info: "No such unit".to_owned()}), 
        };
        self.channels.insert(key, vec![gobk,]);
        Ok(())
    }

    pub fn pass_message(&mut self, key: String, evnt: MessageEvents) -> errors::LogicResult<Vec<(String, MessageEvents)>> {
        let mut events = vec![(key, evnt),];
        let mut ind = 0;
        while events.len() < ind {
            let (key, evnt) = events.get(ind).unwrap().clone();
            ind += 1;
            let mut subs = match self.channels.get_mut(&key){
                Some(some) => some,
                None => return Err(errors::GameLogicError{info:"No such channel".to_string()}),
            };
            for i in &mut subs.iter_mut(){
                let mut gobj = Rc::get_mut(i).unwrap(); 
                let evs = gobj.process(evnt.clone());
                for j in evs{
                    events.push((gobj.key(), j));
                };
            };
        };
        Ok(events)
    }

    pub fn new() -> Broker {
        let mut brok = Broker{units: HashMap::new() ,channels: HashMap::new(), counter: 1};
        brok.channels.insert(ALL_CHAN_NAME.to_string(), Vec::new());
        return brok;
    }
    
    pub fn produceKey(&mut self) -> String {
        self.counter += 1;
        self.counter.to_string()
    }
}