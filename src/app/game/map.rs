use std::collections::HashMap;
use app::game::logic::{Position, Direction, Events};
use app::game::errors;

#[derive(Debug)]
struct GameField {
    maps: HashMap<usize,Position>
}

impl GameField {
    fn new() -> Self {
        GameField{
            maps: HashMap::new(),
        }
    }

    fn add_new(&mut self,ind: usize){
        self.maps.insert(ind, Position{x:0.0, y:0.0});
    }

    fn get_position(&self, ind: usize) -> errors::LogicResult<Position>{
        match self.maps.get(&ind){
            Some(expr) => return Ok(expr.clone()),
            None => return Err(errors::GameLogicError{info: "Object is not on map".to_owned()}),
        };
    }

    fn move_unit(&mut self, ind: usize, dir: Direction, d :f32) -> errors::LogicResult<Vec<Events>> {
        let unit = match self.maps.get_mut(&ind){
            Some(expr) => expr,
            None => return Err(errors::GameLogicError{info: "Object is not on map".to_owned()}),
        };
        match dir {
            Direction::Down => {
                unit.y -= d;
            },
            Direction::Left => {
                unit.x -= d;
            },
            Direction::Up => {
                unit.y += d;
            },
            Direction::Right => {
                unit.x += d;
            },
        }
        return Ok(vec![Events::ChangePosition{pos: unit.clone(), dir: dir.clone()},],); 
    }
}