use app::game::errors;
use app::game::broker::Broker;
use app::game::events::{Direction, Events, Position};
use app::game::mapobj::{MapObject, WallMapObj};
use app::game::logic::info_object_serializer;
use std::borrow::BorrowMut;
use app::game::logic::InfoObject;
use app::game::wallobj::WallObj;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, Serialize)]
pub struct GameFieldInfo{
    maps: HashMap<usize, Box<InfoObject>>,
    map_dim: u32,
    cell_dim: u32,
    item: String, 
}

impl GameFieldInfo {
    fn new(maps: HashMap<usize, Box<InfoObject>>,
            map_dim: u32,
            cell_dim: u32) -> GameFieldInfo{
        GameFieldInfo {
            item: "map".to_owned(),
            map_dim: map_dim,
            cell_dim: cell_dim,
            maps:  maps.clone(),
        }
    }
}

impl InfoObject for GameFieldInfo {}


#[derive(Clone)]
pub struct GameField {
    maps: HashMap<usize, Box<MapObject>>,
    map_dim: u32,
    cell_dim: u32,
}

impl GameField {
    pub fn new() -> Self {
        GameField {
            maps: HashMap::new(),
            map_dim: 500,
            cell_dim: 10,
        }
    }

    pub fn generate_map(&mut self, brok: &mut Broker) {
        let mut key = brok.produce_key();
        let mut wl = Rc::new(RefCell::new(WallObj { key: key }));
        self.maps.insert(
            key,
            Box::new(WallMapObj{
                key: key,
                pos: Position {
                    x: (self.cell_dim as f32) / 2.0 + 10.0,
                    y: (self.cell_dim as f32) / 2.0,
                },
            })
        );
        brok.add_system(wl).expect("can't add wall");
        key = brok.produce_key();
        wl = Rc::new(RefCell::new(WallObj { key: key }));
        self.maps.insert(
            key,
            Box::new(WallMapObj{
                key: key,
                pos: Position {
                    x: (self.cell_dim as f32) / 2.0,
                    y: (self.cell_dim as f32) / 2.0 + 10.0,
                },
            })
        );
        brok.add_system(wl).expect("can't add wall");
    }

    pub fn add_new(&mut self, obj :Box<MapObject>) {
        self.maps.insert(obj.key(), obj);
    }

    pub fn get_position(&self, ind: usize) -> errors::LogicResult<Position> {
        match self.maps.get(&ind) {
            Some(expr) => return Ok(expr.get_position()),
            None => {
                return Err(errors::GameLogicError {
                    info: "Object is not on map".to_owned(),
                })
            }
        };
    }

    fn calc_collision(&self, pos1: &Position, pos2: &Position) -> (f32, f32) {
        let xcoll = (pos1.x - pos2.x).abs() - (self.cell_dim as f32);
        let ycoll = (pos1.y - pos2.y).abs() - (self.cell_dim as f32);
        return (xcoll, ycoll);
    }

    fn check_collisions(
        &self,
        moved: usize,
        dir: Direction,
        new_pos: &mut Position,
    ) -> errors::LogicResult<Vec<Events>> {
        // TODO: оптиPositionмизируй это. Жутко прожорливый алгоритм
        let mut evs = Vec::new();
        for (i, j) in self.maps.iter() {
            if *i == moved {
                continue;
            }
            let (xcoll, ycoll) = self.calc_collision(&j.get_position(), new_pos);
            println!("newpos {:?} secpos {:?}", *new_pos, j.get_info());
            println!("xcoll {} ycoll {}", xcoll, ycoll);
            match dir {
                Direction::Down | Direction::Up => {
                    if (ycoll < 0.0) && (xcoll < 0.0) {
                        match dir {
                            Direction::Down => new_pos.y -= ycoll,
                            Direction::Up => new_pos.y += ycoll,
                            _default => unimplemented!(),
                        }
                        evs.push(Events::Collision {
                            fst: moved,
                            scd: i.clone(),
                        })
                    }
                }
                Direction::Left | Direction::Right => {
                    if (ycoll < 0.0) && (xcoll < 0.0) {
                        match dir {
                            Direction::Left => new_pos.x -= xcoll,
                            Direction::Right => new_pos.x += xcoll,
                            _default => unimplemented!(),
                        }
                        evs.push(Events::Collision {
                            fst: moved,
                            scd: i.clone(),
                        })
                    }
                }
            }
        }
        return Ok(evs);
    }

    // Direction переделать на reference
    // стал прыгать на 8
    pub fn move_unit(
        &mut self,
        ind: usize,
        dir: Direction,
        d: usize,
    ) -> errors::LogicResult<Vec<Events>> {
        let mut unit_pos = match self.maps.get(&ind) {
            Some(expr) => expr.get_position(),
            None => {
                return Err(errors::GameLogicError {
                    info: "Object is not on map".to_owned(),
                })
            }
        };
        match dir {
            Direction::Down => {
                unit_pos.y -= d as f32;
            }
            Direction::Left => {
                unit_pos.x -= d as f32;
            }
            Direction::Up => {
                unit_pos.y += d as f32;
            }
            Direction::Right => {
                unit_pos.x += d as f32;
            }
        }
        let mut coll_check = self.check_collisions(ind, dir.clone(), &mut unit_pos)?;
        let unit_mut = match self.maps.get_mut(&ind) {
            Some(expr) => expr,
            None => {
                return Err(errors::GameLogicError {
                    info: "Object is not on map".to_owned(),
                })
            }
        };
        unit_mut.set_position(unit_pos.clone());
        let mut res = vec![Events::ChangePosition {
            pos: unit_pos,
            dir: dir.clone(),
        }];
        res.append(&mut coll_check);
        return Ok(res);
    }

    pub fn get_info(&self) -> errors::LogicResult<Vec<Box<InfoObject>>> {
        let mut infomap = HashMap::new();
        for (key, ref val) in self.maps.iter() {
            let info = val.get_info()?;
            infomap.insert(key.clone(), info);
        }
        Ok(vec![Box::new(GameFieldInfo::new(infomap, self.map_dim, self.cell_dim)),])
    }
}
