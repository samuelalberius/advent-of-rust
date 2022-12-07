fn task_a(input: &str) {
    let mut sum = 0;
    for l in input.lines() {
        let ranges = l.split(",").collect::<Vec<&str>>();
        let (r1, r2) = ranges[0].split_once("-").unwrap();
        let (r3, r4) = ranges[1].split_once("-").unwrap();
        let r1: u32 = r1.parse().unwrap();
        let r2: u32 = r2.parse().unwrap();
        let r3: u32 = r3.parse().unwrap();
        let r4: u32 = r4.parse().unwrap();

        if (r1 <= r3 && r2 >= r3) || (r3 <= r1 && r4 >= r1) {
            sum += 1;
        }
    }
    println!("{}", sum);
}

fn main() {
    let input = include_str!("input.txt");
    task_a(input);
    // task_b(input);
}
