use app::game::errors;
use app::game::broker;
use app::game::mapobj::{MapObject, MovementState};
use app::game::events::{Commands, Events, MessageContainer, Direction};
use app::game::events::{EventContainer, AddresableEventsList};
use app::game::logic::{GameObject, InfoObject};
use app::game::map::GameField;
use app::game::maptank::TankMapObj;
use std::rc::Rc;
use std::cell::RefCell;
use std::marker::Send;
use std::any::Any;


#[derive(Debug, Serialize, Clone)]
pub struct TankInfo {
    pub dir: Direction,
    pub id: usize,
    pub item: String,
}

impl InfoObject for TankInfo {}

#[derive(Debug, Serialize, Clone)]
pub struct Tank {
    dir: Direction,
    id: usize,
    owner: usize,
    #[serde(skip_serializing)]
    map_obj: Rc<RefCell<TankMapObj>>,
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
        let (_pos, movement) = self.map_obj.borrow_mut().get_movement();
        let new_state = match movement {
            MovementState::Moving{
                dir: dir,
                vel: vel, 
            } => {
                MovementState::Moving{
                    dir: dir,
                    vel: vel,
                }
            },
            MovementState::Stay{
                dir: dir
            } => {
                MovementState::Stay{
                    dir: dir,
                }
            },
        };
        self.map_obj.borrow_mut().set_movement(new_state);
        return Ok(());
    }

    fn moving_command_process(
        &mut self,
        dir: Direction,
    ) -> errors::LogicResult<()> {
        self.map_obj.borrow_mut().set_movement(MovementState::Moving{
            dir: dir,
            vel: 5.0, 
        });
        print!("Movement set");
        return Ok(());
    }

    pub fn new(id: usize, owner: usize, map_obj: Rc<RefCell<TankMapObj>>) -> Tank {
        Tank {
            dir: Direction::Up,
            id: id,
            owner: owner,
            map_obj: map_obj
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
        Ok(vec![])
    }

    fn key(&self) -> usize {
        self.id.clone()
    }

    fn get_info(&self) -> errors::LogicResult<Box<InfoObject>> {
        let tif = TankInfo {
            dir: self.dir.clone(),
            id: self.id.clone(),
            item: "Tank".to_owned(),
        };
        Ok(Box::new(tif))
    }
}
