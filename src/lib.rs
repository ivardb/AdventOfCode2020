mod days;

pub fn run(day: usize) {
    match day {
        1 => {
            days::day1::part1::run();
            days::day1::part2::run()
        }
        _ => {
            println!("Day not valid");
            return
        }
    }
}