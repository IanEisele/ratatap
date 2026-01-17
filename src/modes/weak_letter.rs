use rand::seq::SliceRandom;

use crate::stats::ProgressData;

const WORDS: &[&str] = &[
    "the", "be", "to", "of", "and", "a", "in", "that", "have", "I", "it", "for", "not", "on",
    "with", "he", "as", "you", "do", "at", "this", "but", "his", "by", "from", "they", "we",
    "say", "her", "she", "or", "an", "will", "my", "one", "all", "would", "there", "their",
    "what", "so", "up", "out", "if", "about", "who", "get", "which", "go", "me", "when", "make",
    "can", "like", "time", "no", "just", "him", "know", "take", "people", "into", "year", "your",
    "good", "some", "could", "them", "see", "other", "than", "then", "now", "look", "only",
    "come", "its", "over", "think", "also", "back", "after", "use", "two", "how", "our", "work",
    "first", "well", "way", "even", "new", "want", "because", "any", "these", "give", "day",
    "most", "us", "very", "where", "much", "through", "find", "tell", "still", "try", "kind",
    "hand", "picture", "again", "change", "off", "play", "spell", "air", "away", "animal",
    "house", "point", "page", "letter", "mother", "answer", "found", "study", "learn", "should",
    "world", "high", "every", "near", "add", "food", "between", "own", "below", "country",
    "plant", "last", "school", "father", "keep", "tree", "never", "start", "city", "earth",
    "eye", "light", "thought", "head", "under", "story", "saw", "left", "don't", "few", "while",
    "along", "might", "close", "something", "seem", "next", "hard", "open", "example", "begin",
    "life", "always", "those", "both", "paper", "together", "got", "group", "often", "run",
];

pub fn generate_text(progress: &ProgressData, word_count: usize) -> String {
    let weak_chars = progress.get_weakest_chars(10);

    if weak_chars.is_empty() {
        let mut rng = rand::thread_rng();
        return (0..word_count)
            .map(|_| *WORDS.choose(&mut rng).unwrap())
            .collect::<Vec<_>>()
            .join(" ");
    }

    let weak_char_set: Vec<char> = weak_chars.iter().map(|(c, _)| *c).collect();

    let filtered_words: Vec<&str> = WORDS
        .iter()
        .filter(|word| {
            word.chars()
                .any(|c| weak_char_set.contains(&c.to_ascii_lowercase()))
        })
        .copied()
        .collect();

    let mut rng = rand::thread_rng();

    if filtered_words.is_empty() {
        (0..word_count)
            .map(|_| *WORDS.choose(&mut rng).unwrap())
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        (0..word_count)
            .map(|_| *filtered_words.choose(&mut rng).unwrap())
            .collect::<Vec<_>>()
            .join(" ")
    }
}
