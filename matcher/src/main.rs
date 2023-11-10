use std::fs::read_to_string;

fn read_lines(filename: &str) -> Result<Vec<String>, String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).map_err(|e| e.to_string())?.lines() {
        result.push(line.to_string())
    }

    Ok(result)
}

fn split_pattern(pattern: &str) -> Vec<String> {
    let r: Vec<String> = pattern.split('/').map(|s| s.to_string()).collect();
    r
}

fn match_object(target: &str, defs: &Vec<String>) -> bool {
    defs.contains(&String::from(target))
}


fn main() {
    let defs = read_lines("../oscal_defs.txt").expect("oops");
    let patterns = read_lines("../oscal_patterns.txt").expect("oops");

    let splits: Vec<Vec<String>> = patterns.iter().map(|s| split_pattern(s)).collect();
    for pattern in splits {
        for component in pattern {
            if component.starts_with("o:") && !match_object(&component, &defs) {
                println!("No match for {}", &component);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompose_pattern() {
        
    }
}
