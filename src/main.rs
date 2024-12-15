mod challenge;
mod year2024;

use anyhow::Result;

const DEFAULT_YEAR: u16 = 2024;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args[1] == "--help" || args[1] == "-h" || args[1] == "help" {
        print_help(0);
    }

    if args.len() < 2 || args.len() > 3 {
        print_help(1);
    }

    let year = match args.len() {
        3 => args[2].parse::<u16>().unwrap_or_else(|_| print_help(1)),
        _ => DEFAULT_YEAR,
    };

    let day = args[1].parse::<u8>().unwrap_or_else(|_| print_help(1));

    run(year, day)
}

fn print_help(exit_code: i32) -> ! {
    println!(
        "Usage: advent_of_code [year] <day>\ndefault year = {}",
        DEFAULT_YEAR
    );
    std::process::exit(exit_code);
}

/// Run the challenge for the given day.
pub fn run(year: u16, day: u8) -> Result<()> {
    let finder = match year {
        2024 => year2024::find_challenge,
        _ => return Err(anyhow::anyhow!("Year {} is implemented", year)),
    };

    let challenge = finder(day)?;
    let base_path = format!("input/year{:04}/day{:02}", year, day);
    let example_path = base_path.clone() + ".example";

    // check if single example file exists
    if std::fs::exists(&example_path)? {
        let input = std::fs::read_to_string(example_path)?;
        println!("Example input:");
        let part1 = challenge.part1(&input)?;
        println!("Part 1: {}", part1);

        let part2_res = challenge.part2(&input);
        let part2 = match part2_res {
            Ok(part2) => part2,
            Err(e) => e.to_string(),
        };

        println!("Part 2: {}", part2);

        std::thread::sleep(std::time::Duration::from_secs(2));
    } else {
        let example1_path = base_path.clone() + "_1.example";
        let example2_path = base_path.clone() + "_2.example";

        if std::fs::exists(&example1_path)? && std::fs::exists(&example2_path)? {
            let input1 = std::fs::read_to_string(example1_path)?;
            println!("Example input 1:");
            let part1 = challenge.part1(&input1)?;
            println!("Part 1: {}", part1);

            let input2 = std::fs::read_to_string(example2_path)?;
            println!("Example input 2:");
            let part2 = challenge.part2(&input2)?;
            println!("Part 2: {}", part2);

            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }

    if std::fs::exists(&base_path)? {
        let input = std::fs::read_to_string(base_path)?;
        let part1 = challenge.part1(&input)?;
        println!("Part 1: {}", part1);

        let part2 = challenge.part2(&input)?;
        println!("Part 2: {}", part2);

        return Ok(());
    } else {
        let input1 = std::fs::read_to_string(base_path.clone() + "_1")?;
        let part1 = challenge.part1(&input1)?;
        println!("Part 1: {}", part1);

        let input2 = std::fs::read_to_string(base_path.clone() + "_2")?;
        let part2 = challenge.part2(&input2)?;
        println!("Part 2: {}", part2);

        return Ok(());
    }
}
