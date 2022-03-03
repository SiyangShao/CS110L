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
// enable unused

extern crate rand;
use rand::Rng;
use std::collections::HashMap;
use std::fs;
// use std::io;
// use std::io::{BufRead, Write};

const NUM_INCORRECT_GUESSES: usize = 5;
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
    // let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    println!("Input Guesses!");
    let mut guess: String = String::new();
    let mut guess_chance = NUM_INCORRECT_GUESSES;
    let mut mp = HashMap::new();
    loop {
        println!("Guess a letter. You have {} guesses left.", guess_chance);
        guess.clear();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: String = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let mut guess_len = 0;
        for i in guess.chars() {
            if i <= 'z' && i >= 'a' {
                guess_len += 1;
            }
        }
        println!("{}", guess_len);
        if guess_len > guess_chance {
            println!("Your guessing character is more then your chance. Please input again!");
            continue;
        }
        guess_chance -= guess_len;
        let mut right = 0;
        for i in guess.chars() {
            mp.insert(i, true);
            if secret_word.contains(i) {
                right += 1;
            }
        }
        guess_chance += right;
        println!(
            "You have guessed {} chars, now you have {} chances left.",
            guess_len, guess_chance
        );
        print!("The world is so far: ");
        let mut remain = secret_word.len();
        for i in secret_word.chars() {
            if mp.contains_key(&i) {
                remain = remain - 1;
                print!("{}", i);
            } else {
                print!("_");
            }
        }
        print!("\n");
        println!("You have {} worlds left.", remain);
        if remain == 0 {
            println!("You win!");
            break;
        }
        match guess_chance {
            0 => {
                println!("You lose!");
                println!("The word is {}", secret_word);
                break;
            }
            _ => continue,
        }
    }
}
