use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
};

use itertools::Itertools;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone)]
struct Circuit(HashSet<JunctionBox>);

impl Deref for Circuit {
    type Target = HashSet<JunctionBox>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Circuit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn day08(input: String, part2: bool) {
    let junction_boxes = input
        .lines()
        .map(|line| {
            let coords = line
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<_>>();
            JunctionBox {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect::<Vec<_>>();

    if part2 {
        let result = find_last_connection(&junction_boxes);
        println!("Last connection: {result}");
    } else {
        let closest_pairs = closest_pairs(&junction_boxes, 1000);

        let circuits = closest_pairs
            .iter()
            .map(|&((b1, b2), _)| {
                let mut s = HashSet::new();
                s.insert(b1);
                s.insert(b2);
                Circuit(s)
            })
            .collect::<Vec<_>>();

        let mut circuits = merge(circuits);

        circuits.sort_by_key(|circuit| circuit.len());
        circuits.reverse();

        let mut result = 1;
        circuits[..3]
            .iter()
            .for_each(|circuit| result *= circuit.len());

        println!("The 3 largest circuits multiplied together: {result}");
    }
}

fn closest_pairs(
    junction_boxes: &[JunctionBox],
    n: usize,
) -> Vec<((JunctionBox, JunctionBox), i64)> {
    let mut pairs = junction_boxes
        .iter()
        .tuple_combinations()
        .map(|(&a, &b)| ((a, b), distance_squared(&a, &b)))
        .collect::<Vec<_>>();

    pairs.sort_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());

    pairs.into_iter().take(n).collect()
}

fn distance_squared(a: &JunctionBox, b: &JunctionBox) -> i64 {
    (a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)
}

fn merge(mut circuits: Vec<Circuit>) -> Vec<Circuit> {
    let mut changed = true;

    while changed {
        changed = false;
        let mut i = 0;

        while i < circuits.len() {
            let mut j = i + 1;
            let mut merged = false;

            while j < circuits.len() {
                if circuits[i].intersection(&circuits[j]).count() > 0 {
                    let circuit_j = circuits.remove(j);
                    circuits[i].extend(circuit_j.iter().copied());
                    changed = true;
                    merged = true;
                } else {
                    j += 1;
                }
            }

            if !merged {
                i += 1;
            }
        }
    }

    circuits
}

/// Part 2
fn find_last_connection(junction_boxes: &[JunctionBox]) -> i64 {
    let mut all_pairs = junction_boxes
        .iter()
        .tuple_combinations()
        .map(|(&a, &b)| ((a, b), distance_squared(&a, &b)))
        .collect::<Vec<_>>();
    all_pairs.sort_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap());

    let mut circuits: Vec<Circuit> = junction_boxes
        .iter()
        .map(|&jb| {
            let mut s = HashSet::new();
            s.insert(jb);
            Circuit(s)
        })
        .collect();

    for ((b1, b2), _) in all_pairs {
        let mut idx1 = None;
        let mut idx2 = None;

        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&b1) {
                idx1 = Some(i);
            }
            if circuit.contains(&b2) {
                idx2 = Some(i);
            }
        }

        let idx1 = idx1.unwrap();
        let idx2 = idx2.unwrap();

        if idx1 != idx2 {
            let (remove_idx, keep_idx) = if idx1 > idx2 {
                (idx1, idx2)
            } else {
                (idx2, idx1)
            };

            let removed_circuit = circuits.remove(remove_idx);
            circuits[keep_idx].extend(removed_circuit.iter().copied());

            if circuits.len() == 1 {
                return b1.x * b2.x;
            }
        }
    }

    panic!("Could not connect all junction boxes into a single circuit");
}
