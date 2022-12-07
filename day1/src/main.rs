pub fn task_a(input: &str) -> i32 {
    let (mut tot, mut max) = (0, 0);
    for line in input.trim().split('\n') {
        match line.parse::<i32>() {
            Ok(c) => tot += c,
            _ => {
                max = std::cmp::max(tot, max);
                tot = 0;
            }
        }
    }
    max
}

pub fn task_b(input: &str) -> i32 {
    let (mut c_total, mut vec) = (0, vec![0; 3]);
    for line in input.trim().split('\n') {
        match line.parse::<i32>() {
            Ok(c) => c_total += c,
            _ => {
               vec.push(c_total);
               c_total = 0;
            }
        }
    }
    vec.sort();
    let total: i32 = vec[vec.len() - 3 .. vec.len()].iter().sum();
    total
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Task 1: {}", task_a(input));
    println!("Task 2: {}", task_b(input));
}