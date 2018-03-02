use std::collections::HashMap;
use app::game::logic::{Position, Direction, Events};
use app::game::errors;
use app::game::events::Broker;

#[derive(Debug)]
pub struct GameField {
    maps: HashMap<usize,Position>
}

impl GameField {
    pub fn new() -> Self {
        GameField{
            maps: HashMap::new(),
        }
    }

    pub fn add_new(&mut self,ind: usize){
        self.maps.insert(ind, Position{x:0.0, y:0.0});
    }

    pub fn get_position(&self, ind: usize) -> errors::LogicResult<Position>{
        match self.maps.get(&ind){
            Some(expr) => return Ok(expr.clone()),
            None => return Err(errors::GameLogicError{info: "Object is not on map".to_owned()}),
        };
    }

    pub fn move_unit(&mut self, brok :&mut Broker, ind: usize, dir: Direction, d :usize) -> errors::LogicResult<Vec<Events>> {
        let unit = match self.maps.get_mut(&ind){
            Some(expr) => expr,
            None => return Err(errors::GameLogicError{info: "Object is not on map".to_owned()}),
        };
        match dir {
            Direction::Down => {
                unit.y -= d as f32;
            },
            Direction::Left => {
                unit.x -= d as f32;
            },
            Direction::Up => {
                unit.y += d as f32;
            },
            Direction::Right => {
                unit.x += d as f32;
            },
        }
        return Ok(vec![Events::ChangePosition{pos: unit.clone(), dir: dir.clone()},],); 
    }
}