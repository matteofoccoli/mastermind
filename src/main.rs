mod mastermind;

use std::io;

fn main() {
    let max_guesses = 2;

    println!("Starting a new game");
    
    let mut game = mastermind::Mastermind::new(generate_secret(), max_guesses)
        .expect("Error initializing game");

    for _ in 0..max_guesses {
        println!("> Make your guess");
        let guess = read_string_and_convert_to_array_of_numbers();
        match game.guess(guess) {
            Ok(hint) => handle_hint(hint),
            _ => println!("> Generic error while playing")
        };
        if game.is_resolved() {
            break;
        }
    }
    if !game.is_resolved() {
        println!("> You have no more remaining guesses :(");
    }
    println!("> Secret was: {:?}", game.get_secret());
}

fn handle_hint(hint: mastermind::hint::Hint) {
    if hint.in_right_position == 4 {
        println!("> You won!")
    } else {
        println!("> You have {} numbers in right position and {} in wrong position", 
            hint.in_right_position,
            hint.in_wrong_position,
        );
    }
}

fn read_string_and_convert_to_array_of_numbers() -> [u32; 4] {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).expect("Error reading input");
    let numbers: Vec<u32> = input.trim().chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut secret: [u32; 4] = [0; 4];
    for i in 0..4 {
        secret[i] = numbers[i];
    }
    secret
}

fn generate_secret() -> [u32; 4] {
    [1,2,3,4]
}
