enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 6;

fn main() {
    let instructions = include_str!("../day_08_input.txt")
        .lines()
        .map(|line| {
            if let Some(rect) = line.strip_prefix("rect ") {
                let (x, y) = rect.split_once('x').unwrap();
                Instruction::Rect(x.parse().unwrap(), y.parse().unwrap())
            } else if let Some(row) = line.strip_prefix("rotate row y=") {
                let (x, y) = row.split_once(" by ").unwrap();
                Instruction::RotateRow(x.parse().unwrap(), y.parse().unwrap())
            } else if let Some(col) = line.strip_prefix("rotate column x=") {
                let (x, y) = col.split_once(" by ").unwrap();
                Instruction::RotateCol(x.parse().unwrap(), y.parse().unwrap())
            } else {
                panic!("invalid instruction");
            }
        })
        .collect::<Vec<_>>();

    let mut screen = [[0u8; SCREEN_WIDTH]; SCREEN_HEIGHT];

    for instruction in &instructions {
        match *instruction {
            Instruction::Rect(a, b) => screen.iter_mut().take(b).for_each(|row| {
                row.iter_mut().take(a).for_each(|pix| {
                    *pix = 1;
                })
            }),
            Instruction::RotateRow(a, b) => {
                screen[a].rotate_right(b % SCREEN_WIDTH);
            }
            Instruction::RotateCol(a, b) => {
                let mut col = [
                    screen[0][a],
                    screen[1][a],
                    screen[2][a],
                    screen[3][a],
                    screen[4][a],
                    screen[5][a],
                ];
                col.rotate_right(b % SCREEN_HEIGHT);
                screen[0][a] = col[0];
                screen[1][a] = col[1];
                screen[2][a] = col[2];
                screen[3][a] = col[3];
                screen[4][a] = col[4];
                screen[5][a] = col[5];
            }
        }
    }

    let part1 = screen
        .iter()
        .map(|row| row.iter().filter(|&&pix| pix == 1).count())
        .sum::<usize>();
    println!("part1: {part1}");

    screen.iter().for_each(|row| {
        row.iter().for_each(|&pix| {
            if pix == 1 {
                print!("#");
            } else {
                print!(".");
            }
        });
        println!();
    });
}
