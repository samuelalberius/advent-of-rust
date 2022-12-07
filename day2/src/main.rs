fn main() {
    task_a();
    task_b();
}

fn task_a() {
    let s = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .fold(0, |sc, (p1, p2)| {
            match (p1, p2) {
                ("A", "X") => sc + 3 + 0,
                ("A", "Y") => sc + 1 + 3,
                ("A", "Z") => sc + 2 + 6,
                ("B", "X") => sc + 1 + 0,
                ("B", "Y") => sc + 2 + 3,
                ("B", "Z") => sc + 3 + 6,
                ("C", "X") => sc + 2 + 0,
                ("C", "Y") => sc + 3 + 3,
                ("C", "Z") => sc + 1 + 6,
                _ => 0,
            }
        });

    println!("{}", s);
}

fn task_b() {
    
}
