fn main() {
    let input = "^..^^.^^^..^^.^...^^^^^....^.^..^^^.^.^.^^...^.^.^.^.^^.....^.^^.^.^.^.^.^.^^..^^^^^...^.....^....^.";
    let input = input.chars().map(|c| c == '^').collect::<Vec<_>>();
    let init = input.iter().filter(|&is_trap| !is_trap).count();
    let (_, part2) = (0..400000 - 1).fold((input, init), |(prev, count), round| {
        let len = prev.len();
        let next = (0..len)
            .map(|i| {
                let (left, center, right) = match i {
                    0 => (false, prev[0], prev[1]),
                    i if i == len - 1 => (prev[i - 1], prev[i], false),
                    i => (prev[i - 1], prev[i], prev[i + 1]),
                };
                matches!(
                    (left, center, right),
                    (true, true, false)
                        | (false, true, true)
                        | (true, false, false)
                        | (false, false, true)
                )
            })
            .collect::<Vec<_>>();
        let count = count + next.iter().filter(|&is_trap| !is_trap).count();
        if round == 39 {
            println!("part1: {count}");
        }
        (next, count)
    });
    println!("part2: {part2}");
}
