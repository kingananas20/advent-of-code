pub fn day01(input: String, _part2: bool) {
    let mut floor = 0;
    let mut position = 1;
    let mut has_been_basement = false;

    input.chars().for_each(|c| {
        if c == '(' {
            floor += 1
        } else if c == ')' {
            floor -= 1
        }

        if floor < 0 && !has_been_basement {
            has_been_basement = true;
        } else if !has_been_basement {
            position += 1;
        }
    });

    println!(
        "Floor number: {floor}\nPosition where Santa enters the basement for the first time: {position}"
    );
}
