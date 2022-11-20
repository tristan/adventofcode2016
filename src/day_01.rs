use std::{collections::HashSet, ops::ControlFlow};

enum Instruction {
    R(i32),
    L(i32),
}

enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Position {
    north: i32,
    east: i32,
}

impl Position {
    fn east(mut self, dist: i32) -> Position {
        self.east += dist;
        self
    }

    fn north(mut self, dist: i32) -> Position {
        self.north += dist;
        self
    }
}

fn main() {
    use Direction::*;
    use Instruction::*;

    let input = vec![
        R(2),
        L(1),
        R(2),
        R(1),
        R(1),
        L(3),
        R(3),
        L(5),
        L(5),
        L(2),
        L(1),
        R(4),
        R(1),
        R(3),
        L(5),
        L(5),
        R(3),
        L(4),
        L(4),
        R(5),
        R(4),
        R(3),
        L(1),
        L(2),
        R(5),
        R(4),
        L(2),
        R(1),
        R(4),
        R(4),
        L(2),
        L(1),
        L(1),
        R(190),
        R(3),
        L(4),
        R(52),
        R(5),
        R(3),
        L(5),
        R(3),
        R(2),
        R(1),
        L(5),
        L(5),
        L(4),
        R(2),
        L(3),
        R(3),
        L(1),
        L(3),
        R(5),
        L(3),
        L(4),
        R(3),
        R(77),
        R(3),
        L(2),
        R(189),
        R(4),
        R(2),
        L(2),
        R(2),
        L(1),
        R(5),
        R(4),
        R(4),
        R(2),
        L(2),
        L(2),
        L(5),
        L(1),
        R(1),
        R(2),
        L(3),
        L(4),
        L(5),
        R(1),
        L(1),
        L(2),
        L(2),
        R(2),
        L(3),
        R(3),
        L(4),
        L(1),
        L(5),
        L(4),
        L(4),
        R(3),
        R(5),
        L(2),
        R(4),
        R(5),
        R(3),
        L(2),
        L(2),
        L(4),
        L(2),
        R(2),
        L(5),
        L(4),
        R(3),
        R(1),
        L(2),
        R(2),
        R(4),
        L(1),
        L(4),
        L(4),
        L(2),
        R(2),
        L(4),
        L(1),
        L(1),
        R(4),
        L(1),
        L(3),
        L(2),
        L(2),
        L(5),
        R(5),
        R(2),
        R(5),
        L(1),
        L(5),
        R(2),
        R(4),
        R(4),
        L(2),
        R(5),
        L(5),
        R(5),
        R(5),
        L(4),
        R(2),
        R(1),
        R(1),
        R(3),
        L(3),
        L(3),
        L(4),
        L(3),
        L(2),
        L(2),
        L(2),
        R(2),
        L(1),
        L(3),
        R(2),
        R(5),
        R(5),
        L(4),
        R(3),
        L(3),
        L(4),
        R(2),
        L(5),
        R(5),
    ];

    let (_, res) = input.iter().fold(
        (N, Position { north: 0, east: 0 }),
        |(dir, pos), next| match (dir, next) {
            (N, R(dist)) => (E, pos.east(*dist)),
            (N, L(dist)) => (W, pos.east(-dist)),
            (E, R(dist)) => (S, pos.north(-dist)),
            (E, L(dist)) => (N, pos.north(*dist)),
            (S, R(dist)) => (W, pos.east(-dist)),
            (S, L(dist)) => (E, pos.east(*dist)),
            (W, R(dist)) => (N, pos.north(*dist)),
            (W, L(dist)) => (S, pos.north(-dist)),
        },
    );

    let part1 = res.north.abs() + res.east.abs();
    println!("part1: {part1}");

    let res = input.into_iter().try_fold(
        (N, Position { north: 0, east: 0 }, HashSet::new()),
        |(dir, mut pos, mut set), next| {
            let (dir, (n, e), dist) = match (dir, next) {
                (N, R(dist)) => (E, (0, 1), dist),
                (N, L(dist)) => (W, (0, -1), dist),
                (E, R(dist)) => (S, (-1, 0), dist),
                (E, L(dist)) => (N, (1, 0), dist),
                (S, R(dist)) => (W, (0, -1), dist),
                (S, L(dist)) => (E, (0, 1), dist),
                (W, R(dist)) => (N, (1, 0), dist),
                (W, L(dist)) => (S, (-1, 0), dist),
            };
            for _ in 0..dist {
                if n != 0 {
                    pos = pos.north(n);
                } else if e != 0 {
                    pos = pos.east(e);
                }
                if !set.insert(pos.clone()) {
                    return ControlFlow::Break((dir, pos, set));
                }
            }
            ControlFlow::Continue((dir, pos, set))
        },
    );

    match res {
        ControlFlow::Break((_, res, _)) | ControlFlow::Continue((_, res, _)) => {
            let part2 = res.north.abs() + res.east.abs();
            println!("part2: {part2}");
        }
    }
}
