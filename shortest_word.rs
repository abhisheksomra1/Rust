fn shortest_word(sentence: &str) -> Option<&str> {
    // Split the sentence into words
    let words: Vec<&str> = sentence.split_whitespace().collect();

    // Initialize variables to track the shortest word and its length
    let mut shortest = None;
    let mut shortest_len = usize::MAX;

    // Iterate over each word and update the shortest word if needed
    for word in words {
        let word_len = word.len();
        if word_len < shortest_len {
            shortest = Some(word);
            shortest_len = word_len;
        }
    }

    shortest
}

fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog";
    
    match shortest_word(sentence) {
        Some(word) => println!("Shortest word: {}", word),
        None => println!("No words found in the sentence"),
    }
}
