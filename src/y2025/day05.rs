use itertools::Itertools;
use std::collections::HashSet;

pub fn day05(input: String, part2: bool) {
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<(u64, u64)> = ranges
        .lines()
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();
    ranges.sort_by_key(|r| r.0);

    // Merge ranges
    let ranges: Vec<(u64, u64)> = ranges
        .into_iter()
        .coalesce(|(s1, e1), (s2, e2)| {
            if e1 >= s2 {
                Ok((s1, e1.max(e2)))
            } else {
                Err(((s1, e1), (s2, e2)))
            }
        })
        .collect();

    let ids: Vec<u64> = ids.lines().map(|id| id.parse().unwrap()).collect();

    let mut fresh_ids = HashSet::new();
    let mut count_fresh_ids = 0u64;

    for (start, end) in ranges {
        if !part2 {
            for id in &ids {
                if start <= *id && *id <= end {
                    fresh_ids.insert(*id);
                }
            }
        } else {
            count_fresh_ids += end - start + 1;
        }
    }

    let n = if !part2 {
        fresh_ids.len() as u64
    } else {
        count_fresh_ids
    };

    println!("{n}");
}
