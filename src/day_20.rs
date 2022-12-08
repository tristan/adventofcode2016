fn main() {
    let input = include_str!("../day_20_input.txt").trim()
        .lines()
        .map(|line| {
            let (from, to) = line.split_once('-').unwrap();
            (from.parse::<u64>().unwrap(), to.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut smallest = 0;
    'outer: loop {
        for &(from, to) in &input {
            if from <= smallest && to >= smallest {
                smallest = to + 1;
                continue 'outer;
            }
        }
        break;
    }
    println!("part1: {smallest}");

    let max_ip = u32::MAX as u64;

    let mut part2 = 0;
    loop {
        // find end of smallest
        let (next_from, next_to) = input.iter()
            .fold((max_ip, max_ip), |(min, min_to), &(from, to)| {
                if from > smallest && from < min {
                    (from, to)
                } else {
                    (min, min_to)
                }
            });
        part2 += next_from - smallest;
        if next_to == max_ip {
            break;
        }
        // find next smallest available ip
        smallest = next_to + 1;
        'find_next_smallest: loop {
            for &(from, to) in &input {
                if from <= smallest && to >= smallest {
                    smallest = to + 1;
                    continue 'find_next_smallest;
                }
            }
            break 'find_next_smallest
        }
    }

    println!("part2: {part2}");
}
