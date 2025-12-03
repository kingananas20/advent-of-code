use std::fs;

fn main() {
    let mut input = fs::read_to_string("inputs/day02.txt").unwrap();
    input = input.strip_suffix("\n").unwrap().to_string();
    let ranges = input.split(",");

    let mut inv_ids: u64 = 0;

    for range in ranges {
        let (lower, upper): (u64, u64) = {
            let mut parts = range.split("-");
            let lower = parts.next().unwrap().parse().unwrap();
            let upper = parts.next().unwrap().parse().unwrap();
            (lower, upper)
        };

        for i in lower..upper {
            if is_invalid_id(i) {
                inv_ids += i;
            }
        }
    }

    println!("Invalid ids added up: {inv_ids}");
}

fn is_invalid_id(n: u64) -> bool {
    if n < 10 {
        return false;
    }

    let mut len = 0;
    let mut temp = n;
    while temp > 0 {
        temp /= 10;
        len += 1;
    }

    let mut pow10 = vec![1u64; len + 1];
    for i in 1..=len {
        pow10[i] = pow10[i - 1] * 10;
    }

    for pattern_len in 1..=len / 2 {
        if len % pattern_len != 0 {
            continue;
        }

        let repeats = len / pattern_len;

        let pattern = n / pow10[len - pattern_len];

        let mut rebuilt = 0u64;
        for _ in 0..repeats {
            rebuilt = rebuilt * pow10[pattern_len] + pattern;
        }

        if rebuilt == n {
            return true;
        }
    }

    false
}
