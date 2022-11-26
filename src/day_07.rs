use std::ops::ControlFlow;

fn is_abba(s: &[u8]) -> bool {
    s[0] == s[3] && s[1] == s[2] && s[0] != s[1]
}

fn main() {
    let part1 = include_str!("../day_07_input.txt")
        .lines()
        .filter(|line| {
            let r = line.as_bytes().windows(4).try_fold(
                (false, false),
                |(has_abba, in_hyper), window| {
                    if window[0] == b'[' {
                        ControlFlow::Continue((has_abba, true))
                    } else if window[0] == b']' {
                        ControlFlow::Continue((has_abba, false))
                    } else if window[1] == b'['
                        || window[2] == b'['
                        || window[3] == b'['
                        || window[1] == b']'
                        || window[2] == b']'
                        || window[3] == b']'
                    {
                        ControlFlow::Continue((has_abba, in_hyper))
                    } else if in_hyper {
                        if is_abba(window) {
                            ControlFlow::Break((false, true))
                        } else {
                            ControlFlow::Continue((has_abba, true))
                        }
                    } else if has_abba || is_abba(window) {
                        ControlFlow::Continue((true, false))
                    } else {
                        ControlFlow::Continue((false, false))
                    }
                },
            );
            match r {
                ControlFlow::Continue((supports_tls, _))
                | ControlFlow::Break((supports_tls, _)) => supports_tls,
            }
        })
        .count();

    println!("part1: {part1}");

    let part2 = include_str!("../day_07_input.txt")
        .lines()
        .filter(|line| {
            let mut aba = Vec::new();
            let mut bab = Vec::new();
            let r = line
                .as_bytes()
                .windows(3)
                .try_fold((false, false), |(_, in_hyper), window| {
                    if window[0] == b'[' {
                        ControlFlow::Continue((false, true))
                    } else if window[0] == b']' {
                        ControlFlow::Continue((false, false))
                    } else if window[1] == b'['
                        || window[2] == b'['
                        || window[1] == b']'
                        || window[2] == b']'
                    {
                        ControlFlow::Continue((false, in_hyper))
                    } else if window[0] == window[2] && window[0] != window[1] {
                        if in_hyper {
                            let r = [window[1], window[0], window[1]];
                            if aba.contains(&r) {
                                ControlFlow::Break((true, in_hyper))
                            } else {
                                bab.push(r);
                                ControlFlow::Continue((false, in_hyper))
                            }
                        } else {
                            let r = [window[0], window[1], window[2]];
                            if bab.contains(&r) {
                                ControlFlow::Break((true, in_hyper))
                            } else {
                                aba.push(r);
                                ControlFlow::Continue((false, in_hyper))
                            }
                        }
                    } else {
                        ControlFlow::Continue((false, in_hyper))
                    }
                });
            match r {
                ControlFlow::Continue((supports_ssl, _))
                | ControlFlow::Break((supports_ssl, _)) => supports_ssl,
            }
        })
        .count();

    println!("part2: {part2}");
}
