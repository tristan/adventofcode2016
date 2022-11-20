mod md5;

fn main() {
    let door_id = b"cxdnnyjw".to_vec();

    let part1 = (0u32..)
        .filter_map(|i| {
            let mut door_id = door_id.clone();
            if i == 0 {
                door_id.push(b'0');
            } else {
                let mut i = i;
                while i > 0 {
                    door_id.insert(8, (i % 10) as u8 + b'0');
                    i /= 10;
                }
            }
            let md5 = md5::md5(door_id);
            if md5[0] == 0 && md5[1] == 0 && (md5[2] & 0xf0) == 0 {
                char::from_digit((md5[2] & 0xf) as u32, 16)
            } else {
                None
            }
        })
        .take(8)
        .collect::<String>();

    println!("part1: {part1}");

    let mut part2 = ['_'; 8];
    let mut i = 0;
    while part2.iter().filter(|&&c| c == '_').count() > 0 {
        let mut door_id = door_id.clone();
        if i == 0 {
            door_id.push(b'0');
        } else {
            let mut i = i;
            while i > 0 {
                door_id.insert(8, (i % 10) as u8 + b'0');
                i /= 10;
            }
        }
        let md5 = md5::md5(door_id);
        if md5[0] == 0 && md5[1] == 0 && (md5[2] & 0xf0) == 0 {
            let pos = md5[2] & 0xf;
            if pos < 8 && part2[pos as usize] == '_' {
                part2[pos as usize] = char::from_digit((md5[3] & 0xf0) as u32 >> 4, 16).unwrap();
                println!("{}", part2.into_iter().collect::<String>());
            }
        }
        i += 1;
    }
}
