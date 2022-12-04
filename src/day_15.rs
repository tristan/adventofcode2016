fn main() {
    // Disc #1 has 17 positions; at time=0, it is at position 5.
    // Disc #2 has 19 positions; at time=0, it is at position 8.
    // Disc #3 has 7 positions; at time=0, it is at position 1.
    // Disc #4 has 13 positions; at time=0, it is at position 7.
    // Disc #5 has 5 positions; at time=0, it is at position 1.
    // Disc #6 has 3 positions; at time=0, it is at position 0.

    let discs = [(17, 5), (19, 8), (7, 1), (13, 7), (5, 1), (3, 0)];

    let part1 = (0i64..)
        .find(|time| {
            discs.iter().enumerate().all(|(i, (num, pos))| {
                (pos + time).rem_euclid(*num) == (num - (i as i64 + 1)).rem_euclid(*num)
            })
        })
        .unwrap();
    println!("part1: {part1}");
    let discs = [(17, 5), (19, 8), (7, 1), (13, 7), (5, 1), (3, 0), (11, 0)];

    let part2 = (0i64..)
        .find(|time| {
            discs.iter().enumerate().all(|(i, (num, pos))| {
                (pos + time).rem_euclid(*num) == (num - (i as i64 + 1)).rem_euclid(*num)
            })
        })
        .unwrap();
    println!("part2: {part2}");
}
