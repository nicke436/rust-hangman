use std::io;

fn main() {

    let mut secret_word: String = String::new();
    let mut tried_characters: Vec<char> = vec![];
    let mut input: String = String::new();
    let mut guesses: i32 = 6;

    // Get secret word
    println!("Enter the secret word: ");
    handle_input(&mut secret_word);
    let mut hidden_word: String = String::from("_".repeat(secret_word.trim().len()));

    loop {
        input = String::new();
        println!("Take a guess: ");
        handle_input(&mut input);

        let input: char = input.trim().parse().expect("Failed to parse");

        
            if secret_word.contains(input) && !tried_characters.contains(&input) {
                println!("Correct guess!");
                replace_hidden_letters(input, &mut hidden_word, get_letter_indexes(&input, &secret_word));
                println!("You've got: {}", hidden_word);
            } else if tried_characters.contains(&input) {
                println!("You've already used this letter");
            } else {
                println!("Incorrect guess, try again!");
                guesses -= 1;
                println!("You have {} more fails left.", guesses);
            }

            if guesses == 0 {
                println!("You lost! The word was: {}", secret_word);
                break;
            }
        
        tried_characters.push(input);

        if !hidden_word.contains("_") {
            println!("You won! The word was: {secret_word}");
            break;
        }
    }
}

fn handle_input(input_string: &mut String) {
    io::stdin()
        .read_line(input_string)
        .expect("Failed to read line");
}

fn get_letter_indexes(letter: &char, word: &String) -> Vec<usize> {
    let mut indexes: Vec<usize> = vec![];
    for (idx, char) in word.chars().enumerate() {
        if char == *letter {
            indexes.push(idx);
        }
    }
    indexes
}

fn replace_hidden_letters(letter: char, word: &mut String, idxs: Vec<usize>) {
    for index in idxs {
        word.remove(index);
        word.insert(index, letter);
    }
}