use rand::seq::SliceRandom;

use crate::finger_map::{get_keys_for_finger, Finger};

pub fn generate_text(finger: Finger, word_count: usize) -> String {
    let keys = get_keys_for_finger(finger);
    let mut rng = rand::thread_rng();

    let patterns = generate_patterns(&keys);

    let mut result = Vec::new();
    for _ in 0..word_count {
        if let Some(pattern) = patterns.choose(&mut rng) {
            result.push(pattern.clone());
        }
    }

    result.join(" ")
}

fn generate_patterns(keys: &[char]) -> Vec<String> {
    let mut patterns = Vec::new();

    for i in 0..keys.len() {
        for j in 0..keys.len() {
            if keys[i].is_alphabetic() && keys[j].is_alphabetic() {
                patterns.push(format!("{}{}", keys[i], keys[j]).repeat(2));
            }
        }
    }

    for key in keys {
        if key.is_alphabetic() {
            patterns.push(key.to_string().repeat(3));
        }
    }

    for i in 0..keys.len() {
        for j in 0..keys.len() {
            for k in 0..keys.len() {
                if keys[i].is_alphabetic() && keys[j].is_alphabetic() && keys[k].is_alphabetic() {
                    patterns.push(format!("{}{}{}", keys[i], keys[j], keys[k]));
                }
            }
        }
    }

    patterns
}
