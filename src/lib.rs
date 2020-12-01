mod days;

pub fn run(day: usize) {
    match day {
        1 => {
            days::day1::part1::run();
            days::day1::part2::run()
        }
        2 => {
            days::day2::part1::run();
            days::day2::part2::run();
        }
        _ => {
            println!("Day not valid");
            return
        }
    }
}

pub fn run_with_input(day : usize, input : &str) {
    match day {
        1 => {
            days::day1::part1::expense_rapport_str(input);
            days::day1::part2::expense_rapport_str(input);
        }
        _ => {
            println!("Day not valid");
            return
        }
    }
}