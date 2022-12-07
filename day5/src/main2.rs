fn main() {
    let (crates_str, commands_str) = include_str!("input.txt").split_once("\n\n").unwrap();
    let mut crates: Vec<Vec<char>> = vec![vec![]; 9];
    let mut commands: Vec<(usize, usize, usize)> = Vec::new();

    let crane_version = 9001;

    crates_str.lines().rev().for_each(|line| line.chars().enumerate().filter(|(_, char)| char.is_alphabetic()).for_each(|(i, char)| {crates[(i as f64 / 4.0).floor() as usize].push(char);}));

    commands_str.lines().for_each(|line| {let t: Vec<usize> = line.split_ascii_whitespace().filter_map(|s| s.parse::<usize>().ok()).collect();commands.push((t[0], t[1], t[2]));});

    commands.iter().for_each(|(m, f, t)| {
        if crane_version == 9000 {
            for _ in 1..=*m {
                let c = crates[*f - 1].pop().unwrap();crates[*t -1].push(c);
            }
        } else {
            let mut temp: Vec<char> = Vec::new();
            for _ in 1..=*m {let c = crates[*f - 1].pop().unwrap();
            temp.push(c);
                }
                temp.iter().rev().for_each(|c| crates[*t -1].push(*c));
            }
        });

    crates.iter().for_each(|v| println!("{:?}", v));

    crates
        .iter()
        .for_each(|v| print!("{}", v.last().unwrap()));
}


