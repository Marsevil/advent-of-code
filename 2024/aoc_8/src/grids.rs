use std::collections::{hash_map::Iter, HashMap, HashSet};

use math::Vec2;

mod errors {
    use thiserror::Error;

    #[derive(Debug, Error)]
    #[error("Position is out of bound")]
    pub struct OutOfBoundError {}
}

use errors::OutOfBoundError;

#[derive(Debug, Clone)]
pub struct Grid {
    size: Vec2,
    table: HashMap<Vec2, char>,
}
impl Grid {
    pub fn new(size: Vec2) -> Grid {
        Self {
            size,
            table: Default::default(),
        }
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn iter(&self) -> Iter<'_, Vec2, char> {
        self.table.iter()
    }

    pub fn insert(&mut self, pos: Vec2, value: char) -> Result<(), OutOfBoundError> {
        let inbound: bool =
            pos.x() >= 0 && pos.x() < self.size.x() && pos.y() >= 0 && pos.y() < self.size.y();
        if !inbound {
            return Err(OutOfBoundError {});
        }

        self.table.insert(pos, value);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct BinGrid {
    size: Vec2,
    table: HashSet<Vec2>,
}
impl BinGrid {
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            table: Default::default(),
        }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn mark(&mut self, p: Vec2) -> Result<(), OutOfBoundError> {
        let inbound: bool =
            p.x() >= 0 && p.x() < self.size.x() && p.y() >= 0 && p.y() < self.size.y();
        if !inbound {
            return Err(OutOfBoundError {});
        }

        self.table.insert(p);
        Ok(())
    }
}
