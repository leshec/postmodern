static WORDS: &str = include_str!("../assets/words.txt");

pub fn get_words() -> String {
    WORDS.to_string()
}
