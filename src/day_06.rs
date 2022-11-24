fn main() {
    let input = include_str!("../day_06_input.txt")
        .split_whitespace()
        .map(|line| {
            line.chars().map(|c| c as u8 - b'a').collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    let mut cache = [[0usize; 26]; 8];
    let mut res = [(0u8, 0usize); 8];

    for row in input {
        for i in 0..8 {
            cache[i][row[i] as usize] += 1;
            if cache[i][row[i] as usize] > res[i].1 {
                res[i] = (row[i], cache[i][row[i] as usize]);
            }
        }
    }

    print!("part1: ");
    for (c, _) in res {
        print!("{}", (c + b'a') as char);
    }
    println!();

    print!("part2: ");
    for column in cache {
        let Some((c, _)) = column.iter().enumerate().fold(None, |p, (idx, count)| {
            match p {
                None => Some((idx, count)),
                Some((pi, pc)) => {
                    if pc > count {
                        Some((idx, count))
                    } else {
                        Some((pi, pc))
                    }
                }
            }
        }) else { panic!("didn't find result") };
        print!("{}", (c as u8 + b'a') as char);
    };
    println!();
}
