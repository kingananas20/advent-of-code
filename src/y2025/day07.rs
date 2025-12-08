use std::collections::{HashSet, VecDeque};

pub fn day07(input: String, part2: bool) {
    let grid = input.lines().collect::<Vec<&str>>();

    let height = grid.len();
    let width = grid[0].len();

    let mut start = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        if let Some(x) = row.find("S") {
            start = (x, y);
            break;
        }
    }

    println!("Start: {start:?}");

    if !part2 {
        let mut splits = 0;
        let mut beams = VecDeque::new();
        let mut splitters = HashSet::new();
        beams.push_back((start.0, start.1));

        while let Some((x, mut y)) = beams.pop_front() {
            while y < height && grid[y].chars().nth(x).unwrap() != '^' {
                y += 1;
            }

            if y >= height {
                continue;
            }

            if !splitters.insert((x, y)) {
                continue;
            }

            splits += 1;

            if x > 0 {
                beams.push_back((x - 1, y));
            }
            if x < width - 1 {
                beams.push_back((x + 1, y));
            }
        }

        println!("The beam split {splits} times");
    } else {
        let mut lasers = vec![0_usize; width];
        lasers[start.0] = 1;

        let mut buffer = vec![0_usize; width];

        #[allow(clippy::needless_range_loop)]
        for y in (start.1 + 1)..height {
            buffer.fill(0);

            for x in 0..width {
                if lasers[x] == 0 {
                    continue;
                }

                let tile = grid[y].chars().nth(x).unwrap();
                match tile {
                    // Particle continues straight down
                    '.' => {
                        buffer[x] += lasers[x];
                    }
                    // Particle splits left and right
                    '^' => {
                        if x > 0 {
                            buffer[x - 1] += lasers[x];
                        }
                        if x < width - 1 {
                            buffer[x + 1] += lasers[x];
                        }
                    }
                    _ => {}
                }
            }

            std::mem::swap(&mut lasers, &mut buffer);
        }

        let total_timelines: usize = lasers.iter().sum();

        println!("There's {total_timelines} total timelines")
    }
}
