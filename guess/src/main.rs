use rand::Rng;
use std::{
    env,
    fmt,
    io,
    //ops::{self},
    str::FromStr,
};

#[derive(Debug)]
enum Difficulty {
    EASY = 10,
    MEDIUM = 8,
    HARD = 5,
    MAX = 3,
}

// For Debugging
impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

/// FromStr allows us to take a variable and essentialy make a pointer
/// of it. What this means is, we can tell the program we want easy
/// mode, and it points the game to the memory location with the difficulty
/// level we want.
impl FromStr for Difficulty {
    type Err = ();

    fn from_str(input: &str) -> Result<Difficulty, Self::Err> {
        match input {
            "EASY" => Ok(Difficulty::EASY),
            "MEDIUM" => Ok(Difficulty::MEDIUM),
            "HARD" => Ok(Difficulty::HARD),
            "MAX" => Ok(Difficulty::MAX),
            // We're trashing the error because the program will not run
            // without one of the  above options.
            _ => Err(()),
        }
    }
}

fn help() {
    println!("Usage ./guess [DIFFICULTY]\n");
    println!("Difficulty Options:\nEasy\nMedium\nHard\nMax");
}

/// Return the difficulty passed as arg
fn get_difficulty() -> Result<Difficulty, ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 2 {
        let game_mode = args[1].to_uppercase();
        if game_mode != "EASY" || game_mode != "MEDIUM" || game_mode != "HARD" || game_mode != "MAX"
        {
            help();
            std::process::exit(0);
        }
    }
    
    let mut dif: String;

    // remnent of the first attempt
    // let _guess: Guess;

    let variant = args[1].to_uppercase();
    dif = format!("{}", variant);
    let dif = Difficulty::from_str(&mut dif).unwrap();
    Ok(dif)
}

fn intro(diff: Difficulty) -> i8 {
    println!("Welcome to the guessing game!");
    let num_guesses: i8;
    match diff {
        Difficulty::EASY => num_guesses = 10,
        Difficulty::MEDIUM => num_guesses = 8,
        Difficulty::HARD => num_guesses = 5,
        Difficulty::MAX => num_guesses = 3,
    }
    println!("You have {} guesses! Good Luck!", num_guesses);
    return num_guesses;
}

fn generate_number() -> i32 {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    return secret_number;
}

fn game(mut num_guesses: i8) {
    let number = generate_number();
    while num_guesses != 0 {
        println!("Please take a guess.");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let guess: i32 = input
            .trim()
            .parse()
            .expect("wanted a number");

        if guess == number {
            println!("You win!");
        } else if guess >= number {
            println!("To high!");
            num_guesses -= 1;
            println!("{} left!", num_guesses);
        } else {
            println!("To low!");
            num_guesses -= 1;
            println!("{} left!", num_guesses);
        }
        if num_guesses == 0 {
            std::process::exit(0);
        }
    }
}

fn main() {
    let guesses = intro(get_difficulty().expect("Failed to read difficulty!"));
    while guesses > 0 {
        game(guesses);
    }
}

// region: failed attempts
// region: struct style with Index implementation

// This was the first implementation. I thought of an array of values,
// something (not even) remotely like a linked list. One address points to
// another. That's where the resemblence ends. The idea was to link the enum
// to the struct using ops::Index<Enum>. It worked pretty well until it came
// to parse it from a string to Enum.
// struct Guess {
//     easy:    Difficulty,
//     medium:  Difficulty,
//     hard:    Difficulty,
//     max:     Difficulty,
// }

// impl ops::Index<Difficulty> for Guess {
//     type Output = Difficulty;

//     fn index(&self, difficulty: Difficulty) -> &Self::Output {
//         match difficulty {
//             Difficulty::EASY     => &self.easy,
//             Difficulty::MEDIUM   => &self.medium,
//             Difficulty::HARD     => &self.hard,
//             Difficulty::MAX      => &self.max,
//         }
//     }
// }
// endregion: struct style with Index implementation
// endregion: failed attempts
