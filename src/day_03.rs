fn main() {
    let input = include_str!("../day_03_input.txt")
        .split_whitespace()
        .map(|s| s.parse::<u16>().unwrap())
        .collect::<Vec<_>>();

    let part1 = input
        .as_slice()
        .chunks(3)
        .filter(|arr| match arr {
            [a, b, c] => a + b > *c && a + c > *b && b + c > *a,
            _ => false,
        })
        .count();
    println!("part1: {part1}");

    let part2 = input
        .as_slice()
        .chunks(9)
        .map(|arr| match arr {
            [a, d, h, b, e, i, c, f, j] => {
                let x = a + b > *c && a + c > *b && b + c > *a;
                let y = d + e > *f && d + f > *e && e + f > *d;
                let z = h + i > *j && h + j > *i && i + j > *h;
                Into::<usize>::into(x) + Into::<usize>::into(y) + Into::<usize>::into(z)
            }
            _ => 0usize,
        })
        .sum::<usize>();
    println!("part2: {part2}");
}
