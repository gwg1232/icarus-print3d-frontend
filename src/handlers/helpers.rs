use std::collections::HashMap;

pub fn parse_errors(input: &str) -> HashMap<String, String> {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(": ")
                .map(|(key, value)| (key.to_string(), value.to_string()))
        })
        .collect()
}
