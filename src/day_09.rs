enum State {
    Chars(usize),
    Repeat(usize, usize),
    Closed(usize, usize, String),
}

fn decode(input: &str) -> String {
    let (res, _) = input.chars().fold((String::new(), None), |(mut res, state), n| {
        match state {
            Some(State::Closed(chars, repeat, mut s)) => {
                s.push(n);
                if s.len() == chars {
                    for _ in 0..repeat {
                        res.push_str(&s);
                    }
                    (res, None)
                } else {
                    (res, Some(State::Closed(chars, repeat, s)))
                }
            }
            _ => match n {
                '(' => {
                    (res, Some(State::Chars(0)))
                }
                ')' => match state {
                    Some(State::Repeat(chars, repeat)) => (res, Some(State::Closed(chars, repeat, String::new()))),
                    _ => {
                        res.push(n);
                        (res, state)
                    }
                }
                'x' => match state {
                    Some(State::Chars(chars)) => (res, Some(State::Repeat(chars, 0))),
                    _ => {
                        res.push(n);
                        (res, state)
                    }
                }
                '0'..='9' => match state {
                    Some(State::Chars(chars)) => (res, Some(State::Chars(chars * 10 + (n as u8 - b'0') as usize))),
                    Some(State::Repeat(chars, repeat)) => (res, Some(State::Repeat(chars, repeat * 10 + (n as u8 - b'0') as usize))),
                    _ => {
                        res.push(n);
                        (res, state)
                    }
                }
                ' ' | '\n' => {
                    (res, state)
                }
                _ => {
                    res.push(n);
                    (res, state)
                }
            }
        }
    });
    res
}

fn part2(input: &str) -> usize {
    // assumes input is well behaved and there are no repeats that stop in the middle of a (AAxBB) block
    if let Some(sidx) = input.find('(') {
        if let Some(eidx) = input[sidx + 1..].find(')') {
            let (chars, repeat) = input[sidx + 1..sidx + eidx + 1].split_once('x').unwrap();
            let chars = chars.parse::<usize>().unwrap();
            let res = part2(&input[sidx + eidx + 2..sidx + eidx + 2 + chars]);
            sidx + res * repeat.parse::<usize>().unwrap() + part2(&input[sidx + eidx + 2 + chars..])
        } else {
            sidx + part2(&input[sidx + 1..])
        }
    } else {
        input.len()
    }
}

fn main() {
    let input = include_str!("../day_09_input.txt").trim();

    let res = decode(input);
    println!("part1: {}", res.len());

    // part2: slow but works (with 32gb ram)!
    // let mut res = res;
    // let mut prev_len = res.len();
    // loop {
    //     res = decode(&res);
    //     let len = res.len();
    //     if dbg!(len) == dbg!(prev_len) {
    //         break;
    //     }
    //     prev_len = len;
    // }
    // println!("part2: {}", res.len());

    let res = part2(input);
    println!("part2: {}", res);
}
