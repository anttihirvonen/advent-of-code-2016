fn fill_disk(len: usize, disk_contents: &mut Vec<u8>) {
    while disk_contents.len() < len {
        let mut flipped = disk_contents.iter().rev()
            .map(|c| if *c == 0 { 1 } else { 0 })
            .collect::<Vec<u8>>();

        disk_contents.push(0);
        disk_contents.append(&mut flipped);
    }

    disk_contents.truncate(len);
}

fn checksum(disk_contents: &Vec<u8>) -> String {
    let mut checksum = disk_contents.clone();

    while checksum.len() % 2 == 0 || checksum.len() == disk_contents.len() {
        let half_len = checksum.len() / 2;

        for i in 0..half_len {
            checksum[i] = if checksum[i * 2] == checksum[i * 2 + 1] { 1 } else { 0 };
        }

        checksum.truncate(half_len);
    }

    checksum.iter().map(|b| if *b == 0 { '0' } else { '1' }).collect::<String>()
}

fn find_checksum(disk_len: usize, initial_contents: &str) -> String{
    let mut initial_state = String::from(initial_contents).chars()
        .map(|c| if c == '0' { 0 } else { 1 })
        .collect::<Vec<u8>>();
    fill_disk(disk_len, &mut initial_state);
    checksum(&initial_state)
}

fn main() {
    println!("A: {}", find_checksum(273, "11011110011011101"));
    println!("B: {}", find_checksum(35651584, "11011110011011101"));
}
