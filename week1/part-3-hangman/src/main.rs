// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::stdout;
use std::io::Write;
use std::iter::FromIterator;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)
    io::stdout().flush().expect("Error flushing stdout");
    let mut guess_word = Vec::new();
    let mut guess_bytes = String::new();
    let mut guess_chance = NUM_INCORRECT_GUESSES;
    let mut hash_set = HashSet::new();

    for _ in 0..secret_word.len() {
        guess_word.push('-');
    }

    let mut secret_word_clone = secret_word_chars.clone();
    secret_word_clone.sort();
    secret_word_clone.dedup();

    let guess_bytes_num = secret_word_clone.len();
    let mut right_bytes_num: usize = 0;

    while guess_chance > 0 {
        println!(
            "The world so far is {}",
            String::from_iter(guess_word.iter())
        );
        println!("You have guessed the following letters: {}", guess_bytes);
        println!("You have {} guesses left", guess_chance);
        print!("Please guess a letter: ");
        stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        println!();
        let guess_byte = input.chars().nth(0).expect("No byte read");

        if !hash_set.contains(&guess_byte) {
            hash_set.insert(guess_byte);
        } else {
            println!("You have guessed this letter!\n");
            continue;
        }

        let mut is_in_word = false;
        for (i, ch) in secret_word_chars.iter().enumerate() {
            if guess_byte == *ch {
                is_in_word = true;
                guess_word[i] = guess_byte;
            }
        }

        guess_bytes.push(guess_byte);

        if !is_in_word {
            guess_chance -= 1;
        } else {
            right_bytes_num += 1;
        }

        if right_bytes_num == guess_bytes_num {
            break;
        }
    }

    if guess_chance > 0 {
        println!(
            "Congratulations you guessed the secret word: {}!",
            secret_word
        );
    } else {
        println!("Sorry, you ran out of guesses!");
    }
}
