use std::collections::HashMap;
use app::game::logic::{Position, Direction, Events};
use app::game::errors;
use app::game::events::Broker;
use app::game::mapobj::WallObj;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct GameField {
    maps: HashMap<usize,Position>,
    map_dim:  u32,
    cell_dim: u32,
}

impl GameField {
    pub fn new() -> Self {
        GameField{
            maps: HashMap::new(),
            map_dim: 500,
            cell_dim: 10,
        }
    }

    pub fn generate_map(&mut self, brok :&mut Broker){
        let mut key = brok.produce_key();
        let mut wl = Rc::new(RefCell::new(WallObj{key:key}));
        self.maps.insert(key,Position{x:(self.cell_dim as f32)/2.0+10.0, y:(self.cell_dim as f32)/2.0,});
        brok.add_system(wl);
        key = brok.produce_key();
        wl = Rc::new(RefCell::new(WallObj{key:key}));
        self.maps.insert(key,Position{x:(self.cell_dim as f32)/2.0, y:(self.cell_dim as f32)/2.0+10.0,});
        brok.add_system(wl);
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

    fn calc_collision(&self, pos1 :&Position,  pos2 :&Position) -> (f32, f32) {
        let xcoll = (pos1.x - pos2.x).abs() - (self.cell_dim as f32);
        let ycoll = (pos1.y - pos2.y).abs() - (self.cell_dim as f32);
        return (xcoll, ycoll);
    }

    fn check_collisions(&mut self, moved: usize, dir: Direction, new_pos: &mut Position)  -> errors::LogicResult<Vec<Events>> {
        // TODO: оптимизируй это. Жутко прожорливый алгоритм
        let mut  evs = Vec::new();
        for (i, j) in self.maps.iter() {
            if *i == moved{
                continue;
            }
            let (xcoll,ycoll) = self.calc_collision(&j,new_pos);
            println!("newpos {:?} secpos {:?}", *new_pos, j);
            println!("xcoll {} ycoll {}", xcoll, ycoll);
            match dir{
                Direction::Down | Direction::Up => {
                    if ((ycoll < 0.0) && (xcoll < 0.0)) {
                        match dir {
                            Direction::Down => new_pos.y -= ycoll,
                            Direction::Up => new_pos.y += ycoll,
                            default => unimplemented!()
                        }
                        evs.push(Events::Collision{fst: moved, scd: i.clone()})
                    }
                },  
                Direction::Left | Direction::Right =>{
                    if ((ycoll < 0.0) && (xcoll < 0.0)) {
                        match dir {
                            Direction::Left => new_pos.x -= xcoll,
                            Direction::Right => new_pos.x += xcoll,
                            default => unimplemented!()
                        }
                        evs.push(Events::Collision{fst: moved, scd: i.clone()})
                    }
                }
            }
        }
        return Ok(evs);
    }

    // Direction переделать на reference
    // стал прыгать на 8
    pub fn move_unit(&mut self, brok :&mut Broker, ind: usize, dir: Direction, d :usize) -> errors::LogicResult<Vec<Events>> {
        let mut unit = match self.maps.get(&ind){
            Some(expr) => expr.clone(),
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
        let mut coll_check  = self.check_collisions(ind, dir.clone(), &mut unit)?;
        self.maps.insert(ind, unit.clone());
        let mut res = vec![Events::ChangePosition{pos: unit.clone(), dir: dir.clone()},];
        res.append(&mut coll_check);
        return Ok(res); 
    }
}