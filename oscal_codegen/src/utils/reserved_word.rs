pub fn reserved_word(word: &str) -> (String, bool) {
    match word {
        "type" | "enum" | "ref" => (format!("_{}", word), true),
        _ => (word.to_string(), false),
    }
}
