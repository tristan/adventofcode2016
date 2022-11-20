enum Direction {
    U,
    D,
    L,
    R,
}

fn main() {
    use Direction::*;
    let input = vec![
        vec![
            R, R, L, L, D, D, R, L, L, D, U, R, L, D, U, U, D, U, L, D, L, R, L, D, D, D, R, L, D,
            U, L, L, R, D, D, R, D, L, U, U, D, L, L, R, R, D, U, R, R, L, U, D, U, D, U, L, L, U,
            U, D, R, U, U, R, R, D, D, D, D, U, R, U, U, L, L, D, U, L, R, L, R, R, R, D, L, L, L,
            R, D, L, R, D, U, L, D, L, R, U, U, D, R, U, R, L, U, L, U, R, U, U, D, R, L, U, U, D,
            R, D, U, R, U, U, D, D, D, R, D, L, L, R, D, L, U, D, R, L, U, U, U, U, U, U, L, D, U,
            R, D, R, D, D, U, R, L, D, D, R, L, U, U, D, L, R, U, R, R, D, R, L, D, R, D, L, R, R,
            R, L, U, R, U, L, D, L, U, U, U, R, D, R, L, U, U, L, R, D, U, D, L, D, R, R, R, D, D,
            L, D, D, D, R, L, R, L, L, D, R, D, U, U, R, D, U, L, U, U, R, R, R, R, U, D, L, L, U,
            D, L, D, R, L, L, R, R, U, L, L, L, R, D, R, D, R, R, D, R, D, R, D, U, U, L, U, D, L,
            U, L, R, L, L, D, R, U, L, U, R, L, U, R, D, L, L, D, D, R, R, L, U, D, R, D, U, U, L,
            L, D, R, U, L, L, L, R, L, U, L, U, D, D, L, U, R, L, U, U, L, D, R, U, D, R, L, D, U,
            U, D, D, L, L, L, R, U, R, D, R, L, D, R, L, U, U, U, U, U, U, D, R, U, D, L, D, L, U,
            L, U, L, R, R, U, R, R, D, D, D, U, D, R, R, R, D, D, D, L, D, D, L, R, L, L, D, D, U,
            U, L, L, U, D, R, U, R, D, D, D, R, D, D, L, U, R, R, U, R, U, L, U, L, U, U, D, R, L,
            L, U, U, D, D, D, R, U, U, L, R, D, L, D, L, L, R, U, U, D, R, R, L, R, R, R, U, L, L,
            R, R, U, R, U, D, D, U, U, D, U, L, D, U, D, U, U, U, D, U, R, U, D, U, U, D, U, D, R,
            U, L, U, D, U, L, R, D, U, D, U, U, U, U, D, L, L, U, R, D, L, R, R, U, D, U, R, D, D,
            U, U, L, L, D, L, L, R, D, U, D, U, L, R, L, R, D, U, R, L, R, D, R, D, L, R, D, L, R,
            U, R, U, D, U, R, L, U, L, D, D, D, L, D, R, L, U, L, L, R, L, U, R, R, L, L, D, L, R,
            L, L, U, L, L, D, U, U, R, U, U, D, R, U, L, D, D, U, D, L, D, D, R,
        ],
        vec![
            L, U, U, R, U, L, U, R, U, U, R, R, R, D, L, R, D, R, D, D, D, U, D, U, L, R, D, D, L,
            U, U, D, U, U, U, L, R, L, R, R, L, R, U, D, D, L, D, L, U, R, L, R, U, L, U, U, D, U,
            U, D, L, D, R, L, L, U, U, R, L, U, U, U, R, D, U, D, L, U, U, L, L, U, U, D, U, U, D,
            R, D, U, D, U, D, U, R, L, L, U, R, D, U, D, L, D, L, D, D, L, D, U, D, R, L, L, U, U,
            L, D, D, R, U, D, D, D, R, L, R, U, D, R, D, U, D, U, L, L, R, R, D, L, L, D, D, L, R,
            L, L, L, R, L, R, U, R, U, R, L, L, D, U, L, U, D, D, U, U, L, U, L, D, R, R, L, U, U,
            R, D, U, L, R, U, L, U, D, U, L, D, U, L, D, U, L, R, U, L, L, L, R, R, D, R, L, D, R,
            L, D, U, U, L, L, D, D, U, L, R, L, U, D, L, L, L, U, L, D, D, L, U, L, L, U, U, L, U,
            U, R, R, U, L, D, L, U, U, L, D, R, R, U, D, D, D, L, R, D, L, U, L, D, U, L, D, R, R,
            D, L, R, R, R, L, R, U, D, U, R, U, D, D, D, U, D, D, U, R, R, R, L, D, U, U, L, R, D,
            D, L, R, R, R, L, R, U, U, L, D, D, U, R, D, R, D, U, U, L, D, L, L, U, L, U, L, D, R,
            D, R, U, U, L, D, L, U, L, R, U, U, D, U, D, L, U, D, R, L, R, D, U, R, R, R, R, L, U,
            L, U, R, D, R, L, L, L, U, D, R, R, R, D, R, U, R, R, L, R, L, L, U, U, R, D, L, U, R,
            L, U, R, D, U, L, U, R, U, D, D, U, L, L, D, U, U, D, D, L, R, L, U, U, L, R, D, U, U,
            D, R, D, R, U, D, R, L, U, U, U, D, U, R, L, L, R, D, R, R, D, R, U, R, D, D, D, D, U,
            L, L, D, D, U, D, L, D, U, U, D, L, R, L, U, R, U, R, L, D, R, R, D, R, D, U, D, L, R,
            R, D, R, U, D, R, D, L, U, R, R, L, L, L, U, L, U, L, U, L, R, U, D, D, D, U, L, U, L,
            U, L, D, D, R, R, L, U, L, U, U, U, U, R, D, D, U, R, U, R, L, D, L, D, D, D, D, D, R,
            U, L, U, U, U, U, L, L, U, D, D, D, R, R, L, U, U, L, D, U, U, L, R, U, U, D, U, U, R,
            R, L, L, R, D, D, R, U, U, R, L,
        ],
        vec![
            R, R, R, L, L, L, L, U, U, L, L, R, R, L, L, D, R, U, L, U, L, L, L, R, D, L, L, D, L,
            D, D, L, U, R, U, D, L, U, L, U, U, L, L, R, U, R, L, D, U, L, L, U, D, U, L, D, R, U,
            R, D, U, R, L, U, L, L, D, U, D, D, R, L, L, R, U, U, R, D, L, L, U, L, U, U, R, L, U,
            L, L, D, L, R, R, D, D, D, U, L, U, U, R, R, U, D, R, D, R, D, U, R, U, L, D, L, L, U,
            R, U, D, L, L, L, D, D, U, D, L, L, L, L, R, L, D, R, D, R, D, D, R, R, D, L, U, U, D,
            L, L, L, U, L, L, L, L, L, D, D, R, D, L, U, L, L, L, L, R, U, R, R, R, U, U, U, U, L,
            L, D, L, R, D, L, D, L, R, R, R, U, L, R, R, R, R, L, D, L, L, R, D, U, R, U, L, D, D,
            L, U, R, U, R, U, U, L, U, L, D, R, D, D, D, U, R, L, R, D, D, R, D, U, L, L, U, U, R,
            U, D, U, U, U, D, R, D, R, R, L, U, R, U, L, U, L, R, L, U, U, D, D, R, D, U, L, D, R,
            L, U, L, U, L, U, U, L, R, L, D, R, L, U, D, R, R, D, D, D, R, U, D, D, R, D, D, R, D,
            D, R, R, L, R, D, L, R, U, R, D, U, L, U, L, R, R, U, U, U, R, D, R, R, R, D, U, R, D,
            D, R, U, D, U, L, U, U, D, R, D, D, L, D, R, D, D, D, U, L, D, L, R, D, U, U, L, D, U,
            U, L, R, U, D, L, R, R, D, D, D, L, L, D, D, L, L, L, R, R, D, L, D, U, U, L, U, U, L,
            U, L, U, R, R, U, L, L, R, R, U, D, U, D, R, U, D, R, R, R, L, D, L, R, L, U, R, R, L,
            U, D, L, L, L, U, U, D, D, U, D, R, U, R, U, U, D, D, U, R, D, U, R, U, L, R, L, D, U,
            D, R, D, L, U, L, D, U, D, R, U, D, D, D, R,
        ],
        vec![
            D, R, D, R, R, U, U, U, U, U, R, R, L, U, D, L, D, L, R, U, U, U, L, R, L, D, L, R, R,
            R, D, D, U, D, L, U, U, D, U, D, D, L, R, D, U, D, L, R, R, L, L, U, R, U, U, D, U, L,
            L, U, D, L, U, L, L, D, L, L, D, D, U, L, U, D, D, U, D, U, U, L, U, R, L, D, L, D, D,
            U, U, D, D, R, D, D, R, L, U, U, R, L, U, U, D, U, D, U, D, U, R, U, L, L, D, R, L, D,
            D, R, U, D, L, U, R, R, R, L, D, R, L, R, U, L, D, D, L, D, D, L, D, D, D, U, L, D, U,
            D, D, L, L, D, U, L, U, U, D, U, D, D, U, U, L, D, R, L, D, R, L, R, U, R, D, U, L, U,
            D, D, R, R, L, R, R, L, R, R, D, U, L, D, R, R, D, U, D, U, D, R, L, D, U, R, R, L, R,
            L, R, D, L, R, L, R, R, L, R, D, D, D, R, U, L, L, U, L, U, L, D, U, D, D, L, D, L, U,
            L, R, L, L, U, R, R, R, R, U, L, U, U, L, R, U, D, L, D, L, R, D, L, L, U, R, U, R, U,
            U, U, R, D, R, R, R, L, D, D, R, L, R, L, U, R, D, D, U, R, D, D, U, U, R, U, U, U, D,
            D, L, R, U, U, R, R, R, D, L, L, U, U, L, U, D, R, L, D, D, R, D, D, D, D, U, U, D, R,
            L, R, R, D, U, L, D, U, L, R, D, L, L, L, R, U, L, U, L, L, R, D, R, U, L, L, R, D, L,
            R, U, U, U, R, R, R, U, R, L, R, R, D, L, D, R, D, D, L, U, R, L, D, L, U, L, L, D, U,
            U, R, R, D, U, L, U, U, U, R, R, L, L, D, D, U, U, U, U, L, D, D, D, D, L, R, D, D, D,
            R, D, L, D, L, L, U, U, R, L, D, D, R, U, L, U, D, D, R, D, D, D, L, R, D, U,
        ],
        vec![
            D, R, R, R, L, U, R, U, D, U, D, U, U, L, D, L, L, U, R, L, U, U, D, D, R, R, R, D, U,
            D, L, U, R, L, L, D, R, R, L, L, D, D, U, R, U, L, U, D, U, U, R, U, R, U, L, L, R, L,
            D, L, L, U, U, R, D, L, L, D, L, L, D, L, D, L, R, U, D, L, L, L, L, D, R, L, L, U, D,
            L, L, D, U, L, D, D, R, R, U, R, D, R, R, L, R, L, R, R, U, U, R, R, U, D, U, R, R, L,
            D, R, D, L, D, U, R, U, U, L, U, D, R, L, L, U, R, L, U, D, U, R, D, U, L, L, R, L, L,
            D, L, U, R, L, R, R, D, L, L, L, L, U, U, D, R, D, U, L, L, D, L, U, R, D, U, L, D, R,
            D, U, R, R, R, L, D, R, L, U, R, U, L, U, L, U, R, L, L, L, R, R, R, U, U, L, R, L, R,
            R, D, R, D, D, D, L, U, L, R, L, R, R, D, L, U, D, D, U, U, U, U, U, D, R, R, D, L, D,
            U, D, U, U, R, L, D, R, R, R, L, R, U, D, R, U, L, D, R, L, U, R, U, U, L, R, D, L, D,
            D, L, R, U, R, D, L, U, R, U, L, R, R, L, D, U, R, L, D, U, U, R, U, R, U, L, R, D, U,
            D, R, R, U, D, U, D, D, L, R, L, U, U, R, U, R, U, L, D, D, L, U, L, U, L, U, L, D, U,
            L, R, R, L, R, R, U, R, U, U, R, L, R, L, L, D, R, R, L, U, D, L, U, U, R, D, R, R, U,
            R, R, U, U, D, U, L, R, L, U, R, R, L, R, L, R, D, D, R, U, R, D, D, L, R, R, D, U, D,
            R, L, L, U, U, U, D, U, L, R, D, R, U, L, U, U, R, R, L, R, L, R, D, U, D, U, L, D, R,
            D, L, L, U, D, R, D, L, L, D, R, U, L, D, L, U, U, U, R, D, D, R, D, D, U, D, D, U, L,
            L, L, D, R, R, D, D, U, D, D, D, D, L, D, D, R, L, R, U, L, R, R, R, U, R, R, D, R, U,
            L, L, D, D, D, U, R, D, D, L, U, R, R, D, D, D, U, D, L, U, R, R, U, D, U, D, L, L, D,
            D, D, L, R, U, U, D, R, L, R, R, L, R, D, U, U, U, D, D, L,
        ],
    ];

    let (part1, _) = input.iter().fold((vec![], 5), |(mut res, last), row| {
        let digit = row.iter().fold(last, |last, dir| match dir {
            U => match last {
                1 | 2 | 3 => last,
                _ => last - 3,
            },
            R => match last {
                3 | 6 | 9 => last,
                _ => last + 1,
            },
            D => match last {
                7 | 8 | 9 => last,
                _ => last + 3,
            },
            L => match last {
                1 | 4 | 7 => last,
                _ => last - 1,
            },
        });
        res.push(digit);
        (res, digit)
    });

    println!("part1: {:?}", part1);

    let (part2, _) = input.iter().fold((vec![], 5), |(mut res, last), row| {
        let digit = row.iter().fold(last, |last, dir| match dir {
            U => match last {
                5 | 2 | 1 | 4 | 9 => last,
                3 => 1,
                6 | 7 | 8 | 10 | 11 | 12 => last - 4,
                13 => 11,
                _ => unreachable!(),
            },
            R => match last {
                1 | 4 | 9 | 12 | 13 => last,
                _ => last + 1,
            },
            D => match last {
                5 | 10 | 13 | 12 | 9 => last,
                1 => 3,
                2 | 3 | 4 | 6 | 7 | 8 => last + 4,
                11 => 13,
                _ => unreachable!(),
            },
            L => match last {
                1 | 2 | 5 | 10 | 13 => last,
                _ => last - 1,
            },
        });
        res.push(digit);
        (res, digit)
    });

    // manually convert 10+ to "hex"
    println!("part2: {:?}", part2);
}
