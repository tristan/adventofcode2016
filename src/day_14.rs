use std::ops::ControlFlow;

mod md5;

fn solve(input: Vec<u8>, hashes: usize) -> u32 {
    let mut keys = vec![];
    let ControlFlow::Break(solution) = (0u32..)
        .try_fold(vec![], |mut prev, i| {
            let mut data = input.clone();
            if i == 0 {
                data.push(b'0');
            } else {
                let mut i = i;
                while i > 0 {
                    data.insert(input.len(), (i % 10) as u8 + b'0');
                    i /= 10;
                }
            }

            let hash = (1..=hashes).fold(data, |data, off| {
                md5::md5(data).into_iter().flat_map(|byte| {
                    let a = [byte >> 4, byte & 0xf];
                    if off == hashes {
                        a
                    } else {
                        a.map(|c| if c < 10 {
                            c + 48
                        } else {
                            c + 87
                        })
                    }
                }).collect::<Vec<_>>()
            });

            for arr in hash.windows(5) {
                if arr[0] == arr[1] && arr[1] == arr[2] && arr[2] == arr[3] && arr[3] == arr[4] {
                    while let Some(idx) = prev.iter().enumerate().find_map(|(idx, (n, _, _))| {
                        if *n == arr[0] {
                            Some(idx)
                        } else {
                            None
                        }
                    }) {
                        let (_, key_idx, _) = prev.remove(idx);
                        let len = keys.len() + 1;
                        if len == 64 {
                            return ControlFlow::Break(key_idx);
                        } else {
                            keys.push(key_idx);
                        }
                    }
                }
            }

            if let Some(triplet) = hash.windows(3).find_map(|arr| {
                if arr[0] == arr[1] && arr[1] == arr[2] {
                    Some((arr[0], i, 1000))
                } else {
                    None
                }
            }) {
                prev.push(triplet);
            }

            ControlFlow::Continue(prev.into_iter().filter_map(|(n, i, r)| {
                if r > 0 {
                    Some((n, i, r - 1))
                } else {
                    None
                }
            }).collect())
        }) else { panic!("failed to find answer!") };
    solution
}

fn main() {
    let input = b"jlmsuwbz".to_vec();
    let part1 = solve(input.clone(), 1);
    println!("part1: {part1}");
    let part2 = solve(input, 2017);
    println!("part2: {part2}");
}
