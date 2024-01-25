mod game;

use game::Action;
use game::Puzzle;

fn get_user_action(invert: bool) -> Action {
    use std::io;

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read STDIN!");

    input = input.trim().to_lowercase();

    let action = match input.as_str() {
        "up" | "u" => Action::Up,
        "down" | "d" => Action::Down,
        "right" | "r" => Action::Right,
        "left" | "l" => Action::Left,
        _ => Action::None,
    };

    if invert {
        match action {
            Action::Down => Action::Up,
            Action::Up => Action::Down,
            Action::Left => Action::Right,
            Action::Right => Action::Left,
            Action::None => Action::None,
        }
    } else {
        action
    }
}

fn print_puzzle(puzzle: &game::Puzzle) {
    println!("{puzzle}\n");
}

use clap::Parser;
#[derive(Parser, Debug)]
struct Args {
    /// Invert the controls
    #[arg(short, long, default_value_t = false)]
    invert: bool,

    /// Width of Puzzle (NxN)
    #[arg(short, long, default_value_t = 4)]
    size: usize,
}

fn main() {
    let args: Args = Args::parse();

    let mut puzzle = Puzzle::new_random(args.size);

    loop {
        print_puzzle(&puzzle);

        if puzzle.is_win() {
            println!("You won!");
            break;
        }

        let action = get_user_action(args.invert);
        puzzle.perform(action);
    }
}
