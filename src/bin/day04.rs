use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day04.txt").unwrap();
    let mut storage: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        let line = line.chars().map(|c| if c == '@' { 1 } else { 0 }).collect();
        storage.push(line);
    }
    let lines = storage.len();
    let line_length = storage.first().unwrap().len();

    let mut paper_rolls = 0;
    let mut prev_rolls = 0;

    loop {
        for idx in 0..lines {
            let mut top = vec![0u8; line_length];
            let mut bottom = vec![0u8; line_length];

            if idx != 0 {
                top = storage[idx - 1].clone();
            }

            if idx != lines - 1 {
                bottom = storage[idx + 1].clone();
            }

            for i in 0..line_length {
                if storage[idx][i] == 0 {
                    continue;
                }
                let start = i.saturating_sub(1);
                let end = (i + 1).min(line_length - 1);
                let mut adj: u32 = 0;
                adj += top[start..=end].iter().filter(|&&x| x == 1).count() as u32;
                adj += storage[idx][start..=end]
                    .iter()
                    .filter(|&&x| x == 1)
                    .count() as u32
                    - 1;
                adj += bottom[start..=end].iter().filter(|&&x| x == 1).count() as u32;
                if adj < 4 {
                    paper_rolls += 1u32;
                    storage[idx][i] = 0;
                };
            }
        }

        if paper_rolls - prev_rolls == 0 {
            break;
        }
        prev_rolls = paper_rolls;
    }

    println!("Amount of paper rolls: {paper_rolls}");
}
