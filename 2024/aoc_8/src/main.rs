use std::{
    io::{stdin, Read},
    time::Instant,
};

mod grids;
mod vec;

use grids::{BinGrid, Grid};
use vec::{Vec2, Vec2Coord};

fn parse_input(input: &str) -> Grid {
    let chars = input.lines().enumerate().flat_map(|(row, line)| {
        line.chars().enumerate().map(move |(col, c)| {
            let row: Vec2Coord = row.try_into().expect("greater than u8::MAX");
            let col: Vec2Coord = col.try_into().expect("greater than u8::MAX");
            (row, col, c)
        })
    });
    let size: Vec2 = chars
        .clone()
        .last()
        .map(|(max_row, max_col, _)| [max_col + 1, max_row + 1].into())
        .expect("input should contains one char");

    let mut grid = Grid::new(size);
    for (row, col, c) in chars {
        if !c.is_alphanumeric() {
            continue;
        }

        let p: Vec2 = [col, row].into();
        grid.insert(p, c)
            .expect("All points should be inbound since bounds are found from points");
    }

    grid
}

fn process_antinodes(antennas: &Grid, max_harmonic: Vec2Coord, exclude_zero: bool) -> BinGrid {
    let mut antinodes = BinGrid::new(antennas.size());

    for (pos, freq) in antennas.iter() {
        for (pos2, freq2) in antennas.iter() {
            let is_same_antenna: bool = pos == pos2 && freq == freq2;
            if is_same_antenna {
                continue;
            }
            if freq != freq2 {
                continue;
            }

            let diff = *pos2 - *pos;

            // Process positive harmonics
            {
                let mut harmonic = if exclude_zero { 1 } else { 0 };
                loop {
                    let p = *pos - (diff * harmonic);

                    let insert_failed: bool = antinodes.mark(p).is_err();
                    if insert_failed || harmonic >= max_harmonic {
                        break;
                    }

                    harmonic += 1;
                }
            }
            // Process negative harmonics
            {
                let mut harmonic = if exclude_zero { 1 } else { 0 };
                loop {
                    let p = *pos2 + (diff * harmonic);

                    let insert_failed: bool = antinodes.mark(p).is_err();
                    if insert_failed || harmonic >= max_harmonic {
                        break;
                    }

                    harmonic += 1;
                }
            }
        }
    }

    antinodes
}

fn main() {
    let input = {
        let mut input = String::new();
        stdin()
            .read_to_string(&mut input)
            .expect("Can't read input from stdin");
        input
    };

    let mut before: Instant;

    before = Instant::now();
    let antennas = parse_input(&input);
    let data_process_time = before.elapsed();

    before = Instant::now();
    let antinodes_part1 = process_antinodes(&antennas, 1, true);
    let part1_time = before.elapsed();

    before = Instant::now();
    let antinodes_part2 = process_antinodes(&antennas, Vec2Coord::MAX, false);
    let part2_time = before.elapsed();

    println!("Data processed in {:.2?}", data_process_time);
    println!(
        "part1 = {}, processed in {:.2?}",
        antinodes_part1.len(),
        part1_time
    );
    println!(
        "part2 = {}, processed in {:.2?}",
        antinodes_part2.len(),
        part2_time
    );
}
