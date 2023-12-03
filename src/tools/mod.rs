use std::fs::read_to_string;

pub fn vec_lines(file_path: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(file_path).unwrap().lines() {
        if !line.is_empty() {
            result.push(line.to_string())
        }
    }

    result
}
