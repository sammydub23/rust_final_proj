/// I wrote a Rust program to play hangman. The program prompts the user to enter a word for someone else (or yourself I guess) to guess.
/// The program puts the guessing player in a loop, prompting them to enter their guesses until they either get the word right or run out of guesses.
/// The program keeps track of the correct and incorrect guesses in their respective HashSet, the user is given 6 guesses and the program is case-sensitive.
 
use std::io;
use std::collections::HashSet;
 
fn main() {
    println!{"***========================================================================***"};
    println!("Welcome to Hangman! Please enter a word for the other player to guess:");
    println!{"***========================================================================***"};
 
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Failed to read line");
 
    let word = word.trim(); // remove leading and trailing whitespace
 
    let word_chars: HashSet<char> = word.chars().collect();
    let mut correct_guesses: HashSet<char> = HashSet::new();
    let mut incorrect_guesses: HashSet<char> = HashSet::new();
 
    let mut game_over = false;
    while !game_over {
        println!("Current word: {}", get_display_word(word, &correct_guesses));
        println!("Incorrect guesses: {:?}", incorrect_guesses);
 
        println!("Please enter your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: char = guess.trim().parse().expect("Please enter a valid character");
 
        if word_chars.contains(&guess) {
            correct_guesses.insert(guess);
        } else {
            incorrect_guesses.insert(guess);
        }
 
        if correct_guesses.len() == word_chars.len() {
            println!("Holy shit you won! The word was {}", word);
            game_over = true;
 
        } else if incorrect_guesses.len() >= 6 {
            println!("You lose! The word was {}", word);
            game_over = true;
        }
    }
}
 
fn get_display_word(word: &str, correct_guesses: &HashSet<char>) -> String {
    word.chars()
        .map(|c| if correct_guesses.contains(&c) { c } else { '_' })
        .collect()
}

