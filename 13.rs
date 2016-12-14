use std::collections::BTreeSet;
use std::collections::VecDeque;

fn is_open(x: i32, y: i32, magic: i32) -> bool {
    let v = x*x + 3*x + 2*x*y + y + y*y + magic;
    v.count_ones() % 2 == 0
}

fn main() {
    let mut front: VecDeque<(i32, i32, u32)> = VecDeque::new();
    let mut locations: BTreeSet<(i32, i32)> = BTreeSet::new();

    let magic = 1362;

    front.push_back((1, 1, 0));
    locations.insert((1, 1));

    let (target, mut under_50) = ((31, 39), 0);

    while let Some((x, y, len)) = front.pop_front() {
        if len <= 50 {
            under_50 += 1;
        }

        if (x, y) == target {
            println!("Target found! Steps: {}", len);
            break;
        }

        for &(xd, yd) in [(-1, 0), (0, 1), (1, 0), (0, -1)].iter() {
            let (x, y) = (x + xd, y + yd);

            if x < 0 || y < 0 { continue; }

            if locations.contains(&(x, y)) {
                continue;
            }

            if is_open(x, y, magic) {
                front.push_back((x, y, len + 1));
                locations.insert((x, y));
            }
        }
    }

    println!("Location reachable in 50 steps: {}", under_50);
}
