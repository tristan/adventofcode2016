fn print(ptr: usize, e: &[u8]) {
    for (i, e) in e.iter().enumerate() {
        if i == ptr {
            print!(">{}<", i + 1);
        } else if *e > 0 {
            print!(" {} ", i + 1);
        }
    }
    println!()
}

fn main() {
    let input = 3014387;

    let mut elves = (0..input).map(|_| 1u8).collect::<Vec<_>>();

    let mut ptr = 0;
    'outer: while elves.len() > 1 {
        if elves[ptr] > 0 {
            let mut next = (ptr + 1) % elves.len();
            //print(ptr, &elves);
            while elves[next] == 0 {
                next = (next + 1) % elves.len();
                if next == ptr {
                    break 'outer;
                }
            }
            elves[next] = 0;
            ptr = (next + 1) % elves.len();
        } else {
            ptr = (ptr + 1) % elves.len();
        }
    }
    println!("part1: {}", ptr + 1);

    // TODO: optimize this, it takes 5 hours to complete right now!
    let s = std::time::Instant::now();
    let mut elves = (0..input).map(|_| 1u8).collect::<Vec<_>>();
    let mut remaining = input;
    let mut ptr = 0;
    'outer: while elves.len() > 1 {
        if elves[ptr] > 0 {
            let mut half = remaining / 2;
            let mut next = (ptr + 1) % elves.len();
            loop {
                if elves[next] > 0 {
                    half -= 1;
                }
                if half > 0 {
                    next = (next + 1) % elves.len();
                    if next == ptr {
                        break 'outer;
                    }
                } else {
                    break;
                }
            }
            elves[next] = 0;
            remaining -= 1;
            if remaining == 1 {
                break;
            }
            ptr = (ptr + 1) % elves.len();
        } else {
            ptr = (ptr + 1) % elves.len();
        }
    }
    println!("part2: {}", ptr + 1);
    println!("{:?}", s.elapsed());
}
