use std::io;
fn main() {
    println!("Convert Words to Pig Latin ....\n");

    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to get word");

    let word = word.trim().to_string();

    let new_word = word_to_pig_latin(&word);

    println!("{}", &new_word);
}

fn word_to_pig_latin(word: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    if word.starts_with(|v: char| vowels.contains(&v)) {
        let word = format!("{word}hay");
        word
    } else {
        let mut chars = word.chars();

        let new_word = match chars.next() {
            None => String::new(),
            Some(first) => {
                let mut rest_of_word: String = chars.collect();
                rest_of_word.push(first);
                rest_of_word
            }
        };

        let word = format!("{new_word}ay");
        word
    }
}
