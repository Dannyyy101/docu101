use std::collections::HashMap;

pub(crate) fn count_word_occurrences(text: &str)-> HashMap<String, usize>{
    let mut occurrences:HashMap<String, usize> = HashMap::new();

    for word in text.split_whitespace() {
        let word = word.trim_matches(|c:char|!c.is_alphanumeric());
        *occurrences.entry(word.to_lowercase()).or_insert(0) += 1
    }
    occurrences
}