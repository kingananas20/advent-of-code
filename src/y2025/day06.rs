use itertools::izip;

pub fn day06(input: String, part2: bool) {
    let mut problems: Vec<Vec<u64>> = Vec::new();
    let mut ops = Vec::new();
    let numbers = input.lines().count();

    if !part2 {
        for (idx, line) in input.lines().enumerate() {
            if idx == numbers - 1 {
                for op in line.split_whitespace() {
                    ops.push(op);
                }
                break;
            }

            for (col, num) in line.split_whitespace().enumerate() {
                let num: u64 = num.parse().unwrap();

                if problems.len() <= col {
                    problems.push(Vec::new());
                }

                problems[col].push(num);
            }
        }
    } else {
        let mut lines: Vec<&str> = input.lines().collect();
        lines
            .pop()
            .unwrap()
            .split_whitespace()
            .for_each(|op| ops.push(op));

        let mut col = 0;

        for (c1, c2, c3, c4) in izip!(
            lines[0].chars(),
            lines[1].chars(),
            lines[2].chars(),
            lines[3].chars()
        ) {
            if c1 == ' ' && c2 == ' ' && c3 == ' ' && c4 == ' ' {
                col += 1;
                continue;
            }

            let num = format!("{c1}{c2}{c3}{c4}").trim().parse::<u64>().unwrap();

            if problems.len() <= col {
                problems.push(Vec::new());
            }

            problems[col].push(num);
        }
    }

    let results: Vec<u64> = problems
        .iter()
        .zip(ops)
        .map(|(nums, op)| match op {
            "+" => nums.iter().sum(),
            "*" => nums.iter().product(),
            _ => unreachable!(),
        })
        .collect();

    let grand_total: u64 = results.iter().sum();

    println!("The grand total is: {grand_total}");
}
