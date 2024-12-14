use std::{cmp::Ordering, io::Read, num::ParseIntError};

use math::{Vec2, Vec2Coord};
use regex::Regex;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Robot {
    pub pos: Vec2,
    pub vel: Vec2,
}

#[derive(Debug)]
pub struct Scene {
    size: Vec2,
    objs: Vec<Robot>,
}
impl Scene {
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            objs: Default::default(),
        }
    }

    pub fn with_objects(self, objs: Vec<Robot>) -> Self {
        Self { objs, ..self }
    }

    pub fn next_tick(&mut self) {
        for obj in &mut self.objs {
            let mut new_pos = obj.pos + obj.vel;
            new_pos = [new_pos.x() % self.size.x(), new_pos.y() % self.size.y()].into();
            new_pos = [
                if new_pos.x() < 0 {
                    new_pos.x() + self.size.x()
                } else {
                    new_pos.x()
                },
                if new_pos.y() < 0 {
                    new_pos.y() + self.size.y()
                } else {
                    new_pos.y()
                },
            ]
            .into();
            obj.pos = new_pos;
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error("Malformed entry")]
    Format,
}

fn get_input() -> Vec<Robot> {
    let mut input = String::default();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Unable to read stdin");

    let nb_entries = input.lines().count();

    let reg = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").expect("Invalid regex");
    let matches = reg.captures_iter(&input);
    let mut robots = Vec::<Robot>::with_capacity(nb_entries);
    for m in matches {
        let px = m
            .get(1)
            .ok_or(ParseError::Format)
            .and_then(|px| Vec2Coord::from_str_radix(px.as_str(), 10).map_err(ParseError::ParseInt))
            .unwrap();
        let py = m
            .get(2)
            .ok_or(ParseError::Format)
            .and_then(|py| Vec2Coord::from_str_radix(py.as_str(), 10).map_err(ParseError::ParseInt))
            .unwrap();
        let vx = m
            .get(3)
            .ok_or(ParseError::Format)
            .and_then(|vx| Vec2Coord::from_str_radix(vx.as_str(), 10).map_err(ParseError::ParseInt))
            .unwrap();
        let vy = m
            .get(4)
            .ok_or(ParseError::Format)
            .and_then(|vy| Vec2Coord::from_str_radix(vy.as_str(), 10).map_err(ParseError::ParseInt))
            .unwrap();

        let rob = Robot {
            pos: [px, py].into(),
            vel: [vx, vy].into(),
        };

        robots.push(rob);
    }

    robots
}

fn process_safety_factor(scene: &Scene) -> u32 {
    // Represent each quadrant counter in clockwise order (NW, NE, SE, SW).
    let mut quads = [0, 0, 0, 0];

    let x_mid = scene.size.x() / 2;
    let y_mid = scene.size.y() / 2;
    for obj in &scene.objs {
        match (obj.pos.x().cmp(&x_mid), obj.pos.y().cmp(&y_mid)) {
            (Ordering::Less, Ordering::Less) => quads[0] += 1,
            (Ordering::Greater, Ordering::Less) => quads[1] += 1,
            (Ordering::Greater, Ordering::Greater) => quads[2] += 1,
            (Ordering::Less, Ordering::Greater) => quads[3] += 1,
            _ => {}
        }
    }

    quads.into_iter().reduce(|acc, val| acc * val).unwrap()
}

fn main() {
    let input = get_input();
    let scene_size = Vec2::from([101, 103]);
    let mut scene = Scene::new(scene_size).with_objects(input);
    for _ in 0..100 {
        scene.next_tick();
    }
    let safety_factor = process_safety_factor(&scene);

    println!("{:?}", safety_factor);
}
