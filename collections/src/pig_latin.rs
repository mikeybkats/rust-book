pub mod pig_latin {
    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
    fn is_vowel(c: &char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            'A' | 'E' | 'I' | 'O' | 'U' => true,
            _ => false,
        }
    }

    // expensive! uses vectors and looping to rearrange letters
    pub fn convert_word(word: &str) -> String {
        let vector_word: Vec<char> = word.chars().collect();

        let mut converted_word_suffix = String::new();
        let mut converted_word_prefix = String::new();

        // let mut starts_with_vowel = false;
        for (index, letter) in vector_word.iter().enumerate() {
            if index == 0 && is_vowel(letter) {
                return String::from(word) + "-hay";
            } else if index == 0 {
                converted_word_suffix.push(*letter);
                converted_word_suffix = "-".to_string() + &converted_word_suffix + "ay";
                continue;
            }

            // push letters to prefix
            converted_word_prefix.push(*letter);
        }

        return converted_word_prefix + &converted_word_suffix.to_string();
    }

    // uses built in Rust methods to test for vowels
    // uses template formatting to print words
    pub fn convert_word_gpt(word: &str) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let first_char = word.chars().next().unwrap_or('z');

        if vowels.contains(&first_char) {
            format!("{}-hay", word)
        } else {
            format!("{}-{}ay", &word[1..], first_char)
        }
    }
}
