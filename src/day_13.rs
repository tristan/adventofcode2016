use std::collections::{VecDeque, HashSet};

fn is_wall(x: i64, y: i64, favorite_number: i64) -> bool {
    let val = x * x + 3 * x + 2 * x * y + y + y * y + favorite_number;
    val.count_ones() % 2 == 1
}

fn solve(input: i64, target_x: i64, target_y: i64) -> Option<(usize, usize)> {
    let mut explored = HashSet::with_capacity(1000);
    let mut queue = VecDeque::with_capacity(1000);
    queue.push_back((0, 1, 1));

    let mut part2 = 0;

    while let Some((steps, x, y)) = queue.pop_front() {
        if x == target_x && y == target_y {
            return Some((steps, part2));
        }
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let x = x + dx;
            let y = y + dy;
            if x >= 0 && y >= 0 && explored.insert((x, y)) && !is_wall(x, y, input) {
                if steps < 50 {
                    part2 += 1;
                }
                queue.push_back((steps + 1, x, y));
            }
        }
    }
    None
}

fn main() {
    let input = 1364;
    let (part1, part2) = solve(input, 31, 39).unwrap();
    println!("part1: {part1}");
    println!("part2: {part2}");
}
