use rand::seq::IteratorRandom;
use std::{
    fs::File,
    io::{prelude::*, BufReader, Write},
    path::Path,
};

fn read_words(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .filter(|word| word.len() >= 5 && word.len() <= 10)
        .collect()
}

fn choose_word(words: &Vec<String>) -> &String {
    words.iter().choose(&mut rand::thread_rng()).unwrap()
}

fn print_status(lives: i32, known_letters: &Vec<String>, guessable_letters: &Vec<String>) {
    println!("\nLives: {}", lives);
    println!(
        "Word ({} letters): {}",
        known_letters.len(),
        known_letters.join("")
    );
    println!("Guessable letters: {}", guessable_letters.join(", "));
}

fn get_guess() -> String {
    print!("Enter guess: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let char_option = input.trim().chars().nth(0);
    match char_option {
        Some(c) => c.to_string(),
        None => get_guess(),
    }
}

fn main() {
    let words = read_words("src/words.txt");

    let word = choose_word(&words);
    let word_letters: Vec<String> = word.chars().map(|c| c.to_string()).collect();
    let mut known_letters: Vec<String> = vec![];
    for _ in word.chars() {
        known_letters.push("_".to_owned());
    }

    let mut guessable_letters: Vec<String> = ('a'..='z')
        .into_iter()
        .collect::<Vec<char>>()
        .iter()
        .map(|c| c.to_string())
        .collect();
    let mut guessed_letters: Vec<String> = vec![];
    let mut lives = 5;

    loop {
        print_status(lives, &known_letters, &guessable_letters);
        let guess = get_guess();

        if guessable_letters.contains(&guess) {
            guessed_letters.push(guess.to_string());
            guessable_letters.retain(|l| l != &guess);

            let mut found = false;
            for (i, letter) in word_letters.iter().enumerate() {
                if letter == &guess {
                    known_letters[i] = guess.to_string();
                    found = true;
                }
            }
            if found {
                println!("You discovered {}", guess);
            } else {
                lives -= 1;
                println!("{} is not in the word", guess);
            }
        } else if guessed_letters.contains(&guess) {
            println!("You already guessed that letter!");
        }

        if lives <= 0 {
            println!("\n\nYou lost!");
            break;
        } else {
            if !known_letters.contains(&"_".to_string()) {
                println!("\n\nYou won!");
                break;
            }
        }
    }
    println!("The word was {}\n", word);
}
