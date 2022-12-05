fn checksum(data: &[bool]) -> Vec<bool> {
    data.chunks_exact(2).map(|c| {
        let &[a, b] = c else { panic!() };
        a == b
    }).collect()
}

fn solve(input: &str, target_disk_len: usize) -> Vec<bool> {
    let mut disk = input.chars().map(|b| b == '1').collect::<Vec<_>>();
    while disk.len() < target_disk_len {
        let initial_len = disk.len();
        disk.push(false);
        for i in (0..initial_len).rev() {
            disk.push(!disk[i]);
        }
    }

    let mut cs = checksum(&disk[..target_disk_len]);
    while cs.len() % 2 == 0 {
        cs = checksum(&cs);
    }
    cs
}

fn main() {
    let input = "01110110101001000";

    for (part, target_disk_len) in [(1, 272), (2, 35651584)] {
        let cs = solve(input, target_disk_len);
        print!("part{part}: ");
        for c in cs {
            print!("{}", match c {
                true => "1",
                false => "0",
            });
        }
        println!();
    }
}
