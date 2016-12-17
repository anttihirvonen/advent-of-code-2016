fn is_aligned(time: u32, offset: u32, disk_pos: u32, disk_size: u32) -> bool {
    return (time + offset + disk_pos) % disk_size == 0;
}

fn search(disks: &Vec<(u32, u32)>) -> u32 {
    for time in 0.. {
        if disks.iter().enumerate().map(|(p, d)| is_aligned(time, d.0, p as u32 + 1, d.1)).all(|a| a) {
            return time;
        }
    }

    0
}

fn main() {
    let mut disks: Vec<(u32, u32)> = vec![
        (0, 7),
        (0, 13),
        (2, 3),
        (2, 5),
        (0, 17),
        (7, 19)];

    println!("A: {}", search(&disks));
    disks.push((0, 11));
    println!("B: {}", search(&disks));

}
