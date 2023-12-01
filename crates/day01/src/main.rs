fn main() {
    let input = include_str!("input.txt");
    let numbers = input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|ch| ch.is_numeric())
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let part01: u32 = numbers
        .iter()
        .map(|line| line.first().unwrap() * 10 + line.last().unwrap())
        .sum();

    println!("Part 01: {part01}");

    let nums_as_letters = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let numbers = input
        .lines()
        .map(|line| {
            let mut nums = Vec::new();
            for (i, ch) in line.char_indices() {
                if ch.is_numeric() {
                    nums.push(ch.to_digit(10).unwrap());
                    continue;
                }

                for (n, potential) in nums_as_letters.iter().enumerate() {
                    if line[i..].starts_with(potential) {
                        nums.push(n as u32);
                    }
                }
            }

            nums
        })
        .collect::<Vec<Vec<u32>>>();

    let part02: u32 = numbers
        .iter()
        .map(|line| line.first().unwrap() * 10 + line.last().unwrap())
        .sum();

    println!("Part 02: {part02}");
}
