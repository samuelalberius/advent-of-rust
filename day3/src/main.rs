fn task_a(input: &str) {
    let alfabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut sum = 0;
    for l in input.lines() {
        let (r1, r2) = l.split_at(l.len()/2);
        for c in r1.chars() {
            if r2.contains(c) {
                sum += alfabet.find(c).unwrap() + 1;
                break;
            }
        }
    }
    println!("{}", sum);
}

fn task_b(input: &str) {
    let alfabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut sum = 0;
    let mut ite = 0;
    for l in input.lines().collect::<Vec<&str>>().windows(3) {
        if ite % 3 == 0 {
            for c in l[0].chars() {
                if l[1].contains(c) && l[2].contains(c) {
                    sum += alfabet.find(c).unwrap() + 1;
                    break;
                }
            }
        }
        ite += 1;
    }
    println!("{}", sum);
}

fn main() {
    let input = include_str!("input.txt");
    task_a(input);
    task_b(input);
}
