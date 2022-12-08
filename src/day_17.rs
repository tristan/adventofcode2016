use std::collections::{VecDeque, HashSet};

mod md5;

fn main() {
    let input = b"yjjvjgan".to_vec();

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    seen.insert((0, 0));
    queue.push_back((vec![], (0, 0)));
    let mut paths = vec![];
    while let Some((path, (x, y))) = queue.pop_front() {
        if (x, y) == (3, 3) {
            paths.push(path);
            continue;
        }
        // print!("{x} {y} ");
        // for p in &path {
        //     let p = *p as char;
        //     print!("{p}");
        // }
        // println!();
        let mut room = input.clone();
        room.extend_from_slice(&path);
        let hash = md5::md5(room);
        let u = hash[0] >> 4;
        let d = hash[0] & 0xf;
        let l = hash[1] >> 4;
        let r = hash[1] & 0xf;

        if (11..=15).contains(&u) && y > 0 {
            let mut path = path.clone();
            path.push(b'U');
            queue.push_back((path, (x, y - 1)));
        }
        if (11..=15).contains(&d) && y < 3 {
            let mut path = path.clone();
            path.push(b'D');
            queue.push_back((path, (x, y + 1)));
        }
        if (11..=15).contains(&l) && x > 0 {
            let mut path = path.clone();
            path.push(b'L');
            queue.push_back((path, (x - 1, y)));
        }
        if (11..=15).contains(&r) && x < 3 {
            let mut path = path.clone();
            path.push(b'R');
            queue.push_back((path, (x + 1, y)));
        }
    }
    let part1 = &paths[0];
    print!("part1: ");
    for p in part1 {
        let p = *p as char;
        print!("{p}");
    }
    println!();
    let part2 = paths.into_iter().last().unwrap().len();
    println!("part2: {part2}");
}
