fn find_elf_index(count: i32) -> i32 {
    let (mut c, mut d, mut i) = (count, 1, 0);

    while c > 1 {
        i -= if c % 2 != 0 { d } else { 0 };
        c = (c + 1) / 2;
        d *= 2;
    }

    count + i + 1
}

fn main() {
    println!("{}", find_elf_index(3014387));
}
