use std::{collections::HashMap, cmp::Ordering};

fn main() {
    let input = include_str!("../day_04_input.txt")
        .split_whitespace()
        .map(|s| {
            let (start, end) = s.rsplit_once('-').unwrap();
            let s = start.replace('-', "");
            let mut map = s.chars().fold(HashMap::<char, usize>::new(), |mut map, c| {
                *map.entry(c).or_default() += 1;
                map
            }).into_iter().collect::<Vec<(char, usize)>>();
            map.sort_unstable_by(|a, b| match b.1.partial_cmp(&a.1).unwrap() {
                Ordering::Equal => {
                    a.0.partial_cmp(&b.0).unwrap()
                },
                other => other,
            });
            let end = end.replace(']', "");
            let (sector_id, checksum) = end.split_once('[').unwrap();
            (sector_id.parse::<i32>().unwrap(), checksum.to_string(), map, start)
        })
        .collect::<Vec<_>>();

    let part1 = input.iter().filter_map(|(sector_id, checksum, map, _)| {
        if checksum.chars().zip(map).all(|(a, &(b, _))| a == b) {
            Some(sector_id)
        } else {
            None
        }
    }).sum::<i32>();
    println!("part1: {part1}");

    input.iter().for_each(|(sector_id, checksum, map, enc)| {
        if checksum.chars().zip(map).all(|(a, &(b, _))| a == b) {
            let s = enc.chars().map(|c| if c == '-' {
                ' '
            } else {
                let as_int = c as u8 - b'a';
                let start = (*sector_id % 26) as u8;
                (((start + as_int) % 26) + b'a') as char
            }).collect::<String>();
            if s.contains("north") {
                println!("{sector_id}: {s}");
            }
        }
    });

}
