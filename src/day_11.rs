use std::{
    io::{self, BufRead, Write},
    ops::ControlFlow, env::args, collections::{HashSet, VecDeque},
};

pub fn read_line_from_stdin(text: &str) -> String {
    print!("{}: ", text);
    io::stdout().lock().flush().unwrap();
    io::stdin().lock().lines().next().unwrap().unwrap()
}

fn validate_obj(obj: &str) -> Option<(u32, char)> {
    if let ControlFlow::Continue((Some(p), Some(t))) =
        obj.chars().try_fold((None, None), |(p, t), c| {
            if p.is_none() {
                if let Some(d) = c.to_digit(10) {
                    if d <= 5 {
                        ControlFlow::Continue((Some(d), None))
                    } else {
                        ControlFlow::Break(())
                    }
                } else {
                    ControlFlow::Break(())
                }
            } else if t.is_none() {
                if let Some(p) = p {
                    if c == 'M' || c == 'G' {
                        ControlFlow::Continue((Some(p), Some(c)))
                    } else {
                        ControlFlow::Break(())
                    }
                } else {
                    ControlFlow::Break(())
                }
            } else {
                ControlFlow::Break(())
            }
        })
    {
        Some((p, t))
    } else {
        None
    }
}

fn move_obj(floors: &mut [[u32; 10]; 4], (pos, typ): (u32, char), from: usize, to: usize) -> bool {
    let idx = pos as usize * 2
        - match typ {
            'G' => 2,
            'M' => 1,
            _ => unreachable!("conditions haven't been correctly tested earlier"),
        };
    if floors[from][idx] == 0 {
        false
    } else {
        floors[from][idx] = 0;
        floors[to][idx] = pos;
        true
    }
}

const TOTAL_FLOORS: usize = 4;

fn validate_state<const N: usize>(floors: &[[u32; N]; TOTAL_FLOORS]) -> bool {
    for floor in floors {
        if N == 10 {
            if (floor[1] > 0
                && floor[0] == 0
                && (floor[2] > 0 || floor[4] > 0 || floor[6] > 0 || floor[8] > 0))
                || (floor[3] > 0
                    && floor[2] == 0
                    && (floor[0] > 0 || floor[4] > 0 || floor[6] > 0 || floor[8] > 0))
                || (floor[5] > 0
                    && floor[4] == 0
                    && (floor[0] > 0 || floor[2] > 0 || floor[6] > 0 || floor[8] > 0))
                || (floor[7] > 0
                    && floor[6] == 0
                    && (floor[0] > 0 || floor[2] > 0 || floor[4] > 0 || floor[8] > 0))
                || (floor[9] > 0
                    && floor[8] == 0
                    && (floor[0] > 0 || floor[2] > 0 || floor[4] > 0 || floor[6] > 0))
            {
                return false;
            }
        } else if N == 14 {
            if (floor[1] > 0
                && floor[0] == 0
                && (floor[2] > 0 || floor[4] > 0 || floor[6] > 0 || floor[8] > 0 || floor[10] > 0 || floor[12] > 0))
                || (floor[3] > 0
                    && floor[2] == 0
                    && (floor[0] > 0 || floor[4] > 0 || floor[6] > 0 || floor[8] > 0 || floor[10] > 0 || floor[12] > 0))
                || (floor[5] > 0
                    && floor[4] == 0
                    && (floor[0] > 0 || floor[2] > 0 || floor[6] > 0 || floor[8] > 0 || floor[10] > 0 || floor[12] > 0))
                || (floor[7] > 0
                    && floor[6] == 0
                    && (floor[0] > 0 || floor[2] > 0 || floor[4] > 0 || floor[8] > 0 || floor[10] > 0 || floor[12] > 0))
                || (floor[9] > 0
                    && floor[8] == 0
                    && (floor[0] > 0 || floor[2] > 0 || floor[4] > 0 || floor[6] > 0 || floor[10] > 0 || floor[12] > 0))
                || (floor[11] > 0
                    && floor[10] == 0
                    && (floor[0] > 0 || floor[2] > 0 || floor[4] > 0 || floor[6] > 0 || floor[8] > 0 || floor[12] > 0))
                || (floor[13] > 0
                    && floor[12] == 0
                    && (floor[0] > 0 || floor[2] > 0 || floor[4] > 0 || floor[6] > 0 || floor[8] > 0 || floor[10] > 0))
            {
                return false;
            }
        } else {
            panic!();
        }
    }
    true
}

