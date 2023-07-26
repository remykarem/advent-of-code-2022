fn main() {
    let raw_string = include_str!("puzzle.txt");

    let mut calories: Vec<u32> = raw_string
        // Split into the different elves
        .split("\n\n")
        // Get the calorie for each elf
        .map(|elf_calories_str| {
            elf_calories_str
                .split('\n')
                .map(str::parse)
                .map(|num| num.expect("Not a number"))
                .reduce(|acc, e| acc + e)
                .unwrap()
        })
        .collect();

    calories.sort();

    let qn1 = calories.last().expect("No value");
    let qn2: u32 = calories.iter().rev().take(3).sum();

    println!("{}", qn1);
    println!("{}", qn2);
}
