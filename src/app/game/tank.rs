use app::game::errors;
use app::game::broker;
use app::game::events::{Commands, Events, MessageContainer, Direction};
use app::game::events::{EventContainer, AddresableEventsList};
use app::game::logic::{GameObject, InfoObject};
use app::game::map::GameField;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    Moving { delta: usize },
    Standing,
}

#[derive(Debug, Serialize, Clone)]
pub struct TankInfo {
    pub dir: Direction,
    pub id: usize,
    pub item: String,
    pub state: Status,
}

impl InfoObject for TankInfo {}

#[derive(Debug, Serialize, Clone)]
pub struct Tank {
    dir: Direction,
    id: usize,
    owner: usize,
    state: Status,
}

impl Tank {
    fn process_message_container(
        &mut self,
        msg: MessageContainer,
    ) -> errors::LogicResult<()> {
        match msg.msg.cmd {
            Commands::Move { direction: dir } => {
                return self.moving_command_process(dir);
            }
            Commands::ChangeDirection { newdir: dir } => {
                return self.change_direction_command_process(dir);
            }
            _ => {
                return Ok(())// TODO: error 
            }
        }
    }

    fn change_direction_command_process(
        &mut self,
        dir: Direction,
    ) -> errors::LogicResult<()> {
        self.state = Status::Standing;
        self.dir = dir;
        return Ok(());
    }

    fn moving_command_process(
        &mut self,
        dir: Direction,
    ) -> errors::LogicResult<()> {
        self.state = Status::Moving { delta: 1 };
        self.dir = dir;
        return Ok(());
    }

    pub fn new(id: usize, owner: usize) -> Tank {
        Tank {
            dir: Direction::Up,
            id: id,
            owner: owner,
            state: Status::Standing
        }
    }
}

impl GameObject for Tank {
    fn process(
        &mut self,
        msg: EventContainer,
    ) -> errors::LogicResult<()> {
        match msg.evs {
            Events::Command(sm) => {
                self.process_message_container(sm)?;
            }
            _ => unimplemented!(),
        };
        Ok(())
    }

    fn tick(
        &mut self,
    ) -> errors::LogicResult<AddresableEventsList> {
        match self.state.clone() {
            Status::Moving {delta: delta} => {
                Ok(vec![]) // вот тут надо уточнить
            }
            Status::Standing => Ok(vec![]),
        }
    }

    fn key(&self) -> usize {
        self.id.clone()
    }

    fn get_info(&self) -> errors::LogicResult<Box<InfoObject>> {
        let tif = TankInfo {
            dir: self.dir.clone(),
            id: self.id.clone(),
            item: "Tank".to_owned(),
            state: self.state.clone(),
        };
        Ok(Box::new(tif))
    }
}