fn solve<const N: usize>(floors: [[u32; N]; TOTAL_FLOORS]) -> Option<usize> {
    let mut explored = HashSet::with_capacity(1000);
    let mut queue = VecDeque::with_capacity(1000);
    queue.push_back((0, 0, floors));
    explored.insert((0, floors));

    while let Some((steps, current_floor, floors)) = queue.pop_front() {
        if floors[TOTAL_FLOORS - 1].iter().all(|&v| v != 0) {
            return Some(steps);
        }
        for floor_change in [-1, 1] {
            let next_floor = if floor_change == -1 && current_floor > 0 {
                current_floor - 1
            } else if floor_change == 1 && current_floor < TOTAL_FLOORS - 1 {
                current_floor + 1
            } else {
                continue;
            };

            if floor_change == -1 && (0..=next_floor).map(|idx| {
                floors[idx].iter().all(|&v| v == 0)
            }).all(|r| r) {
                // don't bother moving things down if all the floors under are empty
                continue;
            }
            for idx1 in 0..N {
                if floors[current_floor][idx1] == 0 {
                    continue;
                }
                let mut next_floors = floors;
                next_floors[next_floor][idx1] = next_floors[current_floor][idx1];
                next_floors[current_floor][idx1] = 0;
                // don't bother moving 2 things down if we can move 1 down
                if floor_change == -1 && validate_state(&next_floors) {
                //if validate_state(&next_floors) {
                    if explored.insert((next_floor, next_floors)) {
                        queue.push_back((steps + 1, next_floor, next_floors));
                    }
                    continue;
                }
                let mut could_move_2_objs = false;
                for idx2 in idx1 + 1..N {
                    if next_floors[current_floor][idx2] == 0 {
                        continue;
                    }
                    let mut next_floors = next_floors; // copy!
                    next_floors[next_floor][idx2] = next_floors[current_floor][idx2];
                    next_floors[current_floor][idx2] = 0;
                    if validate_state(&next_floors) {
                        could_move_2_objs = true;
                        if explored.insert((next_floor, next_floors)) {
                            queue.push_back((steps + 1, next_floor, next_floors));
                        }
                    }
                }
                // only try move a single object up if we couldn't move 2 objects up
                if !could_move_2_objs && floor_change == 1 && validate_state(&next_floors) && explored.insert((next_floor, next_floors)) {
                    queue.push_back((steps + 1, next_floor, next_floors));
                }
            }
        }
    }
    None
}

fn print_board(floors: &[[u32; 10]; 4], current_floor: usize) {
    for (no, floor) in floors.iter().enumerate().rev() {
        print!("F{} ", no + 1);
        if no == current_floor {
            print!("E ");
        } else {
            print!(". ");
        }
        for (idx, &e) in floor.iter().enumerate() {
            if e == 0 {
                print!(".  ");
            } else if idx % 2 == 0 {
                print!("{e}G ");
            } else {
                print!("{e}M ");
            }
        }
        println!();
    }
}

fn main() {
    let floors = [
        // XG, XM, XG, XM, ....
        [1, 0, 2, 2, 3, 0, 4, 4, 5, 5],
        [0, 1, 0, 0, 0, 3, 0, 0, 0, 0],
        [0; 10],
        [0; 10],
    ];
    if let Some("play") = args().next().as_deref() {
        let mut floors = floors;
        let mut current_floor = 0;
        let mut turns = 0;
        // manually play!
        while floors[3].iter().any(|&v| v == 0) {
            print_board(&floors, current_floor);
            let command = read_line_from_stdin("next move");
            let command = command.to_uppercase();
            let mut command = command.split(' ');
            let Some(dir) = command.next() else {
                println!("Missing direction, expected `U` or `D`");
                continue;
            };
            let next_floor = if dir == "D" {
                if current_floor == 0 {
                    println!("Cannot move further down!");
                    continue;
                } else {
                    current_floor - 1
                }
            } else if dir == "U" {
                if current_floor == 3 {
                    println!("Cannot move further up!");
                    continue;
                } else {
                    current_floor + 1
                }
            } else {
                println!("Invalid direction `{dir}`, expected `U` or `D`");
                continue;
            };
            let Some(obj1) = command.next() else {
                println!("Must take at least 1 object with you on elevator");
                continue;
            };
            let Some(obj1) = validate_obj(obj1) else {
                println!("Invalid object `{obj1}`, expected `XM` or `XG` where X is a number");
                continue;
            };
            let mut next_floors = floors;
            if !move_obj(&mut next_floors, obj1, current_floor, next_floor) {
                println!(
                    "Cannot move object `{}{}`, it's not on the current floor",
                    obj1.0, obj1.1
                );
                continue;
            }
            if let Some(obj2) = command.next() {
                let Some(obj2) = validate_obj(obj2) else {
                    println!("Invalid object `{obj2}`, expected `XM` or `XG` where X is a number");
                    continue;
                };
                if !move_obj(&mut next_floors, obj2, current_floor, next_floor) {
                    println!(
                        "Cannot move object `{}{}`, it's not on the current floor",
                        obj2.0, obj2.1
                    );
                    continue;
                }
            }
            if !validate_state(&next_floors) {
                println!("Invalid Move!");
                continue;
            }
            current_floor = next_floor;
            floors = next_floors;
            turns += 1;
        }
        println!("Finished in {} turns", turns);
    } else {
        // robot play!
        let part1 = solve(floors);
        if let Some(part1) = part1 {
            println!("part1: {part1}");
        }
        let floors = [
            // XG, XM, XG, XM, ....
            [1, 0, 2, 2, 3, 0, 4, 4, 5, 5, 6, 6, 7, 7],
            [0, 1, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0],
            [0; 14],
            [0; 14],
        ];
        let part2 = solve(floors);
        if let Some(part2) = part2 {
            println!("part2: {part2}");
        }
    }
}
