const RESERVED_WORDS: [&str; 51] = [
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

/// Convert a Rust language reserved word to a safe word for attributes and variants
/// This is used when generating object attributes and enum variants.  If the provided work
/// is reserved, it is prepended with `_`.
pub fn reserved_word(word: &str) -> (String, bool) {
    match RESERVED_WORDS.contains(&word) {
        true => (format!("_{}", word), true),
        false => (word.to_string(), false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert_eq!(reserved_word("blah"), (String::from("blah"), false));
        assert_eq!(reserved_word("type"), (String::from("_type"), true));
    }
}
