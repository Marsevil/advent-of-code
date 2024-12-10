use std::{collections::VecDeque, io::Read};

use ndarray::prelude::*;

type ElevationT = u8;
type PosT = [u8; 2];
type ElevationMap = Array2<ElevationT>;

const MAX_ELEVATION: ElevationT = 9;

fn get_input_string() -> String {
    let mut input = String::default();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Can't read from stdin");
    input
}

fn parse_input(input: &str) -> ElevationMap {
    let chars = input.lines().enumerate().flat_map(|(line_idx, line)| {
        line.chars()
            .enumerate()
            .map(move |(col_idx, c)| (col_idx, line_idx, c))
    });
    let dim = chars
        .clone()
        .last()
        .map(|(max_x, max_y, _)| (max_y + 1, max_x + 1))
        .expect("input should contains at least one line");

    let mut map = ElevationMap::zeros(dim);
    for (x, y, c) in chars {
        let elevation = c
            .to_digit(10)
            .and_then(|val| ElevationT::try_from(val).ok())
            .expect("elvation should be digit between 0 and 9");
        map[[y, x]] = elevation;
    }

    map
}

fn find_head(map: &ElevationMap) -> Vec<PosT> {
    map.indexed_iter()
        .filter(|(_, val)| **val == 0)
        .map(|((pos0, pos1), _)| {
            [
                pos0.try_into().expect("value too big"),
                pos1.try_into().expect("value too big"),
            ]
        })
        .collect()
}

fn process_score(map: &ElevationMap) -> u16 {
    let heads = find_head(map);

    let mut next_nodes = heads
        .into_iter()
        .map(|head_pos| (head_pos, 0 as ElevationT))
        .collect::<VecDeque<_>>();

    let mut score = 0u16;
    loop {
        let Some((pos, cur_value)) = next_nodes.pop_front() else {
            break;
        };

        // On last value increment score
        if cur_value == MAX_ELEVATION {
            score += 1;
            continue;
        }

        // Otherwise scan neighbourhood
        let next_value = cur_value + 1;
        if let Some(pos0) = pos[0].checked_add(1) {
            if let Some(val) = map.get([pos0.into(), pos[1].into()]) {
                if *val == next_value {
                    next_nodes.push_back(([pos0, pos[1]], *val));
                }
            }
        }
        if let Some(pos0) = pos[0].checked_sub(1) {
            if let Some(val) = map.get([pos0.into(), pos[1].into()]) {
                if *val == next_value {
                    next_nodes.push_back(([pos0, pos[1]], *val));
                }
            }
        }
        if let Some(pos1) = pos[1].checked_add(1) {
            if let Some(val) = map.get([pos[0].into(), pos1.into()]) {
                if *val == next_value {
                    next_nodes.push_back(([pos[0], pos1], *val));
                }
            }
        }
        if let Some(pos1) = pos[1].checked_sub(1) {
            if let Some(val) = map.get([pos[0].into(), pos1.into()]) {
                if *val == next_value {
                    next_nodes.push_back(([pos[0], pos1], *val));
                }
            }
        }
    }

    score
}

fn main() {
    let input = get_input_string();
    let map = parse_input(&input);

    let score = process_score(&map);

    println!("score = {}", score);
}
