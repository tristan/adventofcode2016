#[derive(Clone, Copy)]
enum Target {
    Bot(usize),
    Output(usize),
}

#[derive(Clone, Default)]
struct Bot {
    targets: Option<(Target, Target)>,
    values: (Option<usize>, Option<usize>)
}

impl Bot {
    fn assign(&mut self, value: usize) -> bool {
        if let Some(value0) = self.values.0 {
            if self.values.1.is_none() {
                if value > value0 {
                    self.values.1 = Some(value);
                } else {
                    self.values.0 = Some(value);
                    self.values.1 = Some(value0);
                }
                true
            } else {
                false
            }
        } else {
            self.values.0 = Some(value);
            true
        }
    }
}

fn main() {
    let instructions = include_str!("../day_10_input.txt");
    let mut bots: [Bot; 210] = [(); 210].map(|_| Bot::default());
    let mut outputs: [Option<usize>; 21] = [None; 21];

    for line in instructions.lines() {
        if let Some(line) = line.strip_prefix("bot ") {
            let mut iter = line.split(' ');
            let bot_idx = iter.next().unwrap().parse::<usize>().unwrap();
            let mut iter = iter.skip(3);
            let target = iter.next().unwrap();
            let low = iter.next().unwrap().parse::<usize>().unwrap();
            let low = if target == "bot" {
                Target::Bot(low)
            } else {
                Target::Output(low)
            };
            let mut iter = iter.skip(3);
            let target = iter.next().unwrap();
            let high = iter.next().unwrap().parse::<usize>().unwrap();
            let high = if target == "bot" {
                Target::Bot(high)
            } else {
                Target::Output(high)
            };
            let bot = &mut bots[bot_idx];
            bot.targets = Some((low, high));
        } else if let Some(line) = line.strip_prefix("value ") {
            let mut iter = line.split(' ');
            let value = iter.next().unwrap().parse::<usize>().unwrap();
            let bot_idx = iter.nth(3).unwrap().parse::<usize>().unwrap();
            bots[bot_idx].assign(value);
        }
    };

    let mut part1 = None;
    let mut part2 = None;
    while part1.is_none() || part2.is_none() {
        for bot_idx in 0..bots.len() {
            if let (Some(low), Some(high)) = bots[bot_idx].values {
                if part1.is_none() && low == 17 && high == 61 {
                    part1 = Some(bot_idx);
                }
                let targets = bots[bot_idx].targets.unwrap();
                match targets.0 {
                    Target::Bot(target_bot_idx) => {
                        if bots[target_bot_idx].assign(low) {
                            bots[bot_idx].values.0 = None;
                        }
                    }
                    Target::Output(out_idx) => {
                        outputs[out_idx] = Some(low);
                        if let [Some(a), Some(b), Some(c)] = &outputs[..3] {
                            part2 = Some(a * b * c);
                        }
                    }
                }
                match targets.1 {
                    Target::Bot(target_bot_idx) => {
                        if bots[target_bot_idx].assign(high) {
                            bots[bot_idx].values.1 = None;
                        }
                    }
                    Target::Output(out_idx) => {
                        outputs[out_idx] = Some(high);
                        if let [Some(a), Some(b), Some(c)] = &outputs[..3] {
                            part2 = Some(a * b * c);
                        }
                    }
                }
            }
        }
    }

    let part1 = part1.unwrap();
    println!("part1: {part1}");
    let part2 = part2.unwrap();
    println!("part2: {part2}");
}
